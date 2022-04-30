use proc_macro2 as pm2;

use crate::attributes::*;

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
                                        <<<#fieldtype as std::iter::IntoIterator>::Item as ::boulder::Buildable>::Builder as ::boulder::builder::guts::MegaBuilder<<#fieldtype as std::iter::IntoIterator>::Item as ::boulder::Buildable>>::build(#init),
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
                                #fieldid: <<#fieldtype as ::boulder::Buildable>::Builder as ::boulder::builder::guts::MiniBuilder<<#fieldtype as ::boulder::builder::guts::BuilderBase>::Base>>::build(#init),
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

    // if Self<T1,T2,T3> then bare generics = , T1, T2, T3
    // bare_wc
    let res = quote::quote! {
        const _: () = {
            #vis struct Builder <BoulderTypeMarkerParam #bare_generics> #wc {
                _marker: ::core::marker::PhantomData<BoulderTypeMarkerParam>,
                #body
            }

            #[automatically_derived]
            impl <BoulderTypeMarkerParam #bare_generics> Builder <BoulderTypeMarkerParam #bare_ty_generics> #wc {
                pub fn new() -> Self
                {
                    Self {
                        _marker: Default::default(),
                        #defaults
                    }
                }

                fn change_type<BoulderFunctionTypeParam>(self) -> Builder<BoulderFunctionTypeParam #bare_ty_generics> {
                    Builder {
                        _marker: Default::default(),
                        #make_body
                    }
                }

                #methods
            }

            #[automatically_derived]
            impl #generics ::boulder::builder::guts::BuilderBase for #ident #ty_generics #wc {
                type Base = #ident #ty_generics;
            }

            #[automatically_derived]
            impl #generics ::boulder::builder::guts::MiniBuildable<#ident #ty_generics> for #ident #ty_generics #wc {
                type Builder = Builder<#ident #ty_generics #bare_ty_generics>;
                fn mini_builder() -> Self::Builder {
                    Builder::new()
                }
            }
            
            #[automatically_derived]
            impl #generics ::boulder::builder::guts::MiniBuilder<#ident #ty_generics> for Builder<#ident #ty_generics #bare_ty_generics> #wc
            {
                fn build(self) -> #ident #ty_generics {
                    #ident {
                        #make_body
                    }
                }
            }

            #[automatically_derived]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::builder::guts::MiniBuildable<#ident #ty_generics> for Option<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::builder::guts::MiniBuildable<#ident #ty_generics>,
                Builder<BoulderExtraGenericParam #bare_ty_generics>: ::boulder::builder::guts::MiniBuilder<BoulderExtraGenericParam>,
                #bare_wc
            {
                type Builder = Builder<Option<BoulderExtraGenericParam> #bare_ty_generics>;
                fn mini_builder() -> Self::Builder {
                    Builder::new()
                }
            }
            
            #[automatically_derived]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::builder::guts::MiniBuilder<Option<BoulderExtraGenericParam>> for Builder<Option<BoulderExtraGenericParam> #bare_ty_generics>
            where
                Builder<BoulderExtraGenericParam #bare_ty_generics>: ::boulder::builder::guts::MiniBuilder<BoulderExtraGenericParam>,
                #bare_wc
            {
                fn build(self) -> Option<BoulderExtraGenericParam> {
                    Some( <Builder<BoulderExtraGenericParam #bare_ty_generics> as ::boulder::builder::guts::MiniBuilder<BoulderExtraGenericParam>>::build(self.change_type()) )
                }
            }
            
        };
    };

    //println!("{}", res);
    res
}
