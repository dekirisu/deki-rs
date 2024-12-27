use convert_case::Casing;
use deki_core::*;
use deki_proc::*;
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use proc_macro::TokenStream as CompilerTokens;
use syn::spanned::Spanned;

// Force Name \\

    fn foname_tree(t:&TokenTree) -> Option<TokenTree> {
        exit!{*TokenTree::Group(g0) = t}
        exit!{*Delimiter::Bracket = g0.delimiter()}
        let mut g0 = g0.stream().as_vec();
        exit!{if g0.len()!=1}
        exit!{*TokenTree::Group(g1) = g0.pop().unwrap()}
        exit!{*Delimiter::Parenthesis = g1.delimiter()}
        let stream = g1.stream();
        let span = stream.span();
        let mut split = stream.peek_iter().split_punct('@');
        let case = split.get(1).map(|t|match t.to_string().as_str() {
            "snake" => Case::Snake,
            "camel" => Case::Camel,
            "scream" => Case::UpperSnake,
            "flat" => Case::Flat,
            "upper" => Case::UpperFlat,
            _ => Case::Pascal
        }).unwrap_or(Case::Pascal);
        let stream = split.swap_remove(0);
        let text = stream.to_string().chars()
            .map(|c|if c.is_alphanumeric() {c} else {'_'})
            .collect::<String>()
            .to_case(case);
        Some(text.ident_span(span).into())
    }

    fn foname_stream(i:TokenStream) -> TokenStream {
        TokenStream::from_iter(i.into_iter().map(|p| match foname_tree(&p) {
            Some(t) => t,
            _ => match p {
                TokenTree::Group(g) => {
                    let stream = foname_stream(g.stream());
                    TokenTree::Group(Group::new(g.delimiter(),stream))
                }
                _ => p
            }
        }))
    }

    #[proc_macro]
    pub fn foname(token:CompilerTokens) -> CompilerTokens {
        foname_stream(token.into()).into()
    }

// EOF \\
