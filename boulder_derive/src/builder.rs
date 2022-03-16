use proc_macro2 as pm2;

use crate::attributes::*;

pub fn derive_builder(input: syn::DeriveInput) -> pm2::TokenStream {
    let syn::DeriveInput {
        ident,
        data,
        generics,
        vis,
        ..
    } = input;

    let (generics, ty_generics, wc) = generics.split_for_impl();

    let mut body = pm2::TokenStream::new();
    let mut methods = pm2::TokenStream::new();
    let mut make_body = pm2::TokenStream::new();
    let mut defaults = pm2::TokenStream::new();
    let mut option_methods = pm2::TokenStream::new();

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
                option_methods.extend(quote::quote! {
                    pub fn #fieldid<S>(mut self, value: S) -> Self
                    where
                        S: Into<#fieldtype>
                    {
                        self.inner = self.inner.#fieldid(value);
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
            #vis struct Builder #generics #wc {
                #body
            }

            #[automatically_derived]
            impl #generics Builder #ty_generics #wc {
                #methods
            }

            #[automatically_derived]
            impl #generics ::boulder::Builder for Builder #ty_generics #wc {
                type Result = #ident #ty_generics;
                fn build(self) -> Self::Result {
                    #ident {
                        #make_body
                    }
                }
            }

            #[automatically_derived]
            impl #generics ::boulder::Buildable for #ident #ty_generics #wc {
                type Builder = Builder #ty_generics;
                fn builder() -> Self::Builder {
                    Builder {
                        #defaults
                    }
                }
            }
        };
    };

    //println!("{}", res);
    res
}
