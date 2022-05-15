use super::helpers::get_persian_rug_constraints;
use proc_macro2 as pm2;

use crate::attributes::*;

fn make_generator_id_for_field(field: &syn::Field) -> syn::Ident {
    syn::Ident::new(
        &format!("BoulderDefaultGenerator_{}", field.ident.as_ref().unwrap()),
        pm2::Span::call_site(),
    )
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
    let mut change_type_body = pm2::TokenStream::new();
    let mut defaults = pm2::TokenStream::new();
    let mut dyn_generators = pm2::TokenStream::new();

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
                let mut sequence_needs_context = false;

                for attr in field.attrs.iter() {
                    if attr.path.is_ident("boulder") {
                        let parsed = match attr.parse_args::<BuilderMetaWithPersianRug>() {
                            Ok(parsed) => parsed,
                            Err(e) => return e.to_compile_error(),
                        };
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
                            sequence_needs_context = parsed.builder.sequence_needs_context;
                        }
                    }
                }

                if let Some(sequence) = sequence {
                    let mut gen_init = pm2::TokenStream::new();
                    let needs_context;
                    match &generator {
                        GeneratorType::Generator { expr, .. } => {
                            gen_init.extend(quote::quote! {
                                {
                                    #expr
                                }
                            });
                            needs_context = generator_needs_context;
                        }
                        GeneratorType::Generatable(map) => {
                            let mut inner = pm2::TokenStream::new();
                            if generator_needs_context {
                                inner.extend(quote::quote! {
                                    <<#fieldtype as std::iter::IntoIterator>::Item as ::boulder::GeneratableWithPersianRug<#context>>::generator()
                                });
                            } else {
                                inner.extend(quote::quote!{
                                    <<#fieldtype as std::iter::IntoIterator>::Item as ::boulder::Generatable>::generator()
                                });
                            }
                            needs_context = generator_needs_context;

                            for (k, v) in map {
                                inner.extend(quote::quote! {
                                    .#k(#v)
                                });
                            }
                            gen_init.extend(inner);
                        }
                        GeneratorType::Default => {
                            let mut static_value = pm2::TokenStream::new();
                            needs_context = builder_needs_context;
                            match builder {
                                BuildType::Buildable(map) => {
                                    if needs_context {
                                        let mut init = pm2::TokenStream::new();
                                        init.extend(quote::quote! {
                                            <<#fieldtype as std::iter::IntoIterator>::Item as ::boulder::BuildableWithPersianRug<#context>>::builder()
                                        });
                                        for (k, v) in map {
                                            init.extend(quote::quote! {
                                                .#k(#v)
                                            });
                                        }
                                        static_value.extend(quote::quote!{
                                            <<<#fieldtype as std::iter::IntoIterator>::Item as ::boulder::BuildableWithPersianRug<#context>>::Builder as ::boulder::BuilderWithPersianRug<#context>>::build(#init, context)
                                        });
                                    } else {
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
                                            <<<#fieldtype as std::iter::IntoIterator>::Item as ::boulder::Buildable>::Builder as ::boulder::Builder>::build(#init)
                                        });
                                    }
                                }
                                BuildType::Value { expr: value, .. } => {
                                    if needs_context {
                                        static_value.extend(quote::quote! {
                                            let closure = { #value };
                                            let (result, context) = closure(context);
                                            (result.into(), context)
                                        });
                                    } else {
                                        static_value.extend(quote::quote! {
                                            (#value).into()
                                        });
                                    }
                                }
                                BuildType::Default => {
                                    static_value.extend(quote::quote! {
                                        Default::default()
                                    });
                                }
                            };

                            if needs_context {
                                let new_generator_id = make_generator_id_for_field(field);

                                dyn_generators.extend(quote::quote! {
                                    // This is just so the generics are used and in scope for #fieldtype
                                    struct #new_generator_id #generics {
                                        _marker: core::marker::PhantomData<#ident #ty_generics>
                                    }

                                    #[persian_rug::constraints(#constraints)]
                                    impl #generics ::boulder::persian_rug::GeneratorWithPersianRug<#context> for #new_generator_id #ty_generics #wc
                                    {
                                        type Output = <#fieldtype as IntoIterator>::Item;
                                        fn generate<'boulder_mutator_lifetime, BoulderMutatorParam>(
                                            &mut self, context: BoulderMutatorParam
                                        ) -> (Self::Output, BoulderMutatorParam)
                                        where
                                            BoulderMutatorParam: 'boulder_mutator_lifetime + ::persian_rug::Mutator<Context = #context>
                                        {
                                            #static_value
                                        }
                                    }
                                });
                                gen_init.extend(quote::quote! {
                                    #new_generator_id { _marker: Default::default() }
                                });
                            } else {
                                gen_init.extend(quote::quote! {
                                    || { #static_value }
                                });
                            }
                        }
                    }

                    let sequence = {
                        let (seq, _ty) = sequence;
                        if sequence_needs_context {
                            quote::quote! {
                                let closure = { #seq };
                                let (count, mut context) = closure(context);
                            }
                        } else {
                            quote::quote! {
                                let count = #seq;
                            }
                        }
                    };

                    if needs_context {
                        defaults.extend(quote::quote! {
                            let (#fieldid, mut context) = if let Some(value) = self.#fieldid {
                                (value, context)
                            } else {
                                #sequence
                                let mut gen = #gen_init;
                                let mut iter = ::boulder::persian_rug::generator::GeneratorWithPersianRugIterator::new(gen, context);
                                let mut storage = Vec::new();
                                for _ in 0..count {
                                    storage.push(iter.next().unwrap());
                                }
                                let (gen, context) = iter.into_inner();
                                let value = storage.into_iter().collect();
                                (value, context)
                            };
                        });
                    } else {
                        defaults.extend(quote::quote! {
                            let (#fieldid, mut context) = if let Some(value) = self.#fieldid {
                                (value, context)
                            } else {
                                #sequence
                                let mut gen = #gen_init;
                                let iter = ::boulder::GeneratorIterator::new(gen);
                                let value = iter.take(count).collect();
                                (value, context)
                            };
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
                                    <#fieldtype as ::boulder::Buildable>::builder()
                                });
                                for (k, v) in map {
                                    initializers.extend(quote::quote! {
                                        .#k(#v)
                                    });
                                }
                                defaults.extend(quote::quote! {
                                    let #fieldid = if let Some(value) = self.#fieldid {
                                        value
                                    } else {
                                        <<#fieldtype as ::boulder::Buildable>::Builder as ::boulder::Builder>::build(#initializers)
                                    };
                                });
                            }
                        }
                        BuildType::Value { expr: value, .. } => {
                            if builder_needs_context {
                                defaults.extend(quote::quote! {
                                    let (#fieldid, context) = if let Some(value) = self.#fieldid {
                                        (value, context)
                                    } else {
                                        let closure = { #value };
                                        closure(context)
                                    };
                                });
                            } else {
                                defaults.extend(quote::quote! {
                                    let #fieldid = self.#fieldid.unwrap_or_else(|| (#value).into());
                                });
                            }
                        }
                        BuildType::Default => {
                            defaults.extend(quote::quote! {
                                let #fieldid = self.#fieldid.unwrap_or_default();
                            });
                        }
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
            #vis struct Builder<BoulderTypeMarkerParam #bare_generics> #wc {
                _boulder_created_marker: ::core::marker::PhantomData<BoulderTypeMarkerParam>,
                #body
            }

            #dyn_generators

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

            // Arc
            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::builder::guts::MiniBuildableWithPersianRug<#ident #ty_generics, #context> for ::std::sync::Arc<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::persian_rug::builder::guts::MiniBuildableWithPersianRug<#ident #ty_generics, #context>,
                Builder<BoulderExtraGenericParam #bare_ty_generics>: ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context, Result=BoulderExtraGenericParam>,
                #bare_wc
            {
                type Builder = Builder<::std::sync::Arc<BoulderExtraGenericParam> #bare_ty_generics>;
                fn mini_builder() -> Self::Builder {
                    Builder::new()
                }
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context> for Builder<::std::sync::Arc<BoulderExtraGenericParam> #bare_ty_generics>
            where
                Builder<BoulderExtraGenericParam #bare_ty_generics>: ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context, Result=BoulderExtraGenericParam>,
                #bare_wc
            {
                type Result = ::std::sync::Arc<BoulderExtraGenericParam>;
                fn build<'boulder_mutator_lifetime, BoulderMutatorParam>(self, mut context: BoulderMutatorParam) -> (Self::Result, BoulderMutatorParam)
                where
                    BoulderMutatorParam: 'boulder_mutator_lifetime + ::persian_rug::Mutator<Context=#context>
                {
                    let (result, context) = <Builder<BoulderExtraGenericParam #bare_ty_generics> as ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context>>::build(self.change_type(), context);
                    (::std::sync::Arc::new(result), context)
                }
            }

            // Rc
            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::builder::guts::MiniBuildableWithPersianRug<#ident #ty_generics, #context> for ::std::rc::Rc<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::persian_rug::builder::guts::MiniBuildableWithPersianRug<#ident #ty_generics, #context>,
                Builder<BoulderExtraGenericParam #bare_ty_generics>: ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context, Result=BoulderExtraGenericParam>,
                #bare_wc
            {
                type Builder = Builder<::std::rc::Rc<BoulderExtraGenericParam> #bare_ty_generics>;
                fn mini_builder() -> Self::Builder {
                    Builder::new()
                }
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context> for Builder<::std::rc::Rc<BoulderExtraGenericParam> #bare_ty_generics>
            where
                Builder<BoulderExtraGenericParam #bare_ty_generics>: ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context, Result=BoulderExtraGenericParam>,
                #bare_wc
            {
                type Result = ::std::rc::Rc<BoulderExtraGenericParam>;
                fn build<'boulder_mutator_lifetime, BoulderMutatorParam>(self, mut context: BoulderMutatorParam) -> (Self::Result, BoulderMutatorParam)
                where
                    BoulderMutatorParam: 'boulder_mutator_lifetime + ::persian_rug::Mutator<Context=#context>
                {
                    let (result, context) = <Builder<BoulderExtraGenericParam #bare_ty_generics> as ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context>>::build(self.change_type(), context);
                    (::std::rc::Rc::new(result), context)
                }
            }

            // Cell
            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::builder::guts::MiniBuildableWithPersianRug<#ident #ty_generics, #context> for ::std::cell::Cell<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::persian_rug::builder::guts::MiniBuildableWithPersianRug<#ident #ty_generics, #context>,
                Builder<BoulderExtraGenericParam #bare_ty_generics>: ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context, Result=BoulderExtraGenericParam>,
                #bare_wc
            {
                type Builder = Builder<::std::cell::Cell<BoulderExtraGenericParam> #bare_ty_generics>;
                fn mini_builder() -> Self::Builder {
                    Builder::new()
                }
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context> for Builder<::std::cell::Cell<BoulderExtraGenericParam> #bare_ty_generics>
            where
                Builder<BoulderExtraGenericParam #bare_ty_generics>: ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context, Result=BoulderExtraGenericParam>,
                #bare_wc
            {
                type Result = ::std::cell::Cell<BoulderExtraGenericParam>;
                fn build<'boulder_mutator_lifetime, BoulderMutatorParam>(self, mut context: BoulderMutatorParam) -> (Self::Result, BoulderMutatorParam)
                where
                    BoulderMutatorParam: 'boulder_mutator_lifetime + ::persian_rug::Mutator<Context=#context>
                {
                    let (result, context) = <Builder<BoulderExtraGenericParam #bare_ty_generics> as ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context>>::build(self.change_type(), context);
                    (::std::cell::Cell::new(result), context)
                }
            }

            // RefCell
            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::builder::guts::MiniBuildableWithPersianRug<#ident #ty_generics, #context> for ::std::cell::RefCell<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::persian_rug::builder::guts::MiniBuildableWithPersianRug<#ident #ty_generics, #context>,
                Builder<BoulderExtraGenericParam #bare_ty_generics>: ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context, Result=BoulderExtraGenericParam>,
                #bare_wc
            {
                type Builder = Builder<::std::cell::RefCell<BoulderExtraGenericParam> #bare_ty_generics>;
                fn mini_builder() -> Self::Builder {
                    Builder::new()
                }
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context> for Builder<::std::cell::RefCell<BoulderExtraGenericParam> #bare_ty_generics>
            where
                Builder<BoulderExtraGenericParam #bare_ty_generics>: ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context, Result=BoulderExtraGenericParam>,
                #bare_wc
            {
                type Result = ::std::cell::RefCell<BoulderExtraGenericParam>;
                fn build<'boulder_mutator_lifetime, BoulderMutatorParam>(self, mut context: BoulderMutatorParam) -> (Self::Result, BoulderMutatorParam)
                where
                    BoulderMutatorParam: 'boulder_mutator_lifetime + ::persian_rug::Mutator<Context=#context>
                {
                    let (result, context) = <Builder<BoulderExtraGenericParam #bare_ty_generics> as ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context>>::build(self.change_type(), context);
                    (::std::cell::RefCell::new(result), context)
                }
            }

            // Mutex
            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::builder::guts::MiniBuildableWithPersianRug<#ident #ty_generics, #context> for ::std::sync::Mutex<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::persian_rug::builder::guts::MiniBuildableWithPersianRug<#ident #ty_generics, #context>,
                Builder<BoulderExtraGenericParam #bare_ty_generics>: ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context, Result=BoulderExtraGenericParam>,
                #bare_wc
            {
                type Builder = Builder<::std::sync::Mutex<BoulderExtraGenericParam> #bare_ty_generics>;
                fn mini_builder() -> Self::Builder {
                    Builder::new()
                }
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context> for Builder<::std::sync::Mutex<BoulderExtraGenericParam> #bare_ty_generics>
            where
                Builder<BoulderExtraGenericParam #bare_ty_generics>: ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context, Result=BoulderExtraGenericParam>,
                #bare_wc
            {
                type Result = ::std::sync::Mutex<BoulderExtraGenericParam>;
                fn build<'boulder_mutator_lifetime, BoulderMutatorParam>(self, mut context: BoulderMutatorParam) -> (Self::Result, BoulderMutatorParam)
                where
                    BoulderMutatorParam: 'boulder_mutator_lifetime + ::persian_rug::Mutator<Context=#context>
                {
                    let (result, context) = <Builder<BoulderExtraGenericParam #bare_ty_generics> as ::boulder::persian_rug::builder::guts::MiniBuilderWithPersianRug<#context>>::build(self.change_type(), context);
                    (::std::sync::Mutex::new(result), context)
                }
            }
        };
    };

    res
}
