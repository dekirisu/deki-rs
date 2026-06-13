use extension_traits::extension as ext;
use std::iter::Peekable;
use maflow::*;
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

// Quick Implement Prepare \\

    /// Quickly add a method to a Type
    /// - `#[imp(Struct)]`: for a owned Type
    /// - `#[imp(Struct|Trait)]`: to impl a singe-method trait
    /// - `#[imp(Struct|*)]`: for a foreign Type (generates a new trait)
    pub fn imp (attr:TokenStream,stream:TokenStream) -> TokenStream {
        let mut split = attr.peek_iter().split_punct('|');
        let mut iter = split.remove(0).peek_iter();

        let name = iter.next().unwrap();
        let gens: Generics = parse2(TokenStream::from_iter(iter)).unwrap();
        let (gen_impl,gen_typ,gen_where) = gens.split_for_impl();

        let mut trat = qt!();
        let mut new = qt!();
        if let Some(tok) = split.pop() {
            if tok.to_string().as_str() == "*" {
                let fn_name = stream.clone().into_iter().skip(1).next().unwrap().to_string().to_case(Case::Pascal);
                let ident = format!("{name}{fn_name}Ext").ident_span(tok.span());
                new.extend(qt!(#[ext(pub trait #ident)]));
            } else {
                trat.extend(qt!(#tok for));
            }
        }

        qt!( #new impl #gen_impl #trat #name #gen_typ #gen_where {#stream} ).into()
    }


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


// Neat Token Iterator \\

    /// Token Level Return
    /// - Some: It's the requested thing
    /// - None: It can't be the requested thing
    /// - Maybe: It can be the requested thing, but something is missing
    #[derive(Default)]
    pub enum Check<T,M> {#[default] None, Some(T), Maybe(M)}


// Token Tree Extension \\

    macro_rules! is_any {
        ($self:ident = $var:ident) => {match $self {Self::$var(_) => true, _ => false}};
        (*$self:ident = $var:ident) => {match $self {Self::$var => true, _ => false}};
    }

    macro_rules! unwrpr {($self:ident $(.$meth:ident($($tt:tt)*))* = $var:ident) => {
        match $self $(.$meth($($tt)*))* {TokenTree::$var(v) => v, _ => panic!{"deki-rs|proc: Risk Failed!"}}
    }}

    macro_rules! checker {($self:ident::$var:ident $block:expr) => {
        match $self {Self::$var(v) => $block(v), _ => Default::default()}
    }}

    #[ext(pub trait DekiTokenTreeExt)]
    impl TokenTree {

        // Check Types
        #[inline] fn is_any_punct(&self) -> bool {is_any!(self = Punct)}
        #[inline] fn is_any_ident(&self) -> bool {is_any!(self = Ident)}
        #[inline] fn is_any_group(&self) -> bool {is_any!(self = Group)}
        #[inline] fn is_any_literal(&self) -> bool {is_any!(self = Literal)}

        // Check Unwraps
        #[inline] fn unwrap_punct(self) -> Punct {unwrpr!(self = Punct)}
        #[inline] fn unwrap_ident(self) -> Ident {unwrpr!(self = Ident)}
        #[inline] fn unwrap_group(self) -> Group {unwrpr!(self = Group)}
        #[inline] fn unwrap_literal(self) -> Literal {unwrpr!(self = Literal)}

        // Check Inner
        #[inline] fn map_punct<O:Default,F:FnOnce(&Punct)->O>(&self,check:F) -> O {checker!(self::Punct|v|(check)(v))}
        #[inline] fn map_ident<O:Default,F:FnOnce(&Ident)->O>(&self,check:F) -> O {checker!(self::Ident|v|(check)(v))}
        #[inline] fn map_group<O:Default,F:FnOnce(&Group)->O>(&self,check:F) -> O {checker!(self::Group|v|(check)(v))}
        #[inline] fn map_literal<O:Default,F:FnOnce(&Literal)->O>(&self,check:F) -> O {checker!(self::Literal|v|(check)(v))}

        // Shortcuts
        #[inline] fn is_numeric(&self) -> bool {self.map_literal(move|x|x.is_numeric())}
        #[inline] fn is_string(&self,text:&str) -> bool {self.to_string().as_str() == text}
        #[inline] fn is_punct(&self,punct:char) -> bool {self.map_punct(move|x|x.as_char()==punct)}

        // Utils \|

        /// convert to [TokenStream] or get inner tokens of a [Group]
        #[inline] fn inner_tokens(&self) -> TokenStream {match self{
            TokenTree::Group(g) => g.stream(),
            _ => self.to_token_stream()
        }}

    }

    #[ext(pub trait DekiOptTokenTreeExt)]
    impl <'a> Option<&'a TokenTree> {

        // Check Types
        #[inline] fn is_any_punct(&self) -> bool {exit!{t=self}t.is_any_punct()}
        #[inline] fn is_any_ident(&self) -> bool {exit!{t=self}t.is_any_ident()}
        #[inline] fn is_any_group(&self) -> bool {exit!{t=self}t.is_any_group()}
        #[inline] fn is_any_literal(&self) -> bool {exit!{t=self}t.is_any_literal()}

        // Check Unwraps
        #[inline] fn unwrap_punct(self) -> &'a Punct {unwrpr!(self.unwrap() = Punct)}
        #[inline] fn unwrap_ident(self) -> &'a Ident {unwrpr!(self.unwrap() = Ident)}
        #[inline] fn unwrap_group(self) -> &'a Group {unwrpr!(self.unwrap() = Group)}
        #[inline] fn unwrap_literal(self) -> &'a Literal {unwrpr!(self.unwrap() = Literal)}

        // Check Inner
        #[inline] fn map_punct<O:Default,F:FnOnce(&Punct)->O>(&self,check:F) -> O {exit!{t=self}t.map_punct(check)}
        #[inline] fn map_ident<O:Default,F:FnOnce(&Ident)->O>(&self,check:F) -> O {exit!{t=self}t.map_ident(check)}
        #[inline] fn map_group<O:Default,F:FnOnce(&Group)->O>(&self,check:F) -> O {exit!{t=self}t.map_group(check)}
        #[inline] fn map_literal<O:Default,F:FnOnce(&Literal)->O>(&self,check:F) -> O {exit!{t=self}t.map_literal(check)}

        // Shortcuts
        #[inline] fn is_numeric(&self) -> bool {exit!{t=self}t.is_numeric()}
        #[inline] fn is_string(&self,text:&str) -> bool {exit!{t=self}t.is_string(text)}
        #[inline] fn is_punct(&self,punct:char) -> bool {exit!{t=self}t.is_punct(punct)}

    }


// Variants \\

    #[ext(pub trait IdentExt)]
    impl Ident { 
        fn to_case(&self,case:Case) -> Ident {
            Ident::new(&self.to_string().to_case(case),self.span())
        }
    }

    #[ext(pub trait LiteralExt)]
    impl Literal {
        fn is_numeric(&self) -> bool {
            match self.to_string().chars().next(){
                Some(first) => first.is_numeric(),
                _ => false
            }
        }
        #[inline] fn try_int(&self) -> Result<LitInt> {parse2(self.into_token_stream())}
        #[inline] fn try_float(&self) -> Result<LitFloat> {parse2(self.into_token_stream())}
    }

    #[ext(pub trait PunctExt)]
    impl Punct {
    
    }

    #[ext(pub trait DelimiterExt)]
    impl Delimiter {
        #[inline] fn is_parenthesis(&self) -> bool {is_any!(*self = Parenthesis)}        
        #[inline] fn is_brace(&self) -> bool {is_any!(*self = Brace)}        
        #[inline] fn is_bracket(&self) -> bool {is_any!(*self = Bracket)}
        #[inline] fn is_none(&self) -> bool {is_any!(*self = None)}
    }


// -- \\



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

        fn replace_atoms(self,func:fn(TokenTree)->TokenTree) -> Self {
            Self::from_iter(self.into_iter().map(|a|match &a {
                TokenTree::Group(g) => TokenTree::Group(Group::new(g.delimiter(),g.stream().replace_atoms(func))),
                _ => func(a)
            }))
        }

    }

    pub type PeekIter = Peekable<IntoIter>;

    #[ext(pub trait TreeIterExt)]
    impl PeekIter {

        fn collect_til_punct(&mut self,punct:char) -> Vec<TokenTree> {
            let mut out = vec![];
            while let Some(a) = self.next_if(|a|!a.is_punct(punct)){
                out.push(a);
            }
            out
        }

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
            self.next().map(|l|l.unwrap_literal())
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

