use proc_macro2 as pm2;

pub(crate) struct RepeatCall {
    items: Vec<syn::Expr>,
}

impl syn::parse::Parse for RepeatCall {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let punc =
            syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated(input)?;
        Ok(Self {
            items: punc.into_iter().collect(),
        })
    }
}

pub(crate) fn repeat_macro(call: RepeatCall) -> pm2::TokenStream {
    let mut items = pm2::TokenStream::new();

    for item in call.items {
        items.extend(quote::quote! {
            items.push(#item.into());
        });
    }

    let res: pm2::TokenStream = quote::quote! {
        {
            let mut items = Vec::new();
            #items

            ::boulder::gen::Repeat::new(items)
        }
    };

    res
}
