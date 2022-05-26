use proc_macro2 as pm2;

pub(crate) struct StringPatternCall {
    pattern: syn::LitStr,
    args: Vec<syn::Expr>,
}

impl syn::parse::Parse for StringPatternCall {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let pattern: syn::LitStr = input.parse()?;
        if input.lookahead1().peek(syn::Token![,]) {
            let _: syn::Token![,] = input.parse()?;
            let punc =
                syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated(input)?;
            Ok(Self {
                pattern,
                args: punc.into_iter().collect(),
            })
        } else {
            Ok(Self {
                pattern,
                args: Vec::new(),
            })
        }
    }
}

pub(crate) fn pattern_macro(call: StringPatternCall) -> pm2::TokenStream {
    let mut generics = pm2::TokenStream::new();
    let mut wc = pm2::TokenStream::new();
    let mut sbody = pm2::TokenStream::new();
    let mut fcall = pm2::TokenStream::new();
    let mut args = pm2::TokenStream::new();

    for (count, arg) in call.args.iter().enumerate() {
        let typename = syn::Ident::new(&format!("T{}", count), pm2::Span::call_site());
        generics.extend(quote::quote! {
            #typename,
        });
        wc.extend(quote::quote! {
            #typename: ::boulder::Generator,
            <#typename as ::boulder::Generator>::Output: ::std::fmt::Display,
        });
        sbody.extend(quote::quote! {
            #typename,
        });
        let c = syn::Index::from(count);
        fcall.extend(quote::quote! {
            , self.#c.generate()
        });
        args.extend(quote::quote! {
            #arg,
        });
    }

    let pattern = call.pattern;

    let persian_rug = if cfg!(feature = "persian-rug") {
        quote::quote! {
            impl<BoulderContextParam: ::persian_rug::Context, #generics> ::boulder::GeneratorWithPersianRug<BoulderContextParam> for Pattern<#generics> where #wc {
                type Output = String;
                fn generate<'boulder_lifetime_param, BoulderFunctionParam>(&mut self, context: BoulderFunctionParam) -> (Self::Output, BoulderFunctionParam)
                where
                    BoulderFunctionParam: 'boulder_lifetime_param + ::persian_rug::Mutator<Context = BoulderContextParam>
                {
                    (format!(#pattern #fcall), context)
                }
            }
        }
    } else {
        quote::quote! {}
    };

    let res: pm2::TokenStream = quote::quote! {
        {
            #[derive(Clone)]
            struct Pattern<#generics> (
                #sbody
            ) where #wc;

            impl<#generics> ::boulder::Generator for Pattern<#generics> where #wc {
                type Output = String;
                fn generate(&mut self) -> String {
                    format!(#pattern #fcall)
                }
            }

            #persian_rug

            Pattern(#args)
        }
    };

    res
}
