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

pub fn derive_buildable(input: syn::DeriveInput) -> pm2::TokenStream {
    let syn::DeriveInput {
        ident,
        data,
        generics,
        vis,
        ..
    } = input;

    let full_generics = generics.clone();

    let (generics, ty_generics, wc) = full_generics.split_for_impl();

    let builder_generics = {
        let mut g = full_generics.clone();

        g.params.push(syn::GenericParam::Type(
            syn::parse_quote! { BoulderConverterParam },
        ));
        // g.params
        //     .push(syn::GenericParam::Type(syn::parse_quote! { BoulderResultParam }));

        let wc = g.make_where_clause();

        wc.predicates.push(syn::parse_quote! {
            BoulderConverterParam: ::boulder::builder::Converter<#ident #ty_generics>
        });

        g
    };

    let (builder_generics, builder_ty_generics, builder_wc) = builder_generics.split_for_impl();

    let self_builder_type = make_concrete_builder(
        full_generics.type_params(),
        syn::parse_quote! { ::boulder::builder::SelfConverter },
    );
    let option_builder_type = make_concrete_builder(
        full_generics.type_params(),
        syn::parse_quote! { ::boulder::builder::OptionConverter },
    );
    let rc_builder_type = make_concrete_builder(
        full_generics.type_params(),
        syn::parse_quote! { ::boulder::builder::RcConverter },
    );
    let arc_builder_type = make_concrete_builder(
        full_generics.type_params(),
        syn::parse_quote! { ::boulder::builder::ArcConverter },
    );
    let mutex_builder_type = make_concrete_builder(
        full_generics.type_params(),
        syn::parse_quote! { ::boulder::builder::MutexConverter },
    );
    let ref_cell_builder_type = make_concrete_builder(
        full_generics.type_params(),
        syn::parse_quote! { ::boulder::builder::RefCellConverter },
    );
    let cell_builder_type = make_concrete_builder(
        full_generics.type_params(),
        syn::parse_quote! { ::boulder::builder::CellConverter },
    );
    let box_builder_type = make_concrete_builder(
        full_generics.type_params(),
        syn::parse_quote! { ::boulder::builder::BoxConverter },
    );
    let arc_mutex_builder_type = make_concrete_builder(
        full_generics.type_params(),
        syn::parse_quote! { ::boulder::builder::ArcMutexConverter },
    );

    let mut body = pm2::TokenStream::new();
    let mut methods = pm2::TokenStream::new();
    let mut make_body = pm2::TokenStream::new();
    let mut defaults = pm2::TokenStream::new();

    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(syn::FieldsNamed { named, .. }) = s.fields {
            for field in named.iter() {
                let fieldid = field.ident.as_ref().unwrap();
                let fieldtype = &field.ty;
                let mut builder = BuildType::Default;
                let mut generator = GeneratorType::Default;
                let mut sequence = None;

                for attr in field.attrs.iter() {
                    if attr.path.is_ident("boulder") {
                        let parsed = attr
                            .parse_args::<BuilderMeta>()
                            .expect("failed to parse boulder attribute");
                        if let BuildType::Default = builder {
                            builder = parsed.builder.element;
                        }
                        if let GeneratorType::Default = generator {
                            generator = parsed.generator.element;
                        }
                        if sequence.is_none() {
                            sequence = parsed.builder.sequence;
                        }
                    }
                }

                if let Some(sequence) = sequence {
                    let mut gen_init = pm2::TokenStream::new();
                    match generator {
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
                                    let mut init = pm2::TokenStream::new();

                                    init.extend(quote::quote! {
                                        <<#fieldtype as std::iter::IntoIterator>::Item as ::boulder::Buildable>::builder()
                                    });
                                    for (k, v) in map {
                                        init.extend(quote::quote! {
                                            .#k(#v)
                                        });
                                    }

                                    static_value.extend(quote::quote!{
                                        <<<#fieldtype as std::iter::IntoIterator>::Item as ::boulder::Buildable>::Builder as ::boulder::Builder>::build(#init),
                                    });
                                }
                                BuildType::Value(value) => {
                                    static_value.extend(quote::quote! {
                                        (#value).into(),
                                    });
                                }
                                BuildType::Default => {
                                    static_value.extend(quote::quote! {
                                        Default::default()
                                    });
                                }
                            };
                            gen_init.extend(quote::quote! {
                                || #static_value
                            });
                        }
                    }
                    defaults.extend(quote::quote! {
                        #fieldid: ::boulder::GeneratorIterator::new(#gen_init).take(#sequence).collect(),
                    })
                } else {
                    match builder {
                        BuildType::Buildable(map) => {
                            let mut init = pm2::TokenStream::new();

                            init.extend(quote::quote! {
                                <#fieldtype as ::boulder::Buildable>::builder()
                            });
                            for (k, v) in map {
                                init.extend(quote::quote! {
                                    .#k(#v)
                                });
                            }
                            defaults.extend(quote::quote! {
                                #fieldid: <<#fieldtype as ::boulder::Buildable>::Builder as ::boulder::Builder>::build(#init),
                            });
                        }
                        BuildType::Value(value) => defaults.extend(quote::quote! {
                            #fieldid: (#value).into(),
                        }),
                        BuildType::Default => defaults.extend(quote::quote! {
                            #fieldid: Default::default(),
                        }),
                    };
                }

                body.extend(quote::quote! {
                    #fieldid: #fieldtype,
                });
                methods.extend(quote::quote! {
                    pub fn #fieldid<S>(mut self, value: S) -> Self
                    where
                        S: Into<#fieldtype>
                    {
                        self.#fieldid = value.into();
                        self
                    }
                });
                make_body.extend(quote::quote! {
                    #fieldid: self.#fieldid,
                });
            }
        }
    }

    let res = quote::quote! {
        const _: () = {
            #vis struct Builder #builder_generics #builder_wc {
                converter: BoulderConverterParam,
                #body
            }

            #[automatically_derived]
            impl #builder_generics Builder #builder_ty_generics #builder_wc {
                pub fn new(converter: BoulderConverterParam) -> Self
                {
                    Self {
                        converter,
                        #defaults
                    }
                }

                #methods
            }

            #[automatically_derived]
            impl #builder_generics ::boulder::Builder for Builder #builder_ty_generics #builder_wc {
                type Result = BoulderConverterParam::Output;
                fn build(self) -> Self::Result {
                    self.converter.convert(#ident {
                        #make_body
                    })
                }
            }

            #[automatically_derived]
            impl #generics ::boulder::Buildable for #ident #ty_generics #wc {
                type Builder = #self_builder_type;
                fn builder() -> Self::Builder {
                    Builder::new(::boulder::builder::SelfConverter)
                }
            }

            #[automatically_derived]
            impl #generics ::boulder::builder::OptionBuildable for #ident #ty_generics #wc {
                type Builder = #option_builder_type;
                fn option_builder() -> Self::Builder {
                    Builder::new(::boulder::builder::OptionConverter)
                }
            }

            #[automatically_derived]
            impl #generics ::boulder::builder::RcBuildable for #ident #ty_generics #wc {
                type Builder = #rc_builder_type;
                fn rc_builder() -> Self::Builder {
                    Builder::new(::boulder::builder::RcConverter)
                }
            }

            #[automatically_derived]
            impl #generics ::boulder::builder::ArcBuildable for #ident #ty_generics #wc {
                type Builder = #arc_builder_type;
                fn arc_builder() -> Self::Builder {
                    Builder::new(::boulder::builder::ArcConverter)
                }
            }

            #[automatically_derived]
            impl #generics ::boulder::builder::MutexBuildable for #ident #ty_generics #wc {
                type Builder = #mutex_builder_type;
                fn mutex_builder() -> Self::Builder {
                    Builder::new(::boulder::builder::MutexConverter)
                }
            }

            #[automatically_derived]
            impl #generics ::boulder::builder::RefCellBuildable for #ident #ty_generics #wc {
                type Builder = #ref_cell_builder_type;
                fn ref_cell_builder() -> Self::Builder {
                    Builder::new(::boulder::builder::RefCellConverter)
                }
            }

            #[automatically_derived]
            impl #generics ::boulder::builder::CellBuildable for #ident #ty_generics #wc {
                type Builder = #cell_builder_type;
                fn cell_builder() -> Self::Builder {
                    Builder::new(::boulder::builder::CellConverter)
                }
            }

            #[automatically_derived]
            impl #generics ::boulder::builder::BoxBuildable for #ident #ty_generics #wc {
                type Builder = #box_builder_type;
                fn box_builder() -> Self::Builder {
                    Builder::new(::boulder::builder::BoxConverter)
                }
            }

            #[automatically_derived]
            impl #generics ::boulder::builder::ArcMutexBuildable for #ident #ty_generics #wc {
                type Builder = #arc_mutex_builder_type;
                fn arc_mutex_builder() -> Self::Builder {
                    Builder::new(::boulder::builder::ArcMutexConverter)
                }
            }
        };
    };

    //println!("{}", res);
    res
}
