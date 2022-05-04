use std::collections::BTreeMap;

pub struct AttributeValue {
    name: syn::Ident,
    value: syn::Expr,
}

impl syn::parse::Parse for AttributeValue {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let field: syn::Ident = input.parse()?;
        let _: syn::Token![=] = input.parse()?;
        let value: syn::Expr = input.parse()?;
        Ok(Self { name: field, value })
    }
}

pub enum AttributeItem {
    Buildable {
        buildable: syn::Ident,
        initializers: BTreeMap<syn::Ident, syn::Expr>,
    },
    Default {
        default: syn::Ident,
        expr: syn::Expr,
    },
    Sequence {
        sequence: syn::Ident,
        expr: syn::Expr,
    },
    Generatable {
        generatable: syn::Ident,
        initializers: BTreeMap<syn::Ident, syn::Expr>,
    },
    Generator {
        generator: syn::Ident,
        expr: syn::Expr,
    },
    SequenceGenerator {
        sequence_generator: syn::Ident,
        expr: syn::Expr,
    },

    BuildableWithPersianRug {
        buildable_with_persian_rug: syn::Ident,
        initializers: BTreeMap<syn::Ident, syn::Expr>,
    },
    DefaultWithPersianRug {
        default_with_persian_rug: syn::Ident,
        expr: syn::Expr,
        ty: Option<syn::Type>,
    },
    SequenceWithPersianRug {
        sequence_with_persian_rug: syn::Ident,
        expr: syn::Expr,
        ty: Option<syn::Type>,
    },
    GeneratableWithPersianRug {
        generatable_with_persian_rug: syn::Ident,
        initializers: BTreeMap<syn::Ident, syn::Expr>,
    },
    GeneratorWithPersianRug {
        generator_with_persian_rug: syn::Ident,
        expr: syn::Expr,
        ty: Option<syn::Type>,
    },
    SequenceGeneratorWithPersianRug {
        sequence_generator_with_persian_rug: syn::Ident,
        expr: syn::Expr,
        ty: Option<syn::Type>,
    },
}

impl syn::parse::Parse for AttributeItem {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let attr: syn::Ident = input.parse()?;
        match attr.to_string().as_str() {
            "buildable" | "buildable_with_persian_rug" => {
                let initializers = if input.lookahead1().peek(syn::token::Paren) {
                    let content;
                    let _: syn::token::Paren = syn::parenthesized!(content in input);
                    let punc = syn::punctuated::Punctuated::<AttributeValue, syn::Token![,]>::parse_terminated(&content)?;
                    BTreeMap::from_iter(punc.into_iter().map(|x| (x.name, x.value)))
                } else {
                    BTreeMap::new()
                };
                if attr.to_string().ends_with("with_persian_rug") {
                    Ok(AttributeItem::BuildableWithPersianRug {
                        buildable_with_persian_rug: attr,
                        initializers,
                    })
                } else {
                    Ok(AttributeItem::Buildable {
                        buildable: attr,
                        initializers,
                    })
                }
            }
            "default" => {
                let _: syn::Token![=] = input.parse()?;
                let value: syn::Expr = input.parse()?;
                Ok(AttributeItem::Default {
                    default: attr,
                    expr: value,
                })
            }
            "default_with_persian_rug" => {
                let _: syn::Token![=] = input.parse()?;
                let value: syn::Expr = input.parse()?;
                let (value, ty) = if let syn::Expr::Type(ty) = value {
                    (*ty.expr, Some(*ty.ty))
                } else {
                    (value, None)
                };
                Ok(AttributeItem::DefaultWithPersianRug {
                    default_with_persian_rug: attr,
                    expr: value,
                    ty,
                })
            }
            "sequence" => {
                let _: syn::Token![=] = input.parse()?;
                let value: syn::Expr = input.parse()?;
                Ok(AttributeItem::Sequence {
                    sequence: attr,
                    expr: value,
                })
            }
            "sequence_with_persian_rug" => {
                let _: syn::Token![=] = input.parse()?;
                let value: syn::Expr = input.parse()?;
                let (value, ty) = if let syn::Expr::Type(ty) = value {
                    (*ty.expr, Some(*ty.ty))
                } else {
                    (value, None)
                };
                Ok(AttributeItem::SequenceWithPersianRug {
                    sequence_with_persian_rug: attr,
                    expr: value,
                    ty,
                })
            }
            "generatable" | "generatable_with_persian_rug" => {
                let initializers = if input.lookahead1().peek(syn::token::Paren) {
                    let content;
                    let _: syn::token::Paren = syn::parenthesized!(content in input);
                    let punc = syn::punctuated::Punctuated::<AttributeValue, syn::Token![,]>::parse_terminated(&content)?;
                    BTreeMap::from_iter(punc.into_iter().map(|x| (x.name, x.value)))
                } else {
                    BTreeMap::new()
                };
                if attr.to_string().ends_with("with_persian_rug") {
                    Ok(AttributeItem::GeneratableWithPersianRug {
                        generatable_with_persian_rug: attr,
                        initializers,
                    })
                } else {
                    Ok(AttributeItem::Generatable {
                        generatable: attr,
                        initializers,
                    })
                }
            }
            "generator" => {
                let _: syn::Token![=] = input.parse()?;
                let value: syn::Expr = input.parse()?;
                Ok(AttributeItem::Generator {
                    generator: attr,
                    expr: value,
                })
            }
            "generator_with_persian_rug" => {
                let _: syn::Token![=] = input.parse()?;
                let value: syn::Expr = input.parse()?;
                let (value, ty) = if let syn::Expr::Type(ty) = value {
                    (*ty.expr, Some(*ty.ty))
                } else {
                    (value, None)
                };
                Ok(AttributeItem::GeneratorWithPersianRug {
                    generator_with_persian_rug: attr,
                    expr: value,
                    ty,
                })
            }
            "sequence_generator" => {
                let _: syn::Token![=] = input.parse()?;
                let value: syn::Expr = input.parse()?;
                Ok(AttributeItem::SequenceGenerator {
                    sequence_generator: attr,
                    expr: value,
                })
            }
            "sequence_generator_with_persian_rug" => {
                let _: syn::Token![=] = input.parse()?;
                let value: syn::Expr = input.parse()?;
                let (value, ty) = if let syn::Expr::Type(ty) = value {
                    (*ty.expr, Some(*ty.ty))
                } else {
                    (value, None)
                };
                Ok(AttributeItem::SequenceGeneratorWithPersianRug {
                    sequence_generator_with_persian_rug: attr,
                    expr: value,
                    ty,
                })
            }
            _ => Err(syn::Error::new_spanned(
                attr,
                "unsupported boulder attribute",
            )),
        }
    }
}

pub enum BuildType {
    Buildable(BTreeMap<syn::Ident, syn::Expr>),
    Value {
        expr: syn::Expr,
        ty: Option<syn::Type>,
    },
    Default,
}

pub struct BuilderData {
    pub element: BuildType,
    pub sequence: Option<(syn::Expr, Option<syn::Type>)>,
    pub needs_context: bool,
    pub sequence_needs_context: bool,
}

pub enum GeneratorType {
    Generator {
        expr: syn::Expr,
        ty: Option<syn::Type>,
    },
    Generatable(BTreeMap<syn::Ident, syn::Expr>),
    Default,
}

pub struct GeneratorData {
    pub element: GeneratorType,
    pub sequence: Option<(syn::Expr, Option<syn::Type>)>,
    pub needs_context: bool,
    pub sequence_needs_context: bool,
}

pub struct BuilderMeta {
    pub builder: BuilderData,
    pub generator: GeneratorData,
}

impl syn::parse::Parse for BuilderMeta {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let punc =
            syn::punctuated::Punctuated::<AttributeItem, syn::Token![,]>::parse_terminated(input)?;
        let mut bd = BuilderData {
            element: BuildType::Default,
            sequence: None,
            needs_context: false,
            sequence_needs_context: false,
        };
        let mut gd = GeneratorData {
            element: GeneratorType::Default,
            sequence: None,
            needs_context: false,
            sequence_needs_context: false,
        };

        for ref item in punc {
            match item {
                AttributeItem::Buildable {
                    initializers: map, ..
                } => {
                    bd.element = BuildType::Buildable(map.clone());
                }
                AttributeItem::Default { expr, .. } => {
                    bd.element = BuildType::Value {
                        expr: expr.clone(),
                        ty: None,
                    };
                }
                AttributeItem::Sequence { expr, .. } => {
                    bd.sequence = Some((expr.clone(), None));
                }
                AttributeItem::Generatable {
                    initializers: map, ..
                } => {
                    gd.element = GeneratorType::Generatable(map.clone());
                }
                AttributeItem::Generator { expr, .. } => {
                    gd.element = GeneratorType::Generator {
                        expr: expr.clone(),
                        ty: None,
                    };
                }
                AttributeItem::SequenceGenerator { expr, .. } => {
                    gd.sequence = Some((expr.clone(), None));
                }
                AttributeItem::BuildableWithPersianRug {
                    buildable_with_persian_rug: ident,
                    ..
                }
                | AttributeItem::DefaultWithPersianRug {
                    default_with_persian_rug: ident,
                    ..
                }
                | AttributeItem::SequenceWithPersianRug {
                    sequence_with_persian_rug: ident,
                    ..
                }
                | AttributeItem::GeneratableWithPersianRug {
                    generatable_with_persian_rug: ident,
                    ..
                }
                | AttributeItem::GeneratorWithPersianRug {
                    generator_with_persian_rug: ident,
                    ..
                }
                | AttributeItem::SequenceGeneratorWithPersianRug {
                    sequence_generator_with_persian_rug: ident,
                    ..
                } => {
                    return Err(syn::Error::new_spanned(
                        ident,
                        format!(
                            "Cannot use {} outside of a persian-rug enabled derive.",
                            ident
                        ),
                    ))
                }
            }
        }

        Ok(BuilderMeta {
            builder: bd,
            generator: gd,
        })
    }
}

pub struct BuilderMetaWithPersianRug {
    pub builder: BuilderData,
    pub generator: GeneratorData,
}

impl syn::parse::Parse for BuilderMetaWithPersianRug {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let punc =
            syn::punctuated::Punctuated::<AttributeItem, syn::Token![,]>::parse_terminated(input)?;
        let mut bd = BuilderData {
            element: BuildType::Default,
            sequence: None,
            needs_context: false,
            sequence_needs_context: false,
        };
        let mut gd = GeneratorData {
            element: GeneratorType::Default,
            sequence: None,
            needs_context: false,
            sequence_needs_context: false,
        };

        for item in punc {
            match item {
                AttributeItem::Buildable {
                    initializers: map, ..
                } => {
                    bd.element = BuildType::Buildable(map.clone());
                }
                AttributeItem::Default { expr, .. } => {
                    bd.element = BuildType::Value {
                        expr: expr.clone(),
                        ty: None,
                    };
                }
                AttributeItem::Sequence { expr, .. } => {
                    bd.sequence = Some((expr.clone(), None));
                }
                AttributeItem::Generatable {
                    initializers: map, ..
                } => {
                    gd.element = GeneratorType::Generatable(map.clone());
                }
                AttributeItem::Generator { expr, .. } => {
                    gd.element = GeneratorType::Generator {
                        expr: expr.clone(),
                        ty: None,
                    };
                }
                AttributeItem::SequenceGenerator { expr, .. } => {
                    gd.sequence = Some((expr.clone(), None));
                }

                AttributeItem::BuildableWithPersianRug {
                    initializers: map, ..
                } => {
                    bd.element = BuildType::Buildable(map.clone());
                    bd.needs_context = true;
                }
                AttributeItem::DefaultWithPersianRug { expr, ty, .. } => {
                    bd.element = BuildType::Value {
                        expr: expr.clone(),
                        ty: ty.clone(),
                    };
                    bd.needs_context = true;
                }
                AttributeItem::SequenceWithPersianRug { expr, ty, .. } => {
                    bd.sequence = Some((expr.clone(), ty.clone()));
                    bd.sequence_needs_context = true;
                }
                AttributeItem::GeneratableWithPersianRug {
                    initializers: map, ..
                } => {
                    gd.element = GeneratorType::Generatable(map.clone());
                    gd.needs_context = true;
                }
                AttributeItem::GeneratorWithPersianRug { expr, ty, .. } => {
                    gd.element = GeneratorType::Generator {
                        expr: expr.clone(),
                        ty: ty.clone(),
                    };
                    gd.needs_context = true;
                }
                AttributeItem::SequenceGeneratorWithPersianRug { expr, ty, .. } => {
                    gd.sequence = Some((expr.clone(), ty.clone()));
                    gd.sequence_needs_context = true;
                }
            }
        }

        Ok(BuilderMetaWithPersianRug {
            builder: bd,
            generator: gd,
        })
    }
}
