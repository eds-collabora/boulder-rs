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

                let sequence = if let Some((sequence, _ty)) = sequence {
                    Some(quote::quote! { #sequence })
                } else {
                    build_sequence.map(|(sequence, _ty)| {
                        quote::quote! { || ((#sequence) as usize) }
                    })
                };

                body.extend(quote::quote! {
                    #fieldid: Box<dyn ::boulder::Generator<Output=#fieldtype>>,
                });

                methods.extend(quote::quote! {
                    pub fn #fieldid<V>(mut self, generator: V) -> Self
                    where
                        V: 'static + ::boulder::Generator<Output=#fieldtype>
                    {
                        self.#fieldid = Box::new(generator);
                        self
                    }
                });

                make_body.extend(quote::quote! {
                    #fieldid: gen.#fieldid.generate(),
                });

                let element_type = if sequence.is_some() {
                    quote::quote! { <#fieldtype as std::iter::IntoIterator>::Item }
                } else {
                    quote::quote! { #fieldtype }
                };

                let value = match generator {
                    GeneratorType::Generator { expr, .. } => {
                        quote::quote! {
                            #expr
                        }
                    }
                    GeneratorType::Generatable(map) => {
                        let mut inner = pm2::TokenStream::new();
                        inner.extend(quote::quote! {
                            <#element_type as ::boulder::Generatable>::generator()
                        });

                        for (k, v) in map {
                            inner.extend(quote::quote! {
                                .#k(#v)
                            });
                        }
                        inner
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
                            BuildType::Value { expr: value, .. } => {
                                static_value.extend(quote::quote! {
                                    (#value).into()
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
                            let mut value = { #value };
                            Box::new(move || {
                                ::boulder::GeneratorMutIterator::new(
                                    &mut value
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

    let bare_generics = if full_generics.params.is_empty() {
        quote::quote! {}
    } else {
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
                }
                syn::GenericParam::Lifetime(syn::LifetimeDef { lifetime, .. }) => {
                    res.extend(quote::quote! {
                        , #lifetime
                    });
                }
                syn::GenericParam::Const(syn::ConstParam {
                    const_token, ident, ..
                }) => {
                    res.extend(quote::quote! {
                        , #const_token #ident
                    });
                }
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
            #vis struct Generator<BoulderTypeMarkerParam #bare_generics> #wc {
                _boulder_type_marker: ::core::marker::PhantomData<BoulderTypeMarkerParam>,
                #body
            }

            #vis trait NestedGenerate #generics #wc {
                type Output;
                fn nested_generate<BoulderFunctionParam>(gen: &mut Generator<BoulderFunctionParam #bare_ty_generics>) -> Self::Output;
            }

            #[automatically_derived]
            impl<BoulderTypeMarkerParam #bare_generics> Generator <BoulderTypeMarkerParam #bare_ty_generics> #wc {
                pub fn new() -> Self
                {
                    Self {
                        _boulder_type_marker: Default::default(),
                        #defaults
                    }
                }

                #methods
            }

            #[automatically_derived]
            impl<BoulderTypeMarkerParam #bare_generics> ::boulder::guts::generator::MiniGenerator for Generator<BoulderTypeMarkerParam #bare_ty_generics>

            where
                Self: NestedGenerate #ty_generics,
                BoulderTypeMarkerParam: 'static,
                #bare_wc
            {
                type Output = <Self as NestedGenerate #ty_generics>::Output;
                fn generate(&mut self) -> Self::Output {
                    <Self as NestedGenerate #ty_generics>::nested_generate(self)
                }
            }

            #[automatically_derived]
            impl #generics ::boulder::guts::generator::BoulderBase for #ident #ty_generics #wc {
                type Base = #ident #ty_generics;
            }

            // Base case

            #[automatically_derived]
            impl #generics ::boulder::guts::generator::MiniGeneratable<#ident #ty_generics> for #ident #ty_generics #wc {
                type Generator = Generator<#ident #ty_generics #bare_ty_generics>;
                fn mini_generator() -> Self::Generator {
                    Generator::new()
                }
            }

            #[automatically_derived]
            impl #generics NestedGenerate #ty_generics for Generator<#ident #ty_generics #bare_ty_generics> #wc {
                type Output = #ident #ty_generics;
                fn nested_generate<BoulderFunctionParam>(gen: &mut Generator<BoulderFunctionParam #bare_ty_generics>) -> Self::Output {
                    #ident {
                        #make_body
                    }
                }
            }

            // Option
            #[automatically_derived]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::guts::generator::MiniGeneratable<#ident #ty_generics> for Option<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::guts::generator::MiniGeneratable<#ident #ty_generics>,
                Generator<Option<BoulderExtraGenericParam> #bare_ty_generics>: ::boulder::guts::generator::MiniGenerator<Output=Option<BoulderExtraGenericParam>>,
                #bare_wc
            {
                type Generator = Generator<Option<BoulderExtraGenericParam> #bare_ty_generics>;
                fn mini_generator() -> Self::Generator {
                    Generator::new()
                }
            }

            #[automatically_derived]
            impl <BoulderExtraGenericParam #bare_generics> NestedGenerate #ty_generics for Generator<Option<BoulderExtraGenericParam> #bare_ty_generics>
            where
                Generator<BoulderExtraGenericParam #bare_ty_generics>: NestedGenerate #ty_generics,
                #bare_wc
            {
                type Output = Option<<Generator<BoulderExtraGenericParam #bare_ty_generics> as NestedGenerate #ty_generics>::Output>;
                fn nested_generate<BoulderFunctionParam>(gen: &mut Generator<BoulderFunctionParam #bare_ty_generics>) -> Self::Output {
                    Some( Generator::<BoulderExtraGenericParam #bare_ty_generics>::nested_generate(gen) )
                }
            }

            // Rc
            #[automatically_derived]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::guts::generator::MiniGeneratable<#ident #ty_generics> for ::std::rc::Rc<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::guts::generator::MiniGeneratable<#ident #ty_generics>,
                Generator<::std::rc::Rc<BoulderExtraGenericParam> #bare_ty_generics>: ::boulder::guts::generator::MiniGenerator<Output=::std::rc::Rc<BoulderExtraGenericParam>>,
                #bare_wc
            {
                type Generator = Generator<::std::rc::Rc<BoulderExtraGenericParam> #bare_ty_generics>;
                fn mini_generator() -> Self::Generator {
                    Generator::new()
                }
            }

            #[automatically_derived]
            impl <BoulderExtraGenericParam #bare_generics> NestedGenerate #ty_generics for Generator<::std::rc::Rc<BoulderExtraGenericParam> #bare_ty_generics>
            where
                Generator<BoulderExtraGenericParam #bare_ty_generics>: NestedGenerate #ty_generics,
                #bare_wc
            {
                type Output = ::std::rc::Rc<<Generator<BoulderExtraGenericParam #bare_ty_generics> as NestedGenerate #ty_generics>::Output>;
                fn nested_generate<BoulderFunctionParam>(gen: &mut Generator<BoulderFunctionParam #bare_ty_generics>) -> Self::Output {
                    ::std::rc::Rc::new( Generator::<BoulderExtraGenericParam #bare_ty_generics>::nested_generate(gen) )
                }
            }

            // Arc
            #[automatically_derived]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::guts::generator::MiniGeneratable<#ident #ty_generics> for ::std::sync::Arc<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::guts::generator::MiniGeneratable<#ident #ty_generics>,
                Generator<::std::sync::Arc<BoulderExtraGenericParam> #bare_ty_generics>: ::boulder::guts::generator::MiniGenerator<Output=::std::sync::Arc<BoulderExtraGenericParam>>,
                #bare_wc
            {
                type Generator = Generator<::std::sync::Arc<BoulderExtraGenericParam> #bare_ty_generics>;
                fn mini_generator() -> Self::Generator {
                    Generator::new()
                }
            }

            #[automatically_derived]
            impl <BoulderExtraGenericParam #bare_generics> NestedGenerate #ty_generics for Generator<::std::sync::Arc<BoulderExtraGenericParam> #bare_ty_generics>
            where
                Generator<BoulderExtraGenericParam #bare_ty_generics>: NestedGenerate #ty_generics,
                #bare_wc
            {
                type Output = ::std::sync::Arc<<Generator<BoulderExtraGenericParam #bare_ty_generics> as NestedGenerate #ty_generics>::Output>;
                fn nested_generate<BoulderFunctionParam>(gen: &mut Generator<BoulderFunctionParam #bare_ty_generics>) -> Self::Output {
                    ::std::sync::Arc::new( Generator::<BoulderExtraGenericParam #bare_ty_generics>::nested_generate(gen) )
                }
            }

            // Mutex
            #[automatically_derived]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::guts::generator::MiniGeneratable<#ident #ty_generics> for ::std::sync::Mutex<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::guts::generator::MiniGeneratable<#ident #ty_generics>,
                Generator<::std::sync::Mutex<BoulderExtraGenericParam> #bare_ty_generics>: ::boulder::guts::generator::MiniGenerator<Output=::std::sync::Mutex<BoulderExtraGenericParam>>,
                #bare_wc
            {
                type Generator = Generator<::std::sync::Mutex<BoulderExtraGenericParam> #bare_ty_generics>;
                fn mini_generator() -> Self::Generator {
                    Generator::new()
                }
            }

            #[automatically_derived]
            impl <BoulderExtraGenericParam #bare_generics> NestedGenerate #ty_generics for Generator<::std::sync::Mutex<BoulderExtraGenericParam> #bare_ty_generics>
            where
                Generator<BoulderExtraGenericParam #bare_ty_generics>: NestedGenerate #ty_generics,
                #bare_wc
            {
                type Output = ::std::sync::Mutex<<Generator<BoulderExtraGenericParam #bare_ty_generics> as NestedGenerate #ty_generics>::Output>;
                fn nested_generate<BoulderFunctionParam>(gen: &mut Generator<BoulderFunctionParam #bare_ty_generics>) -> Self::Output {
                    ::std::sync::Mutex::new( Generator::<BoulderExtraGenericParam #bare_ty_generics>::nested_generate(gen) )
                }
            }

            // Cell
            #[automatically_derived]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::guts::generator::MiniGeneratable<#ident #ty_generics> for ::std::cell::Cell<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::guts::generator::MiniGeneratable<#ident #ty_generics>,
                Generator<::std::cell::Cell<BoulderExtraGenericParam> #bare_ty_generics>: ::boulder::guts::generator::MiniGenerator<Output=::std::cell::Cell<BoulderExtraGenericParam>>,
                #bare_wc
            {
                type Generator = Generator<::std::cell::Cell<BoulderExtraGenericParam> #bare_ty_generics>;
                fn mini_generator() -> Self::Generator {
                    Generator::new()
                }
            }

            #[automatically_derived]
            impl <BoulderExtraGenericParam #bare_generics> NestedGenerate #ty_generics for Generator<::std::cell::Cell<BoulderExtraGenericParam> #bare_ty_generics>
            where
                Generator<BoulderExtraGenericParam #bare_ty_generics>: NestedGenerate #ty_generics,
                #bare_wc
            {
                type Output = ::std::cell::Cell<<Generator<BoulderExtraGenericParam #bare_ty_generics> as NestedGenerate #ty_generics>::Output>;
                fn nested_generate<BoulderFunctionParam>(gen: &mut Generator<BoulderFunctionParam #bare_ty_generics>) -> Self::Output {
                    ::std::cell::Cell::new( Generator::<BoulderExtraGenericParam #bare_ty_generics>::nested_generate(gen) )
                }
            }

            // RefCell
            #[automatically_derived]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::guts::generator::MiniGeneratable<#ident #ty_generics> for ::std::cell::RefCell<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::guts::generator::MiniGeneratable<#ident #ty_generics>,
                Generator<::std::cell::RefCell<BoulderExtraGenericParam> #bare_ty_generics>: ::boulder::guts::generator::MiniGenerator<Output=::std::cell::RefCell<BoulderExtraGenericParam>>,
                #bare_wc
            {
                type Generator = Generator<::std::cell::RefCell<BoulderExtraGenericParam> #bare_ty_generics>;
                fn mini_generator() -> Self::Generator {
                    Generator::new()
                }
            }

            #[automatically_derived]
            impl <BoulderExtraGenericParam #bare_generics> NestedGenerate #ty_generics for Generator<::std::cell::RefCell<BoulderExtraGenericParam> #bare_ty_generics>
            where
                Generator<BoulderExtraGenericParam #bare_ty_generics>: NestedGenerate #ty_generics,
                #bare_wc
            {
                type Output = ::std::cell::RefCell<<Generator<BoulderExtraGenericParam #bare_ty_generics> as NestedGenerate #ty_generics>::Output>;
                fn nested_generate<BoulderFunctionParam>(gen: &mut Generator<BoulderFunctionParam #bare_ty_generics>) -> Self::Output {
                    ::std::cell::RefCell::new( Generator::<BoulderExtraGenericParam #bare_ty_generics>::nested_generate(gen) )
                }
            }

            // Iterators

            #[automatically_derived]
            impl <BoulderExtraGenericParam #bare_generics> ::std::iter::IntoIterator for Generator<BoulderExtraGenericParam #bare_ty_generics>
            where
                Generator<BoulderExtraGenericParam #bare_ty_generics>: ::boulder::guts::generator::MiniGenerator<Output=BoulderExtraGenericParam>,
                #bare_wc
            {
                type Item = BoulderExtraGenericParam;
                type IntoIter = ::boulder::GeneratorIterator<Self>;
                fn into_iter(self) -> Self::IntoIter {
                    Self::IntoIter::new(self)
                }
            }

            #[automatically_derived]
            impl<'boulder_reference_lifetime, BoulderExtraGenericParam #bare_generics> ::std::iter::IntoIterator for &'boulder_reference_lifetime mut Generator<BoulderExtraGenericParam #bare_ty_generics>
            where
                Generator<BoulderExtraGenericParam #bare_ty_generics>: ::boulder::guts::generator::MiniGenerator<Output=BoulderExtraGenericParam>,
               #bare_wc
            {
                type Item = BoulderExtraGenericParam;
                type IntoIter = ::boulder::GeneratorMutIterator<'boulder_reference_lifetime, Generator<BoulderExtraGenericParam #bare_ty_generics>>;
                fn into_iter(self) -> Self::IntoIter {
                    Self::IntoIter::new(self)
                }
            }
        };
    };

    res
}
