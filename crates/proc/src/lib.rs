use std::iter::Peekable;
use deki_core::*;
use syn::*;
use proc_macro2::token_stream::IntoIter; 
pub use quote;
pub use proc_macro2;
pub use syn;
pub use convert_case;

// Specific Re-Exports \\

    pub use convert_case::{Case,Casing};
    pub use quote::{quote as qt, quote_spanned as qts, ToTokens};
    pub use proc_macro2::{
        Delimiter, Span, TokenStream, TokenTree, // no-brainers
        Group, Ident, Punct, Literal // probably always needed
    };
    pub use syn::spanned::Spanned;


// String <> Ident \\

    #[ext(pub trait StringProcExt)]
    impl String {
        fn ident(&self) -> Ident {Ident::new(&self,Span::call_site())}
        fn ident_span(&self,span:Span) -> Ident {Ident::new(self,span)}
    }

    impl <'a> StringProcExt for &'a str {
        fn ident(&self) -> Ident {Ident::new(self,Span::call_site())}
        fn ident_span(&self,span:Span) -> Ident {Ident::new(self,span)}
    }


// Atomic Exts \\

    #[ext(pub trait IdentExt)]
    impl Ident { 
        fn to_case(&self,case:Case) -> Ident {
            Ident::new(&self.to_string().to_case(case),self.span())
        }
    }

    #[ext(pub trait LiteralExt)]
    impl Literal {
        fn is_numeric(&self) -> bool {
            exit!{first = self.to_string().chars().next()}
            first.is_numeric()
        }
    }

    #[ext(pub trait PunctExt)]
    impl Punct {
    
    }


// Neat Token Iterator \\

    /// Token Level Return
    /// - Some: It's the requested thing
    /// - None: It can't be the requested thing
    /// - Maybe: It can be the requested thing, but something is missing
    #[derive(Default)]
    pub enum Check<T,M> {#[default] None, Some(T), Maybe(M)}

    #[ext(pub trait TokenTreeExt)]
    impl TokenTree {
        /// check if this is an ident, use .. && self.is_string(..) for a specific
        fn is_ident(&self) -> bool {
            exit!{*TokenTree::Ident(_)=self}
            true
        }
        /// dirty string check of the token
        fn is_string(&self,text:&str) -> bool {
            self.to_string().as_str() == text
        }
        /// check if it's a certain punct
        fn is_punct(&self,punct:char) -> bool {
            exit!{*TokenTree::Punct(p) = self}
            p.as_char() == punct
        }
        /// check if it's a numeric literal
        fn is_numeric(&self) -> bool {
            exit!{*TokenTree::Literal(lit) = self}
            lit.is_numeric()
        }
        /// get literal if you know it's one
        fn risk_literal(self) -> Literal {
            kill!{*Self::Literal(lit) = self}
            lit
        }
        /// get literal if you know it's one
        fn risk_ident(self) -> Ident {
            kill!{*Self::Ident(idn) = self}
            idn
        }
        /// get punct if you know it's one
        fn risk_punct(self) -> Punct {
            kill!{*Self::Punct(pct) = self}
            pct
        }
        /// get group if you know it's one
        fn risk_group(self) -> Group {
            kill!{*Self::Group(grp) = self}
            grp
        }
    }

    #[ext(pub trait OptTreeExt)]
    impl <'a> Option<&'a TokenTree> {
        /// check if this is an ident, use .. && self.is_string(..) for a specific
         fn is_ident(&self) -> bool {
            exit!{tok = self}
            tok.is_ident()
        } 
        /// dirty string check of the token
        fn is_string(&self,text:&str) -> bool {
            exit!{tok = self}
            tok.is_string(text)
        }
        /// check if it's a numeric literal
        fn is_numeric(&self) -> bool {
            exit!{tok = self}
            tok.is_numeric()
        }
        /// check if it's a certain punct
        fn is_punct(&self,punct:char) -> bool {
            exit!{tok = self}
            tok.is_punct(punct)
        }
    }


// Stream Handling \\

    /// Stream Level Return
    /// - None: Iter didn't progress: Can't be the requested thing
    /// - Base: Iter progressed (e.g. due to checks), here are the OG things
    /// - Shift: Iter progressed: Successfully processed whatever requested
    #[derive(Default)]
    pub enum Step<T,M> {#[default] None, Base(T), Shift(M)}

    impl <T,M> Step <T,M> {
        pub fn risk_shift(self) -> M {match self {
            Self::Shift(m) => m,
            _ => panic!{}
        }}
        pub fn shift_or(self,m:M) -> M {match self {
            Self::Shift(m) => m,
            _ => m
        }}
    }

    #[ext(pub trait TokenStreamExt)]
    impl TokenStream {
        fn peek_iter(self) -> PeekIter {
            self.into_iter().peekable()
        }
        fn with_span(self,span:Span) -> Self {
            Self::from_iter(self.into_iter().map(|mut a|{a.set_span(span);a}))
        }
        fn as_vec(self) -> Vec<TokenTree> {
            self.into_iter().collect()
        }
    }

    pub type PeekIter = Peekable<IntoIter>;

    #[ext(pub trait TreeIterExt)]
    impl PeekIter {

        /// skip until [Self::next()] isn't a [TokenTree::Punct]
        fn skip_puncts(&mut self,stoppers:&str) -> Vec<TokenTree> {
            let mut out = vec![];
            while let Some(TokenTree::Punct(pnc)) = self.peek() {
                if stoppers.contains(pnc.as_char()) {return out}
                out.push(self.next().unwrap());
            }
            out
        }

        /// only progress iter if next is a numeric literal & return it
        fn next_if_num(&mut self) -> Option<Literal> {
            exit!{if !self.peek().is_numeric()}
            self.next().map(|l|l.risk_literal())
        }
    
        /// 
        fn try_next_float(&mut self) -> Option<(f32,LitFloat)> {
            exit!{lit = self.next_if_num()}
            let lit: LitFloat = lit.clone().into();
            kill!{num = lit.base10_parse::<f32>()}
            Some((num,lit))
        }

        /// get next punct as [char] without progressing iter, if not a punct its a `'n'` 
        fn peek_punct(&mut self) -> char {
            exit!{>(peek = self.peek())'n'}
            exit!{>(*TokenTree::Punct(p) = peek)'n'}
            p.as_char()
        }

        /// splits Tokens into multiple [TokenStream]s  by a char delimiter. 
        /// - doesn't include empty parts.
        fn split_punct(self,punct:char) -> Vec<TokenStream> {
            let mut out = vec![];
            let mut curr = vec![];
            for tree in self {
                if tree.is_punct(punct){
                    if !curr.is_empty(){
                        out.push(TokenStream::from_iter(std::mem::take(&mut curr)));
                    }
                    continue
                }
                curr.push(tree);
            }
            if !curr.is_empty(){
                out.push(TokenStream::from_iter(curr));
            }
            out
        }
    }


// EOF \\

