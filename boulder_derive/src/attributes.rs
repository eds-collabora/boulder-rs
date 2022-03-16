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
    Default(syn::Expr),
    Sequence(syn::Expr),
    Generatable(BTreeMap<syn::Ident, syn::Expr>),
    Generator(syn::Expr),
    SequenceGenerator(syn::Expr),
}

impl syn::parse::Parse for AttributeItem {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let attr: syn::Ident = input.parse()?;
        match attr.to_string().as_str() {
            "default" => {
                let _: syn::Token![=] = input.parse()?;
                let value: syn::Expr = input.parse()?;
                Ok(AttributeItem::Default(value))
            }
            "buildable" => {
                if input.lookahead1().peek(syn::token::Paren) {
                    let content;
                    let _: syn::token::Paren = syn::parenthesized!(content in input);
                    let punc = syn::punctuated::Punctuated::<AttributeValue, syn::Token![,]>::parse_terminated(&content)?;
                    Ok(AttributeItem::Buildable(BTreeMap::from_iter(
                        punc.into_iter().map(|x| (x.name, x.value)),
                    )))
                } else {
                    Ok(AttributeItem::Buildable(BTreeMap::new()))
                }
            }
            "sequence" => {
                let _: syn::Token![=] = input.parse()?;
                let value: syn::Expr = input.parse()?;
                Ok(AttributeItem::Sequence(value))
            }
            "generator" => {
                let _: syn::Token![=] = input.parse()?;
                let value: syn::Expr = input.parse()?;
                Ok(AttributeItem::Generator(value))
            }
            "generatable" => {
                if input.lookahead1().peek(syn::token::Paren) {
                    let content;
                    let _: syn::token::Paren = syn::parenthesized!(content in input);
                    let punc = syn::punctuated::Punctuated::<AttributeValue, syn::Token![,]>::parse_terminated(&content)?;
                    Ok(AttributeItem::Generatable(BTreeMap::from_iter(
                        punc.into_iter().map(|x| (x.name, x.value)),
                    )))
                } else {
                    Ok(AttributeItem::Generatable(BTreeMap::new()))
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
}

pub enum GeneratorType {
    Generator(Box<syn::Expr>),
    Generatable(BTreeMap<syn::Ident, syn::Expr>),
    Default,
}

pub struct GeneratorData {
    pub element: GeneratorType,
    pub sequence: Option<syn::Expr>,
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
        };
        let mut gd = GeneratorData {
            element: GeneratorType::Default,
            sequence: None,
        };

        for item in punc {
            match item {
                AttributeItem::Buildable(map) => {
                    bd.element = BuildType::Buildable(map);
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
                AttributeItem::Generator(expr) => {
                    gd.element = GeneratorType::Generator(Box::new(expr));
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
