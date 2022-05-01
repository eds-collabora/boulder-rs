use super::helpers::get_persian_rug_constraints;
use proc_macro2 as pm2;

use crate::attributes::*;

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
    let mut change_type_body = pm2::TokenStream::new();
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
                change_type_body.extend(quote::quote! {
                    #fieldid: self.#fieldid,
                });
            }
        }
    }
    
    let bare_generics = {
        let params = &full_generics.params;
        quote::quote! {
            , #params
        }
    };

    let bare_ty_generics = {
        let mut res = pm2::TokenStream::new();
        for p in &full_generics.params {
            match p {
                syn::GenericParam::Type(syn::TypeParam { ident, .. }) => {
                    res.extend(quote::quote! {
                        , #ident
                    });
                },
                syn::GenericParam::Lifetime(syn::LifetimeDef { lifetime, .. }) => {
                    res.extend(quote::quote! {
                        , #lifetime
                    });
                },
                syn::GenericParam::Const(syn::ConstParam { const_token, ident, ..}) => {
                    res.extend(quote::quote! {
                        , #const_token #ident
                    });
                },
            }
        }
        res
    };
    
    let bare_wc = {
        let wc = &full_generics.where_clause.as_ref().map(|w| &w.predicates);

        quote::quote! {
            #wc
        }
    };
    
    let res = quote::quote! {
        const _: () = {
            #vis struct Builder<BoulderTypeMarkerParam #bare_generics> #wc {
                _boulder_created_marker: ::core::marker::PhantomData<BoulderTypeMarkerParam>,
                #body
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl<BoulderTypeMarkerParam #bare_generics> Builder<BoulderTypeMarkerParam #bare_ty_generics> #wc {
                pub fn new() -> Self {
                    Self {
                        _boulder_created_marker: Default::default(),
                        #default_body
                    }
                }

                fn change_type<BoulderFunctionTypeParam>(self) -> Builder<BoulderFunctionTypeParam #bare_ty_generics> {
                    Builder {
                        _boulder_created_marker: Default::default(),
                        #change_type_body
                    }
                }

                #methods
            }


            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl #generics ::boulder::persian_rug::builder::guts::BoulderBase for #ident #ty_generics #wc {
                type Base = #ident #ty_generics;
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl #generics ::boulder::persian_rug::builder::guts::MiniBuildableWithPersianRug<#ident #ty_generics, #context> for #ident #ty_generics #wc {
                type Builder = Builder<#ident #ty_generics #bare_ty_generics>;
                fn mini_builder() -> Self::Builder {
                    Builder::new()
                }
            }
            
            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl #generics ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context> for Builder<#ident #ty_generics #bare_ty_generics> #wc
            {
                type Result = #ident #ty_generics;
                fn build<'boulder_mutator_lifetime, BoulderMutatorParam>(self, mut context: BoulderMutatorParam) -> (Self::Result, BoulderMutatorParam)
                where
                    BoulderMutatorParam: 'boulder_mutator_lifetime + ::persian_rug::Mutator<Context=#context>
                {
                    #defaults

                    (
                        #ident {
                            #make_body
                        },
                        context
                    )
                }
            }

            // Option
            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::builder::guts::MiniBuildableWithPersianRug<#ident #ty_generics, #context> for Option<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::persian_rug::builder::guts::MiniBuildableWithPersianRug<#ident #ty_generics, #context>,
                Builder<BoulderExtraGenericParam #bare_ty_generics>: ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context, Result=BoulderExtraGenericParam>,
                #bare_wc
            {
                type Builder = Builder<Option<BoulderExtraGenericParam> #bare_ty_generics>;
                fn mini_builder() -> Self::Builder {
                    Builder::new()
                }
            }
            
            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context> for Builder<Option<BoulderExtraGenericParam> #bare_ty_generics>
            where
                Builder<BoulderExtraGenericParam #bare_ty_generics>: ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context, Result=BoulderExtraGenericParam>,
                #bare_wc
            {
                type Result = Option<BoulderExtraGenericParam>;
                fn build<'boulder_mutator_lifetime, BoulderMutatorParam>(self, mut context: BoulderMutatorParam) -> (Self::Result, BoulderMutatorParam)
                where
                    BoulderMutatorParam: 'boulder_mutator_lifetime + ::persian_rug::Mutator<Context=#context>
                {
                    let (result, context) = <Builder<BoulderExtraGenericParam #bare_ty_generics> as ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context>>::build(self.change_type(), context);
                    (Some(result), context)
                }
            }

            // Proxy 
            #[automatically_derived]
            #[persian_rug::constraints(#constraints, access(BoulderExtraGenericParam))]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::builder::guts::MiniBuildableWithPersianRug<#ident #ty_generics, #context> for ::persian_rug::Proxy<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::persian_rug::builder::guts::MiniBuildableWithPersianRug<#ident #ty_generics, #context>,
                Builder<BoulderExtraGenericParam #bare_ty_generics>: ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context, Result=BoulderExtraGenericParam>,
                #bare_wc
            {
                type Builder = Builder<::persian_rug::Proxy<BoulderExtraGenericParam> #bare_ty_generics>;
                fn mini_builder() -> Self::Builder {
                    Builder::new()
                }
            }
            
            #[automatically_derived]
            #[persian_rug::constraints(#constraints, access(BoulderExtraGenericParam))]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context> for Builder<::persian_rug::Proxy<BoulderExtraGenericParam> #bare_ty_generics>
            where
                Builder<BoulderExtraGenericParam #bare_ty_generics>: ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context, Result=BoulderExtraGenericParam>,
                #bare_wc
            {
                type Result = ::persian_rug::Proxy<BoulderExtraGenericParam>;
                fn build<'boulder_mutator_lifetime, BoulderMutatorParam>(self, mut context: BoulderMutatorParam) -> (Self::Result, BoulderMutatorParam)
                where
                    BoulderMutatorParam: 'boulder_mutator_lifetime + ::persian_rug::Mutator<Context=#context>
                {
                    let (result, mut context) = <Builder<BoulderExtraGenericParam #bare_ty_generics> as ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context>>::build(self.change_type(), context);
                    (context.add(result), context)
                }
            }
       };
    };

    //println!("{}", res);
    res
}
