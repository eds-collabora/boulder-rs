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
    Buildable(BTreeMap<syn::Ident, syn::Expr>),
    BuildableWithContext(BTreeMap<syn::Ident, syn::Expr>),
    Default(syn::Expr),
    Sequence(syn::Expr),
    Generatable(BTreeMap<syn::Ident, syn::Expr>),
    GeneratableWithContext(BTreeMap<syn::Ident, syn::Expr>),
    Generator(syn::Expr),
    GeneratorWithContext(syn::Expr),
    SequenceGenerator(syn::Expr),
}

impl syn::parse::Parse for AttributeItem {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let attr: syn::Ident = input.parse()?;
        match attr.to_string().as_str() {
            //FIXME: DefaultWithContext is a plausible thing
            "default" => {
                let _: syn::Token![=] = input.parse()?;
                let value: syn::Expr = input.parse()?;
                Ok(AttributeItem::Default(value))
            }
            "buildable" | "buildable_with_context" => {
                let contents = if input.lookahead1().peek(syn::token::Paren) {
                    let content;
                    let _: syn::token::Paren = syn::parenthesized!(content in input);
                    let punc = syn::punctuated::Punctuated::<AttributeValue, syn::Token![,]>::parse_terminated(&content)?;
                    BTreeMap::from_iter(punc.into_iter().map(|x| (x.name, x.value)))
                } else {
                    BTreeMap::new()
                };
                if attr.to_string().ends_with("with_context") {
                    Ok(AttributeItem::BuildableWithContext(contents))
                } else {
                    Ok(AttributeItem::Buildable(contents))
                }
            }
            "sequence" => {
                let _: syn::Token![=] = input.parse()?;
                let value: syn::Expr = input.parse()?;
                Ok(AttributeItem::Sequence(value))
            }
            "generator" | "generator_with_context" => {
                let _: syn::Token![=] = input.parse()?;
                let value: syn::Expr = input.parse()?;
                if attr.to_string().ends_with("with_context") {
                    Ok(AttributeItem::GeneratorWithContext(value))
                } else {
                    Ok(AttributeItem::Generator(value))
                }
            }
            "generatable" | "generatable_with_context" => {
                let contents = if input.lookahead1().peek(syn::token::Paren) {
                    let content;
                    let _: syn::token::Paren = syn::parenthesized!(content in input);
                    let punc = syn::punctuated::Punctuated::<AttributeValue, syn::Token![,]>::parse_terminated(&content)?;
                    BTreeMap::from_iter(punc.into_iter().map(|x| (x.name, x.value)))
                } else {
                    BTreeMap::new()
                };
                if attr.to_string().ends_with("with_context") {
                    Ok(AttributeItem::GeneratableWithContext(contents))
                } else {
                    Ok(AttributeItem::Generatable(contents))
                }
            }
            "sequence_generator" => {
                let _: syn::Token![=] = input.parse()?;
                let value: syn::Expr = input.parse()?;
                Ok(AttributeItem::SequenceGenerator(value))
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
    Value(Box<syn::Expr>),
    Default,
}

pub struct BuilderData {
    pub element: BuildType,
    pub sequence: Option<syn::Expr>,
    pub needs_context: bool,
}

pub enum GeneratorType {
    Generator(Box<syn::Expr>),
    Generatable(BTreeMap<syn::Ident, syn::Expr>),
    Default,
}

pub struct GeneratorData {
    pub element: GeneratorType,
    pub sequence: Option<syn::Expr>,
    pub needs_context: bool,
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
        };
        let mut gd = GeneratorData {
            element: GeneratorType::Default,
            sequence: None,
            needs_context: false,
        };

        for item in punc {
            match item {
                AttributeItem::Buildable(map) => {
                    bd.element = BuildType::Buildable(map);
                }
                AttributeItem::BuildableWithContext(map) => {
                    bd.element = BuildType::Buildable(map);
                    bd.needs_context = true;
                }
                AttributeItem::Default(expr) => {
                    bd.element = BuildType::Value(Box::new(expr));
                }
                AttributeItem::Sequence(expr) => {
                    bd.sequence = Some(expr);
                }
                AttributeItem::Generatable(map) => {
                    gd.element = GeneratorType::Generatable(map);
                }
                AttributeItem::GeneratableWithContext(map) => {
                    gd.element = GeneratorType::Generatable(map);
                    gd.needs_context = true;
                }
                AttributeItem::Generator(expr) => {
                    gd.element = GeneratorType::Generator(Box::new(expr));
                }
                AttributeItem::GeneratorWithContext(expr) => {
                    gd.element = GeneratorType::Generator(Box::new(expr));
                    gd.needs_context = true;
                }
                AttributeItem::SequenceGenerator(expr) => {
                    gd.sequence = Some(expr);
                }
            }
        }

        Ok(BuilderMeta {
            builder: bd,
            generator: gd,
        })
    }
}
