// Possible restrictions to make this feasible:
// - No native lambdas (wrap in Generator)
// - No customisation over default of nested generatable_with_persian_rug items by default
// - Provide types as well as values for generator_with_persian_rug fields
// FUTURE:
// - Lambda restriction is intrinsic, because of the type of the fn
// - Customisation depends on impl Trait RFC 2515
// - Types can be removed when RFC 2515 lands

use super::helpers::get_persian_rug_constraints;
use proc_macro2 as pm2;

use crate::attributes::*;

fn make_generator_id_for_field(field: &syn::Field) -> syn::Ident {
    syn::Ident::new(
        &format!("BoulderDefaultGenerator_{}", field.ident.as_ref().unwrap()),
        pm2::Span::call_site(),
    )
}

fn make_sequence_generator_id_for_field(field: &syn::Field) -> syn::Ident {
    syn::Ident::new(
        &format!(
            "BoulderDefaultSequenceGenerator_{}",
            field.ident.as_ref().unwrap()
        ),
        pm2::Span::call_site(),
    )
}

fn make_generic_id_for_field(field: &syn::Field) -> syn::Ident {
    syn::Ident::new(
        &format!("BoulderGenericType_{}", field.ident.as_ref().unwrap()),
        pm2::Span::call_site(),
    )
}

fn make_method_body(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
    replace: &syn::Ident,
    replace_with: &syn::Expr,
    replace_type: &syn::Type,
) -> (pm2::TokenStream, pm2::TokenStream) {
    let mut body = pm2::TokenStream::new();
    let mut generics = pm2::TokenStream::new();
    for field in fields.iter() {
        let ident = &field.ident;
        if field.ident.as_ref().unwrap() == replace {
            body.extend(quote::quote! {
                #ident: #replace_with,
            });
            generics.extend(quote::quote! {
                , #replace_type
            });
        } else {
            body.extend(quote::quote! {
                #ident: self.#ident,
            });
            let ty = make_generic_id_for_field(field);
            generics.extend(quote::quote! {
                , #ty
            });
        }
    }
    (body, generics)
}

pub fn derive_generatable_with_persian_rug(input: syn::DeriveInput) -> pm2::TokenStream {
    let syn::DeriveInput {
        attrs,
        ident,
        data,
        generics,
        vis,
        ..
    } = input;

    let mut body = pm2::TokenStream::new();
    let mut methods = pm2::TokenStream::new();
    let mut make_body = pm2::TokenStream::new();
    let mut default_values = pm2::TokenStream::new();
    let mut default_types = pm2::TokenStream::new();
    let mut vars = pm2::TokenStream::new();
    let mut added_generics = pm2::TokenStream::new();
    let mut added_wc = pm2::TokenStream::new();
    let mut dyn_generators = pm2::TokenStream::new();

    let (context, used_types) = match get_persian_rug_constraints(&attrs) {
        Ok(context) => context,
        Err(e) => {
            return e.to_compile_error();
        }
    };

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
    let mut gen_generics = generics.clone();
    let (generics, ty_generics, wc) = generics.split_for_impl();

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

    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(syn::FieldsNamed { named, .. }) = s.fields {
            for field in named.iter() {
                let fieldid = field.ident.as_ref().unwrap();
                let fieldtype = &field.ty;
                let mut builder = BuildType::Default;
                let mut generator = GeneratorType::Default;
                let mut sequence = None;
                let mut build_sequence = None;
                let mut builder_needs_context = false;
                let mut generator_needs_context = false;
                let mut sequence_needs_context = false;
                let mut build_sequence_needs_context = false;

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
                            sequence = parsed.generator.sequence;
                            sequence_needs_context = parsed.generator.sequence_needs_context;
                        }
                        if build_sequence.is_none() {
                            build_sequence = parsed.builder.sequence;
                            build_sequence_needs_context = parsed.builder.sequence_needs_context;
                        }
                    }
                }

                let new_generic_arg = make_generic_id_for_field(field);

                body.extend(quote::quote! {
                    #fieldid: #new_generic_arg,
                });

                gen_generics
                    .params
                    .push(syn::parse_quote! { #new_generic_arg });
                gen_generics.make_where_clause().predicates.push(syn::parse_quote!{
                    #new_generic_arg: ::boulder::GeneratorWithPersianRug<#context, Output=#fieldtype>
                });
                added_generics.extend(quote::quote! {
                    , #new_generic_arg
                });
                added_wc.extend(quote::quote! {
                    #new_generic_arg: ::boulder::GeneratorWithPersianRug<#context, Output=#fieldtype>,
                });

                let (method_body, method_generics) = make_method_body(
                    &named,
                    field.ident.as_ref().unwrap(),
                    &syn::parse_quote! {
                        generator
                    },
                    &syn::parse_quote! {
                        BoulderFieldTypeParam
                    },
                );

                methods.extend(quote::quote! {
                    pub fn #fieldid<BoulderFieldTypeParam>(self, generator: BoulderFieldTypeParam) -> Generator<BoulderTypeMarkerParam #bare_ty_generics #method_generics>
                    where
                        BoulderFieldTypeParam: ::boulder::GeneratorWithPersianRug<#context, Output=#fieldtype>
                    {
                        Generator {
                            _boulder_type_marker: Default::default(),
                            _boulder_type_storage: Default::default(),
                            #method_body
                        }
                    }
                });

                vars.extend(quote::quote! {
                    let (#fieldid, mut context) = gen.#fieldid.generate(context);
                });

                make_body.extend(quote::quote! {
                    #fieldid,
                });

                let element_type = if sequence.is_some() || build_sequence.is_some() {
                    quote::quote! { <#fieldtype as std::iter::IntoIterator>::Item }
                } else {
                    quote::quote! { #fieldtype }
                };

                let (value, value_type) = match generator {
                    GeneratorType::Generator { expr, ty, .. } => {
                        if generator_needs_context {
                            if ty.is_none() {
                                return syn::Error::new_spanned(
                                    expr,
                                    "a type must be provided to use this value in GeneratableWithPersianRug (syntax: `value: type`)"
                                ).into_compile_error();
                            }
                            (quote::quote! { #expr }, quote::quote! { #ty })
                        } else {
                            (
                                quote::quote! { ::boulder::persian_rug::generator::GeneratorWrapper::new(#expr) },
                                quote::quote! { ::boulder::persian_rug::generator::GeneratorWrapper<#element_type> },
                            )
                        }
                    }
                    GeneratorType::Generatable(map) => {
                        if generator_needs_context {
                            if let Some(k) = map.keys().next() {
                                return syn::Error::new_spanned(
                                    k,
                                    "cannot customise nested persian-rug generators from within a persian-rug generator"
                                ).into_compile_error();
                            }
                            (
                                quote::quote! {
                                    <#element_type as ::boulder::GeneratableWithPersianRug<#context>>::generator()
                                },
                                quote::quote! {
                                    <#element_type as ::boulder::GeneratableWithPersianRug<#context>>::Generator
                                },
                            )
                        } else {
                            let mut value = pm2::TokenStream::new();

                            value.extend(quote::quote! {
                                <#element_type as ::boulder::Generatable>::generator()
                            });

                            for (k, v) in map {
                                value.extend(quote::quote! {
                                    .#k(#v)
                                });
                            }

                            (
                                quote::quote! {
                                    ::boulder::persian_rug::generator::GeneratorWrapper::new(#value)
                                },
                                quote::quote! {
                                    ::boulder::persian_rug::generator::GeneratorWrapper<#element_type>
                                },
                            )
                        }
                    }
                    GeneratorType::Default => {
                        let mut static_value = pm2::TokenStream::new();
                        match builder {
                            BuildType::Buildable(map) => {
                                if builder_needs_context {
                                    let mut init = pm2::TokenStream::new();
                                    init.extend(quote::quote! {
                                        <#element_type as ::boulder::BuildableWithPersianRug<#context>>::builder()
                                    });
                                    for (k, v) in map {
                                        init.extend(quote::quote! {
                                            .#k(#v)
                                        });
                                    }
                                    static_value.extend(quote::quote! {
                                        <<#element_type as ::boulder::BuildableWithPersianRug<#context>>::Builder as ::boulder::BuilderWithPersianRug<#context>>::build(#init, context)
                                    });
                                } else {
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
                            }
                            BuildType::Value { expr: value, .. } => {
                                if builder_needs_context {
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

                        if builder_needs_context {
                            let new_generator_id = make_generator_id_for_field(field);

                            dyn_generators.extend(quote::quote! {
                                // This is just so the generics are used and in scope for #fieldtype
                                struct #new_generator_id #generics #wc {
                                    _marker: core::marker::PhantomData<#ident #ty_generics>
                                }

                                impl #generics GeneratorWithPersianRug<#context> for #new_generator_id #ty_generics #wc
                                {
                                    type Output = #element_type;
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

                            (
                                quote::quote! {
                                    #new_generator_id { _marker: Default::default() }
                                },
                                quote::quote! {
                                    #new_generator_id #ty_generics
                                },
                            )
                        } else {
                            (
                                quote::quote! {
                                    ::boulder::persian_rug::generator::GeneratorWrapper::new(|| { #static_value })
                                },
                                quote::quote! {
                                    ::boulder::persian_rug::generator::GeneratorWrapper<#element_type>
                                },
                            )
                        }
                    }
                };
                if let Some((sequence, seq_ty)) = sequence {
                    if sequence_needs_context {
                        if seq_ty.is_none() {
                            return syn::Error::new_spanned(
                                sequence,
                                "a type must be provided to use this value in GeneratableWithPersianRug (syntax: `value: type`)"
                            ).to_compile_error();
                        }
                        default_values.extend(quote::quote! {
                            #fieldid: ::boulder::persian_rug::generator::SequenceGeneratorWithPersianRug::new(#sequence, #value),
                        });
                        default_types.extend(quote::quote! {
                            , ::boulder::persian_rug::generator::SequenceGeneratorWithPersianRug<#seq_ty, #value_type, #fieldtype>
                        });
                    } else {
                        default_values.extend(quote::quote! {
                            #fieldid: ::boulder::persian_rug::generator::SequenceGeneratorWithPersianRug::new(::boulder::persian_rug::generator::GeneratorWrapper::new(#sequence), #value),
                        });
                        default_types.extend(quote::quote! {
                            , ::boulder::persian_rug::generator::SequenceGeneratorWithPersianRug<::boulder::persian_rug::generator::GeneratorWrapper<usize>, #value_type, #fieldtype>
                        });
                    }
                } else if let Some((sequence, _seq_ty)) = build_sequence {
                    if build_sequence_needs_context {
                        let new_generator_id = make_sequence_generator_id_for_field(field);
                        dyn_generators.extend(quote::quote! {
                            // This is just so the generics are used and in scope for #fieldtype
                            struct #new_generator_id #generics #wc {
                                _marker: core::marker::PhantomData<#ident #ty_generics>
                            }

                            impl #generics GeneratorWithPersianRug<#context> for #new_generator_id #ty_generics #wc
                            {
                                type Output = usize;
                                fn generate<'boulder_mutator_lifetime, BoulderMutatorParam>(
                                    &mut self, mut context: BoulderMutatorParam
                                ) -> (Self::Output, BoulderMutatorParam)
                                where
                                    BoulderMutatorParam: 'boulder_mutator_lifetime + ::persian_rug::Mutator<Context = #context>
                                {
                                    let closure = { #sequence };
                                    closure(context).into()
                                }
                            }
                        });

                        default_values.extend(quote::quote! {
                            #fieldid: ::boulder::persian_rug::generator::SequenceGeneratorWithPersianRug::new(
                                #new_generator_id { _marker: Default::default() },
                                #value),
                        });
                        default_types.extend(quote::quote! {
                            , ::boulder::persian_rug::generator::SequenceGeneratorWithPersianRug<#new_generator_id #ty_generics, #value_type, #fieldtype>
                        });
                    } else {
                        default_values.extend(quote::quote! {
                            #fieldid: ::boulder::persian_rug::generator::SequenceGeneratorWithPersianRug::new(::boulder::persian_rug::generator::GeneratorWrapper::new(|| (#sequence).into()), #value),
                        });
                        default_types.extend(quote::quote! {
                            , ::boulder::persian_rug::generator::SequenceGeneratorWithPersianRug<::boulder::persian_rug::generator::GeneratorWrapper<usize>, #value_type, #fieldtype>
                        });
                    }
                } else {
                    default_values.extend(quote::quote! {
                        #fieldid: #value,
                    });
                    default_types.extend(quote::quote! {
                        , #value_type
                    });
                }
            }
        }
    }

    let (gen_generics, gen_ty_generics, gen_wc) = gen_generics.split_for_impl();

    let res = quote::quote! {
        const _: () = {
            #vis struct Generator<BoulderTypeMarkerParam #bare_generics #added_generics> #wc {
                _boulder_type_marker: ::core::marker::PhantomData<BoulderTypeMarkerParam>,
                _boulder_type_storage: ::core::marker::PhantomData<#ident #ty_generics>,
                #body
            }

            #vis trait NestedGenerate #gen_generics
            {
                type Output;
                fn nested_generate<'boulder_mutator_lifetime, BoulderMutatorParam, BoulderFunctionParam>(
                    gen: &mut Generator<BoulderFunctionParam #bare_ty_generics #added_generics>,
                    context: BoulderMutatorParam)
                    -> (Self::Output, BoulderMutatorParam)
                where
                    BoulderMutatorParam: 'boulder_mutator_lifetime + ::persian_rug::Mutator<Context=#context>;
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl<BoulderTypeMarkerParam #bare_generics> Generator <BoulderTypeMarkerParam #bare_ty_generics #default_types> {
                pub fn new() -> Self
                {
                    Self {
                        _boulder_type_marker: Default::default(),
                        _boulder_type_storage: Default::default(),
                        #default_values
                    }
                }
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl<BoulderTypeMarkerParam #bare_generics #added_generics> Generator <BoulderTypeMarkerParam #bare_ty_generics #added_generics> #gen_wc
            {
                #methods
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl<BoulderTypeMarkerParam #bare_generics #added_generics> ::boulder::persian_rug::generator::guts::MiniGeneratorWithPersianRug<#context> for Generator<BoulderTypeMarkerParam #bare_ty_generics #added_generics>

            where
                Self: NestedGenerate #gen_ty_generics,
                #added_wc
                #bare_wc
            {
                type Output = <Self as NestedGenerate #gen_ty_generics>::Output;
                fn generate<'boulder_mutator_lifetime, BoulderMutatorParam>(
                    &mut self,
                    context: BoulderMutatorParam)
                    -> (Self::Output, BoulderMutatorParam)
                where
                    BoulderMutatorParam: 'boulder_mutator_lifetime + ::persian_rug::Mutator<Context=#context>,
                {
                    <Self as NestedGenerate #gen_ty_generics>::nested_generate(self, context)
                }
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl #generics ::boulder::persian_rug::generator::guts::BoulderBase for #ident #ty_generics #wc {
                type Base = #ident #ty_generics;
            }

            #dyn_generators

            // Base case

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl #generics ::boulder::persian_rug::generator::guts::MiniGeneratableWithPersianRug<#ident #ty_generics, #context> for #ident #ty_generics #wc

            {
                type Generator = Generator<#ident #ty_generics #bare_ty_generics #default_types>;
                fn mini_generator() -> Self::Generator {
                    Generator::new()
                }
             }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl #gen_generics NestedGenerate #gen_ty_generics for Generator<#ident #ty_generics #bare_ty_generics #added_generics> #gen_wc
            {
                type Output = #ident #ty_generics;
                fn nested_generate<'boulder_lifetime_param, BoulderMutatorParam, BoulderFunctionParam>(
                    gen: &mut Generator<BoulderFunctionParam #bare_ty_generics #added_generics>,
                    mut context: BoulderMutatorParam
                ) -> (Self::Output, BoulderMutatorParam)
                where
                    BoulderMutatorParam: 'boulder_lifetime_param + ::persian_rug::Mutator<Context=#context>
                {
                    #vars
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
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::generator::guts::MiniGeneratableWithPersianRug<#ident #ty_generics, #context> for Option<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::persian_rug::generator::guts::MiniGeneratableWithPersianRug<#ident #ty_generics, #context>,
                Generator<Option<BoulderExtraGenericParam> #bare_ty_generics #default_types>: ::boulder::persian_rug::generator::guts::MiniGeneratorWithPersianRug<#context, Output=Option<BoulderExtraGenericParam>>,
                #bare_wc
            {
                type Generator = Generator<Option<BoulderExtraGenericParam> #bare_ty_generics #default_types>;
                fn mini_generator() -> Self::Generator {
                    Generator::new()
                }
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_ty_generics #added_generics> NestedGenerate #gen_ty_generics for Generator<Option<BoulderExtraGenericParam> #bare_ty_generics #added_generics>
            where
                Generator<BoulderExtraGenericParam #bare_ty_generics #added_generics>: NestedGenerate #gen_ty_generics,
            #added_wc
            #bare_wc

            {
                type Output = Option<<Generator<BoulderExtraGenericParam #bare_ty_generics #added_generics> as NestedGenerate #gen_ty_generics>::Output>;
                fn nested_generate<'boulder_lifetime_param, BoulderMutatorParam, BoulderFunctionParam>(gen: &mut Generator<BoulderFunctionParam #bare_ty_generics #added_generics>, mut context: BoulderMutatorParam) -> (Self::Output, BoulderMutatorParam)
                where
                    BoulderMutatorParam: 'boulder_lifetime_param + ::persian_rug::Mutator<Context = #context>
                {
                    let (result, mut context) = Generator::<BoulderExtraGenericParam #bare_ty_generics #added_generics>::nested_generate(gen, context);
                    (Some(result), context)
                }
            }

            // Proxy
            #[automatically_derived]
            #[persian_rug::constraints(#constraints, access(BoulderExtraGenericParam))]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::generator::guts::MiniGeneratableWithPersianRug<#ident #ty_generics, #context> for ::persian_rug::Proxy<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::persian_rug::generator::guts::MiniGeneratableWithPersianRug<#ident #ty_generics, #context>,
                Generator<::persian_rug::Proxy<BoulderExtraGenericParam> #bare_ty_generics #default_types>: ::boulder::persian_rug::generator::guts::MiniGeneratorWithPersianRug<#context, Output=::persian_rug::Proxy<BoulderExtraGenericParam>>,

                #bare_wc
            {
                type Generator = Generator<::persian_rug::Proxy<BoulderExtraGenericParam> #bare_ty_generics #default_types>;
                fn mini_generator() -> Self::Generator {
                    Generator::new()
                }
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints, access(<Generator::<BoulderExtraGenericParam #bare_ty_generics #added_generics> as NestedGenerate #gen_ty_generics>::Output))]
            impl <BoulderExtraGenericParam #bare_ty_generics #added_generics> NestedGenerate #gen_ty_generics for Generator<::persian_rug::Proxy<BoulderExtraGenericParam> #bare_ty_generics #added_generics>
            where
                Generator<BoulderExtraGenericParam #bare_ty_generics #added_generics>: NestedGenerate #gen_ty_generics,
            #added_wc
            #bare_wc

            {
                type Output = ::persian_rug::Proxy<<Generator<BoulderExtraGenericParam #bare_ty_generics #added_generics> as NestedGenerate #gen_ty_generics>::Output>;
                fn nested_generate<'boulder_lifetime_param, BoulderMutatorParam, BoulderFunctionParam>(gen: &mut Generator<BoulderFunctionParam #bare_ty_generics #added_generics>, mut context: BoulderMutatorParam) -> (Self::Output, BoulderMutatorParam)
                where
                    BoulderMutatorParam: 'boulder_lifetime_param + ::persian_rug::Mutator<Context = #context>,
                {
                    let (result, mut context) = Generator::<BoulderExtraGenericParam #bare_ty_generics #added_generics>::nested_generate(gen, context);
                    (context.add(result), context)
                }
            }

            // Arc
            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::generator::guts::MiniGeneratableWithPersianRug<#ident #ty_generics, #context> for ::std::sync::Arc<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::persian_rug::generator::guts::MiniGeneratableWithPersianRug<#ident #ty_generics, #context>,
                Generator<::std::sync::Arc<BoulderExtraGenericParam> #bare_ty_generics #default_types>: ::boulder::persian_rug::generator::guts::MiniGeneratorWithPersianRug<#context, Output=::std::sync::Arc<BoulderExtraGenericParam>>,
                #bare_wc
            {
                type Generator = Generator<::std::sync::Arc<BoulderExtraGenericParam> #bare_ty_generics #default_types>;
                fn mini_generator() -> Self::Generator {
                    Generator::new()
                }
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_ty_generics #added_generics> NestedGenerate #gen_ty_generics for Generator<::std::sync::Arc<BoulderExtraGenericParam> #bare_ty_generics #added_generics>
            where
                Generator<BoulderExtraGenericParam #bare_ty_generics #added_generics>: NestedGenerate #gen_ty_generics,
            #added_wc
            #bare_wc

            {
                type Output = ::std::sync::Arc<<Generator<BoulderExtraGenericParam #bare_ty_generics #added_generics> as NestedGenerate #gen_ty_generics>::Output>;
                fn nested_generate<'boulder_lifetime_param, BoulderMutatorParam, BoulderFunctionParam>(gen: &mut Generator<BoulderFunctionParam #bare_ty_generics #added_generics>, mut context: BoulderMutatorParam) -> (Self::Output, BoulderMutatorParam)
                where
                    BoulderMutatorParam: 'boulder_lifetime_param + ::persian_rug::Mutator<Context = #context>
                {
                    let (result, mut context) = Generator::<BoulderExtraGenericParam #bare_ty_generics #added_generics>::nested_generate(gen, context);
                    (::std::sync::Arc::new(result), context)
                }
            }

            // Rc
            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::generator::guts::MiniGeneratableWithPersianRug<#ident #ty_generics, #context> for ::std::rc::Rc<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::persian_rug::generator::guts::MiniGeneratableWithPersianRug<#ident #ty_generics, #context>,
                Generator<::std::rc::Rc<BoulderExtraGenericParam> #bare_ty_generics #default_types>: ::boulder::persian_rug::generator::guts::MiniGeneratorWithPersianRug<#context, Output=::std::rc::Rc<BoulderExtraGenericParam>>,
                #bare_wc
            {
                type Generator = Generator<::std::rc::Rc<BoulderExtraGenericParam> #bare_ty_generics #default_types>;
                fn mini_generator() -> Self::Generator {
                    Generator::new()
                }
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_ty_generics #added_generics> NestedGenerate #gen_ty_generics for Generator<::std::rc::Rc<BoulderExtraGenericParam> #bare_ty_generics #added_generics>
            where
                Generator<BoulderExtraGenericParam #bare_ty_generics #added_generics>: NestedGenerate #gen_ty_generics,
            #added_wc
            #bare_wc

            {
                type Output = ::std::rc::Rc<<Generator<BoulderExtraGenericParam #bare_ty_generics #added_generics> as NestedGenerate #gen_ty_generics>::Output>;
                fn nested_generate<'boulder_lifetime_param, BoulderMutatorParam, BoulderFunctionParam>(gen: &mut Generator<BoulderFunctionParam #bare_ty_generics #added_generics>, mut context: BoulderMutatorParam) -> (Self::Output, BoulderMutatorParam)
                where
                    BoulderMutatorParam: 'boulder_lifetime_param + ::persian_rug::Mutator<Context = #context>
                {
                    let (result, mut context) = Generator::<BoulderExtraGenericParam #bare_ty_generics #added_generics>::nested_generate(gen, context);
                    (::std::rc::Rc::new(result), context)
                }
            }

            // Cell
            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::generator::guts::MiniGeneratableWithPersianRug<#ident #ty_generics, #context> for ::std::cell::Cell<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::persian_rug::generator::guts::MiniGeneratableWithPersianRug<#ident #ty_generics, #context>,
                Generator<::std::cell::Cell<BoulderExtraGenericParam> #bare_ty_generics #default_types>: ::boulder::persian_rug::generator::guts::MiniGeneratorWithPersianRug<#context, Output=::std::cell::Cell<BoulderExtraGenericParam>>,
                #bare_wc
            {
                type Generator = Generator<::std::cell::Cell<BoulderExtraGenericParam> #bare_ty_generics #default_types>;
                fn mini_generator() -> Self::Generator {
                    Generator::new()
                }
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_ty_generics #added_generics> NestedGenerate #gen_ty_generics for Generator<::std::cell::Cell<BoulderExtraGenericParam> #bare_ty_generics #added_generics>
            where
                Generator<BoulderExtraGenericParam #bare_ty_generics #added_generics>: NestedGenerate #gen_ty_generics,
            #added_wc
            #bare_wc

            {
                type Output = ::std::cell::Cell<<Generator<BoulderExtraGenericParam #bare_ty_generics #added_generics> as NestedGenerate #gen_ty_generics>::Output>;
                fn nested_generate<'boulder_lifetime_param, BoulderMutatorParam, BoulderFunctionParam>(gen: &mut Generator<BoulderFunctionParam #bare_ty_generics #added_generics>, mut context: BoulderMutatorParam) -> (Self::Output, BoulderMutatorParam)
                where
                    BoulderMutatorParam: 'boulder_lifetime_param + ::persian_rug::Mutator<Context = #context>
                {
                    let (result, mut context) = Generator::<BoulderExtraGenericParam #bare_ty_generics #added_generics>::nested_generate(gen, context);
                    (::std::cell::Cell::new(result), context)
                }
            }

            // RefCell
            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::generator::guts::MiniGeneratableWithPersianRug<#ident #ty_generics, #context> for ::std::cell::RefCell<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::persian_rug::generator::guts::MiniGeneratableWithPersianRug<#ident #ty_generics, #context>,
                Generator<::std::cell::RefCell<BoulderExtraGenericParam> #bare_ty_generics #default_types>: ::boulder::persian_rug::generator::guts::MiniGeneratorWithPersianRug<#context, Output=::std::cell::RefCell<BoulderExtraGenericParam>>,
                #bare_wc
            {
                type Generator = Generator<::std::cell::RefCell<BoulderExtraGenericParam> #bare_ty_generics #default_types>;
                fn mini_generator() -> Self::Generator {
                    Generator::new()
                }
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_ty_generics #added_generics> NestedGenerate #gen_ty_generics for Generator<::std::cell::RefCell<BoulderExtraGenericParam> #bare_ty_generics #added_generics>
            where
                Generator<BoulderExtraGenericParam #bare_ty_generics #added_generics>: NestedGenerate #gen_ty_generics,
            #added_wc
            #bare_wc

            {
                type Output = ::std::cell::RefCell<<Generator<BoulderExtraGenericParam #bare_ty_generics #added_generics> as NestedGenerate #gen_ty_generics>::Output>;
                fn nested_generate<'boulder_lifetime_param, BoulderMutatorParam, BoulderFunctionParam>(gen: &mut Generator<BoulderFunctionParam #bare_ty_generics #added_generics>, mut context: BoulderMutatorParam) -> (Self::Output, BoulderMutatorParam)
                where
                    BoulderMutatorParam: 'boulder_lifetime_param + ::persian_rug::Mutator<Context = #context>
                {
                    let (result, mut context) = Generator::<BoulderExtraGenericParam #bare_ty_generics #added_generics>::nested_generate(gen, context);
                    (::std::cell::RefCell::new(result), context)
                }
            }

            // Mutex
            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_generics> ::boulder::persian_rug::generator::guts::MiniGeneratableWithPersianRug<#ident #ty_generics, #context> for ::std::sync::Mutex<BoulderExtraGenericParam>
            where
                BoulderExtraGenericParam: ::boulder::persian_rug::generator::guts::MiniGeneratableWithPersianRug<#ident #ty_generics, #context>,
                Generator<::std::sync::Mutex<BoulderExtraGenericParam> #bare_ty_generics #default_types>: ::boulder::persian_rug::generator::guts::MiniGeneratorWithPersianRug<#context, Output=::std::sync::Mutex<BoulderExtraGenericParam>>,
                #bare_wc
            {
                type Generator = Generator<::std::sync::Mutex<BoulderExtraGenericParam> #bare_ty_generics #default_types>;
                fn mini_generator() -> Self::Generator {
                    Generator::new()
                }
            }

            #[automatically_derived]
            #[persian_rug::constraints(#constraints)]
            impl <BoulderExtraGenericParam #bare_ty_generics #added_generics> NestedGenerate #gen_ty_generics for Generator<::std::sync::Mutex<BoulderExtraGenericParam> #bare_ty_generics #added_generics>
            where
                Generator<BoulderExtraGenericParam #bare_ty_generics #added_generics>: NestedGenerate #gen_ty_generics,
            #added_wc
            #bare_wc

            {
                type Output = ::std::sync::Mutex<<Generator<BoulderExtraGenericParam #bare_ty_generics #added_generics> as NestedGenerate #gen_ty_generics>::Output>;
                fn nested_generate<'boulder_lifetime_param, BoulderMutatorParam, BoulderFunctionParam>(gen: &mut Generator<BoulderFunctionParam #bare_ty_generics #added_generics>, mut context: BoulderMutatorParam) -> (Self::Output, BoulderMutatorParam)
                where
                    BoulderMutatorParam: 'boulder_lifetime_param + ::persian_rug::Mutator<Context = #context>
                {
                    let (result, mut context) = Generator::<BoulderExtraGenericParam #bare_ty_generics #added_generics>::nested_generate(gen, context);
                    (::std::sync::Mutex::new(result), context)
                }
            }

        };
    };

    res
}
