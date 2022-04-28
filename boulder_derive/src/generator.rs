use proc_macro2 as pm2;

use crate::attributes::*;

pub fn derive_generatable(input: syn::DeriveInput) -> pm2::TokenStream {
    let syn::DeriveInput {
        ident,
        data,
        generics: full_generics,
        vis,
        ..
    } = input;

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
                let mut build_sequence = None;

                for attr in field.attrs.iter() {
                    if attr.path.is_ident("boulder") {
                        let parsed = match attr.parse_args::<BuilderMeta>() {
                            Ok(parsed) => parsed,
                            Err(e) => return e.to_compile_error(),
                        };
                        if let BuildType::Default = builder {
                            builder = parsed.builder.element
                        }
                        if let GeneratorType::Default = generator {
                            generator = parsed.generator.element
                        }
                        if sequence.is_none() {
                            sequence = parsed.generator.sequence;
                        }
                        if build_sequence.is_none() {
                            build_sequence = parsed.builder.sequence;
                        }
                    }
                }

                let sequence = if let Some(sequence) = sequence {
                    Some(quote::quote! { #sequence })
                } else {
                    // if let Some(sequence) = build_sequence {
                    build_sequence.map(|sequence| {
                        quote::quote! { || ((#sequence) as usize) }
                    })
                };

                body.extend(quote::quote! {
                    #fieldid: Box<dyn ::boulder::Generator<Output=#fieldtype>>,
                });

                methods.extend(quote::quote! {
                    pub fn #fieldid<V>(&mut self, generator: V) -> &mut Self
                    where
                        V: ::boulder::Generator<Output=#fieldtype>
                    {
                        self.#fieldid = Box::new(generator);
                        self
                    }
                });

                make_body.extend(quote::quote! {
                    #fieldid: self.#fieldid.generate(),
                });

                let element_type = if sequence.is_some() {
                    quote::quote! { <#fieldtype as std::iter::IntoIterator>::Item }
                } else {
                    quote::quote! { #fieldtype }
                };

                let value = match generator {
                    GeneratorType::Generator(expr) => {
                        quote::quote! {
                            #expr
                        }
                    }
                    GeneratorType::Generatable(map) => {
                        let mut inner = pm2::TokenStream::new();
                        inner.extend(quote::quote! {
                            let mut gen = <#element_type as ::boulder::Generatable>::generator();
                        });

                        for (k, v) in map {
                            inner.extend(quote::quote! {
                                gen.#k(#v);
                            });
                        }
                        inner.extend(quote::quote! {
                            gen
                        });
                        quote::quote! {
                            {
                                #inner
                            }
                        }
                    }
                    GeneratorType::Default => {
                        let mut static_value = pm2::TokenStream::new();
                        match builder {
                            BuildType::Buildable(map) => {
                                let mut init = pm2::TokenStream::new();

                                init.extend(quote::quote! {
                                    <#element_type as ::boulder::Buildable>::builder()
                                });
                                for (k, v) in map {
                                    init.extend(quote::quote! {
                                        .#k(#v)
                                    });
                                }

                                static_value.extend(quote::quote!{
                                    <<#element_type as ::boulder::Buildable>::Builder as ::boulder::Builder>::build(#init)
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
                        }
                        quote::quote! {
                            || #static_value
                        }
                    }
                };

                if let Some(sequence) = sequence {
                    defaults.extend(quote::quote! {
                        #fieldid: {
                            let mut seq = #sequence;
                            Box::new(move || {
                                ::boulder::GeneratorIterator::new(
                                    #value
                                ).take(::boulder::Generator::generate(&mut seq).into()).collect()
                            })
                        },
                    })
                } else {
                    defaults.extend(quote::quote! {
                        #fieldid: Box::new(#value),
                    })
                }
            }
        }
    }

    let gparams = &full_generics.params;

    let res = quote::quote! {
        const _: () = {
            #vis struct Generator #generics #wc {
                #body
            }

            #[automatically_derived]
            impl #generics Generator #ty_generics #wc {
                #methods
            }

            #[automatically_derived]
            impl #generics ::boulder::Generator for Generator #ty_generics #wc {
                type Output = #ident #ty_generics;
                fn generate(&mut self) -> #ident #ty_generics {
                    #ident {
                        #make_body
                    }
                }
            }

            #[automatically_derived]
            impl #generics ::std::iter::IntoIterator for Generator #ty_generics #wc {
                type Item = #ident #ty_generics;
                type IntoIter = ::boulder::GeneratorIterator<Self>;
                fn into_iter(self) -> Self::IntoIter {
                    Self::IntoIter::new(self)
                }
            }

            #[automatically_derived]
            impl<'a, #gparams> ::std::iter::IntoIterator for &'a mut Generator #ty_generics #wc {
                type Item = #ident #ty_generics;
                type IntoIter = ::boulder::GeneratorMutIterator<'a, Generator #ty_generics>;
                fn into_iter(self) -> Self::IntoIter {
                    Self::IntoIter::new(self)
                }
            }

            #[automatically_derived]
            impl #generics ::boulder::Generatable for #ident #ty_generics #wc {
                type Generator = Generator #ty_generics;
                fn generator() -> Self::Generator {
                    Generator {
                        #defaults
                    }
                }
            }
        };
    };

    res
}
