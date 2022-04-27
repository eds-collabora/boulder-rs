use super::helpers::get_persian_rug_constraints;
use proc_macro2 as pm2;

use crate::attributes::*;

fn make_concrete_builder<'a>(
    generics: impl Iterator<Item = &'a syn::TypeParam>,
    converter: syn::Path,
) -> pm2::TokenStream {
    let mut body = pm2::TokenStream::new();

    for ty in generics {
        let id = &ty.ident;
        body.extend(quote::quote! {
            #id,
        });
    }
    body.extend(quote::quote! {
        #converter
    });

    quote::quote! {
        Builder<#body>
    }
}

pub fn derive_buildable_with_persian_rug(input: syn::DeriveInput) -> pm2::TokenStream {
    let syn::DeriveInput {
        attrs,
        ident,
        data,
        generics,
        vis,
        ..
    } = input;

    let mut body = pm2::TokenStream::new();
    let mut default_body = pm2::TokenStream::new();
    let mut methods = pm2::TokenStream::new();
    let mut make_body = pm2::TokenStream::new();
    let mut defaults = pm2::TokenStream::new();

    let (context, used_types) = get_persian_rug_constraints(&attrs);
    let mut constraints = pm2::TokenStream::new();
    constraints.extend(quote::quote! {
        context = #context,
    });
    let used_types: syn::punctuated::Punctuated<syn::Type, syn::Token![,]> =
        used_types.into_iter().collect();
    constraints.extend(quote::quote! {
        access(#used_types)
    });

    let full_generics = generics.clone();
    let (generics, ty_generics, wc) = generics.split_for_impl();
    
    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(syn::FieldsNamed { named, .. }) = s.fields {
            for field in named.iter() {
                let fieldid = field.ident.as_ref().unwrap();
                let fieldtype = &field.ty;
                let mut builder = BuildType::Default;
                let mut generator = GeneratorType::Default;
                let mut sequence = None;
                let mut builder_needs_context = false;
                let mut generator_needs_context = false;

                for attr in field.attrs.iter() {
                    if attr.path.is_ident("boulder") {
                        let parsed = attr
                            .parse_args::<BuilderMeta>()
                            .expect("failed to parse boulder attribute");
                        if let BuildType::Default = builder {
                            builder = parsed.builder.element;
                            builder_needs_context = parsed.builder.needs_context;
                        }
                        if let GeneratorType::Default = generator {
                            generator = parsed.generator.element;
                            generator_needs_context = parsed.generator.needs_context;
                        }
                        if sequence.is_none() {
                            sequence = parsed.builder.sequence;
                        }
                    }
                }

                if let Some(sequence) = sequence {
                    let mut gen_init = pm2::TokenStream::new();
                    match &generator {
                        GeneratorType::Generator(expr) => {
                            gen_init.extend(quote::quote! {
                                {
                                    #expr
                                }
                            });
                        }
                        GeneratorType::Generatable(map) => {
                            let mut gen_body = pm2::TokenStream::new();
                            for (k, v) in map {
                                gen_body.extend(quote::quote! {
                                    g.#k(#v);
                                });
                            }
                            gen_init.extend(quote::quote! {
                                {
                                    let mut g = <<#fieldtype as std::iter::IntoIterator>::Item as ::boulder::Generatable>::generator();
                                    #gen_body
                                    g
                                }
                            });
                        }
                        GeneratorType::Default => {
                            let mut static_value = pm2::TokenStream::new();
                            match builder {
                                BuildType::Buildable(map) => {
                                    if builder_needs_context {
                                        static_value.extend(quote::quote! {
                                            let builder = <<#fieldtype as std::iter::IntoIterator>::Item as ::boulder::BuildableWithPersianRug<#context>>::builder();
                                        });
                                        for (k, v) in map {
                                            static_value.extend(quote::quote! {
                                                builder.#k(#v);
                                            });
                                        }
                                        static_value.extend(quote::quote!{
                                            let (#fieldid, mut context) = <<<#fieldtype as std::iter::IntoIterator>::Item as ::boulder::BuildableWithPersianRug<#context>>::Builder as ::boulder::BuilderWithPersianRug<#context>>::build(builder, context),
                                        });
                                    } else {
                                        static_value.extend(quote::quote! {
                                            let builder = <<#fieldtype as std::iter::IntoIterator>::Item as ::boulder::Buildable<'_, _>>::builder();
                                        });
                                        for (k, v) in map {
                                            static_value.extend(quote::quote! {
                                                builder.#k(#v);
                                            });
                                        }
                                        static_value.extend(quote::quote!{
                                            let #fieldid = <<<#fieldtype as std::iter::IntoIterator>::Item as ::boulder::Buildable>::Builder as ::boulder::Builder>::build(builder);
                                        });
                                    }
                                    static_value.extend(quote::quote! {
                                        #fieldid
                                    });
                                }
                                BuildType::Value(value) => {
                                    static_value.extend(quote::quote! {
                                        (#value).into()
                                    });
                                }
                                BuildType::Default => {
                                    static_value.extend(quote::quote! {
                                        Default::default()
                                    });
                                }
                            };

                            gen_init.extend(quote::quote! {
                                |context: S| { #static_value }
                            });
                        }
                    }

                    let needs_context = match &generator {
                        GeneratorType::Default => builder_needs_context,
                        _ => generator_needs_context,
                    };

                    if needs_context {
                        defaults.extend(quote::quote! {
                            let (#fieldid, mut context) = if let Some(value) = self.#fieldid {
                                (self.#fieldid, context)
                            } else {
                                let mut gen = #gen_init;
                                let iter = ::boulder::GeneratorWithContextIterator::new(gen, context);
                                let value = iter.take(#sequence).collect();
                                (value, iter.into())
                            };
                        });
                    } else {
                        defaults.extend(quote::quote! {
                            let #fieldid = self.#fieldid.unwrap_or_else(|| {
                                let mut gen = #gen_init;
                                let iter = ::boulder::GeneratorIterator::new(gen);
                                let value = iter.take(#sequence).collect();
                                value
                            });
                        });
                    }
                } else {
                    match builder {
                        BuildType::Buildable(map) => {
                            if builder_needs_context {
                                let mut initializers = pm2::TokenStream::new();
                                initializers.extend(quote::quote! {
                                    <#fieldtype as ::boulder::BuildableWithPersianRug<#context>>::builder()
                                });
                                for (k, v) in map {
                                    initializers.extend(quote::quote! {
                                        .#k(#v)
                                    });
                                }
                                defaults.extend(quote::quote! {
                                    let (#fieldid, mut context) = if let Some(value) = self.#fieldid {
                                        (value, context)
                                    } else {
                                        <<#fieldtype as ::boulder::BuildableWithPersianRug<#context>>::Builder as ::boulder::BuilderWithPersianRug<#context>>::build(#initializers, context)

                                    };
                                });
                            } else {
                                let mut initializers = pm2::TokenStream::new();
                                initializers.extend(quote::quote! {
                                    let builder = <#fieldtype as ::boulder::Buildable>::builder();
                                });
                                for (k, v) in map {
                                    initializers.extend(quote::quote! {
                                        builder.#k(#v);
                                    });
                                }
                                initializers.extend(quote::quote! {
                                    <<#fieldtype as ::boulder::Buildable>::Builder as ::boulder::Builder>::build(builder);
                                });
                                defaults.extend(quote::quote! {
                                    let #fieldid = self.#fieldid.unwrap_or_else(|| {
                                        #initializers
                                    });
                                });
                            }
                        }
                        BuildType::Value(value) => defaults.extend(quote::quote! {
                            let #fieldid = self.#fieldid.unwrap_or_else(|| (#value).into());
                        }),
                        BuildType::Default => defaults.extend(quote::quote! {
                            let #fieldid = self.#fieldid.unwrap_or_default();
                        }),
                    };
                }

                body.extend(quote::quote! {
                    #fieldid: Option<#fieldtype>,
                });
                default_body.extend(quote::quote! {
                    #fieldid: ::std::default::Default::default(),
                });
                methods.extend(quote::quote! {
                    pub fn #fieldid<S>(mut self, value: S) -> Self
                    where
                        S: Into<#fieldtype>
                    {
                        self.#fieldid = Some(value.into());
                        self
                    }
                });
                make_body.extend(quote::quote! {
                    #fieldid,
                });
            }
        }
    }
    
    let builder_generics = {
        let mut g = full_generics.clone();

        g.params.push(syn::GenericParam::Type(
            syn::parse_quote! { BoulderConverterParam },
        ));

        let wc = g.make_where_clause();

        wc.predicates.push(syn::parse_quote! {
            BoulderConverterParam: ::boulder::persian_rug::ConverterWithPersianRug<#context, #ident #ty_generics>
        });

        g
    };

    let (builder_generics, builder_ty_generics, builder_wc) = builder_generics.split_for_impl();
    let self_builder_type = make_concrete_builder(
        full_generics.type_params(),
        syn::parse_quote! { ::boulder::persian_rug::SelfConverterWithPersianRug },
    );
    let proxy_builder_type = make_concrete_builder(
        full_generics.type_params(),
        syn::parse_quote! { ::boulder::persian_rug::ProxyConverterWithPersianRug },
    );
    let option_builder_type = make_concrete_builder(
        full_generics.type_params(),
        syn::parse_quote! { ::boulder::persian_rug::OptionConverterWithPersianRug },
    );

    let res = quote::quote! {
        const _: () = {
            #vis struct Builder #builder_generics #wc {
                converter: BoulderConverterParam,
                #body
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl #builder_generics Builder #builder_ty_generics #builder_wc {
                pub fn new(converter: BoulderConverterParam) -> Self {
                    Self {
                        converter,
                        #default_body
                    }
                }

                #methods
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl #builder_generics ::boulder::BuilderWithPersianRug<#context> for Builder #builder_ty_generics #builder_wc {
                type Result = BoulderConverterParam::Output;
                fn build<'b, B: 'b + ::persian_rug::Mutator<Context=#context>>(self, mut context: B) -> (Self::Result, B) {
                    #defaults

                    self.converter.convert(
                        #ident {
                            #make_body
                        },
                        context
                    )
                }
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl #generics ::boulder::persian_rug::BuildableWithPersianRug<#context> for #ident #ty_generics #wc {
                type Builder = #self_builder_type;
                fn builder() -> Self::Builder {
                    Builder::new(::boulder::persian_rug::SelfConverterWithPersianRug)
                }
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl #generics ::boulder::persian_rug::ProxyBuildableWithPersianRug<#context> for #ident #ty_generics #wc {
                type Builder = #proxy_builder_type;
                fn proxy_builder() -> Self::Builder {
                    Builder::new(::boulder::persian_rug::ProxyConverterWithPersianRug)
                }
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl #generics ::boulder::persian_rug::OptionBuildableWithPersianRug<#context> for #ident #ty_generics #wc {
                type Builder = #option_builder_type;
                fn option_builder() -> Self::Builder {
                    Builder::new(::boulder::persian_rug::OptionConverterWithPersianRug)
                }
            }
        };
    };

    //println!("{}", res);
    res
}
