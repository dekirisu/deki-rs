use std::iter::Peekable;

use extension_traits::extension as ext;
use maflow::*;
use proc_macro2::token_stream::IntoIter;
use syn::{token::For, *};

pub use {convert_case, proc_macro2, quote, syn};

// Specific Re-Exports \\

    pub use convert_case::{Case,Casing};
    pub use quote::{quote as qt, quote_spanned as qts, ToTokens};
    pub use proc_macro2::{
        Delimiter, Span, TokenStream, TokenTree, // no-brainers
        Group, Ident, Punct, Literal // probably always needed
    };
    pub use syn::spanned::Spanned;

// Quick Implement Prepare \\

    fn imp_hlpr(attr:TokenStream,imp_block:TokenStream) -> TokenStream {
        let mut iter = attr.peek_iter();
        let create_trait = iter.devour_punct('*');
        let maybe_path = iter.peek().is_some().then_some(TokenStream::from_iter(iter));
        let mut input: ItemImpl = syn::parse2(imp_block).unwrap();
        let path_tok = maybe_path.unwrap_or_else(||{
            let self_ty = input.self_ty.to_token_stream().to_string()
                .chars().filter(|v|v.is_alphanumeric())
                .collect::<String>().to_case(Case::Pascal);
            let first_fn = input.items.iter().find_map(|a|match a {
                ImplItem::Fn(a) => Some(a.sig.ident.to_string().to_case(Case::Pascal)), _=>None
            }).unwrap();
            (self_ty+&first_fn+"Ext").ident().into_token_stream()
        });
        let path: Path = parse2(path_tok).unwrap();
        let mut out = qt![];
        if !create_trait {
            input.trait_ = Some((None,path.clone(),For::default()));
        }
        out.extend(input.into_token_stream());
        if create_trait {
            out = qt![ #[ext(pub trait #path)] #out ];
        }
        out
    }

    /// Add a method to a type or impl block:
    /// - `#[imp(Struct)]` on fn: for an owned type
    /// - `#[imp(*Struct)]` on fn: auto-create a unit struct
    /// - `#[imp(Struct|Trait)]` on fn: impl a single-method trait
    /// - `#[imp(Struct|*)]` on fn: foreign type (generates a new trait)
    /// - `#[imp(TraitName)] impl Type { ... }`: impl an existing trait
    /// - `#[imp(*NewTraitName)] impl Type { ... }`: generate trait + impl
    /// - `#[imp(*)] impl Type { ... }`: auto-generate trait name + impl
    pub fn imp (attr:TokenStream,stream:TokenStream) -> TokenStream {
        let is_impl = stream.to_token_stream().into_iter().find(|v|v.is_string("impl")).is_some();
        if is_impl {
           imp_hlpr(attr,stream)
        } else {
            let mut split = attr.peek_iter().split_punct('|');
            let mut iter = split.remove(0).peek_iter();
            let create_unit = iter.devour_punct('*');
            let name = iter.next().unwrap();
            let gens: Generics = parse2(TokenStream::from_iter(iter)).unwrap();
            let (gen_impl,gen_typ,gen_where) = gens.split_for_impl();
            let mut imp_block = qt!( impl #gen_impl #name #gen_typ #gen_where {#stream} );
            if let Some(tok) = split.pop() {
                imp_block = imp_hlpr(tok,imp_block);
            }
            let unit = if create_unit { qt![pub struct #name;] } else { qt![] };
            qt!{#unit #imp_block}


//           let mut trat = qt!();
//            let mut new = qt!();
//            if let Some(tok) = split.pop() {
//                let mut iter2 = tok.into_token_stream();
//                let create_trait = iter.devour_punct('*');
//                
//                if tok.to_string().as_str() == "*" {
//                    let fn_name = stream.clone().into_iter().nth(1).unwrap().to_string().to_case(Case::Pascal);
//                    let ident = format!("{name}{fn_name}Ext").ident_span(tok.span());
//                    new.extend(qt!(#[ext(pub trait #ident)]));
//                } else {
//                    trat.extend(qt!(#tok for));
//                }
//            }
//
//            qt!( #unit #new impl #gen_impl #trat #name #gen_typ #gen_where {#stream} )
         }
   }


// String <> Ident \\

    /// Convert a string to an `Ident`.
    #[ext(pub trait StringProcExt)]
    impl String {
        fn ident(&self) -> Ident {Ident::new(self,Span::call_site())}
        fn ident_span(&self,span:Span) -> Ident {Ident::new(self,span)}
    }

    impl StringProcExt for &str {
        fn ident(&self) -> Ident {Ident::new(self,Span::call_site())}
        fn ident_span(&self,span:Span) -> Ident {Ident::new(self,span)}
    }


// Neat Token Iterator \\

    /// A token-level return type.
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
        #[inline]
        #[allow(clippy::redundant_closure)]
        fn map_punct<O:Default,F:FnOnce(&Punct)->O>(&self,check:F) -> O {checker!(self::Punct|v|check(v))}
        #[inline]
        #[allow(clippy::redundant_closure)]
        fn map_ident<O:Default,F:FnOnce(&Ident)->O>(&self,check:F) -> O {checker!(self::Ident|v|check(v))}
        #[inline]
        #[allow(clippy::redundant_closure)]
        fn map_group<O:Default,F:FnOnce(&Group)->O>(&self,check:F) -> O {checker!(self::Group|v|check(v))}
        #[inline]
        #[allow(clippy::redundant_closure)]
        fn map_literal<O:Default,F:FnOnce(&Literal)->O>(&self,check:F) -> O {checker!(self::Literal|v|check(v))}

        // Shortcuts
        #[inline] fn is_numeric(&self) -> bool {self.map_literal(move|x|x.is_numeric())}
        #[inline] fn is_string(&self,text:&str) -> bool {self.to_string().as_str() == text}
        #[inline] fn is_punct(&self,punct:char) -> bool {self.map_punct(move|x|x.as_char()==punct)}

        // Utils \|

        /// For Groups return the inner stream; for single tokens, wrap them in a stream.
        #[inline] fn inner_tokens(&self) -> TokenStream {match self{
            TokenTree::Group(g) => g.stream(),
            _ => self.to_token_stream()
        }}

    }

    #[ext(pub trait DekiOptTokenTreeExt)]
    impl <'a> Option<&'a TokenTree> {

        // Check Types
        #[inline] fn is_any_punct(&self) -> bool {exit![t=self];t.is_any_punct()}
        #[inline] fn is_any_ident(&self) -> bool {exit![t=self];t.is_any_ident()}
        #[inline] fn is_any_group(&self) -> bool {exit![t=self];t.is_any_group()}
        #[inline] fn is_any_literal(&self) -> bool {exit![t=self];t.is_any_literal()}

        // Check Unwraps
        #[inline] fn unwrap_punct(self) -> &'a Punct {unwrpr!(self.unwrap() = Punct)}
        #[inline] fn unwrap_ident(self) -> &'a Ident {unwrpr!(self.unwrap() = Ident)}
        #[inline] fn unwrap_group(self) -> &'a Group {unwrpr!(self.unwrap() = Group)}
        #[inline] fn unwrap_literal(self) -> &'a Literal {unwrpr!(self.unwrap() = Literal)}

        // Check Inner
        #[inline] fn map_punct<O:Default,F:FnOnce(&Punct)->O>(&self,check:F) -> O {exit![t=self];t.map_punct(check)}
        #[inline] fn map_ident<O:Default,F:FnOnce(&Ident)->O>(&self,check:F) -> O {exit![t=self];t.map_ident(check)}
        #[inline] fn map_group<O:Default,F:FnOnce(&Group)->O>(&self,check:F) -> O {exit![t=self];t.map_group(check)}
        #[inline] fn map_literal<O:Default,F:FnOnce(&Literal)->O>(&self,check:F) -> O {exit![t=self];t.map_literal(check)}

        // Shortcuts
        #[inline] fn is_numeric(&self) -> bool {exit![t=self];t.is_numeric()}
        #[inline] fn is_string(&self,text:&str) -> bool {exit![t=self];t.is_string(text)}
        #[inline] fn is_punct(&self,punct:char) -> bool {exit![t=self];t.is_punct(punct)}

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

    /// A stream-level return type.
    #[derive(Default)]
    pub enum Step<T,M> {#[default] None, Base(T), Shift(M)}

    impl <T,M> Step <T,M> {
        /// Panic-unwrap the `Shift` variant.
        pub fn risk_shift(self) -> M {match self {
            Self::Shift(m) => m,
            _ => panic!{}
        }}
        /// Unwrap `Shift` or provide a fallback value.
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

    /// A peekable iterator over `TokenTree`s
    pub type PeekIter = Peekable<IntoIter>;

    #[ext(pub trait TreeIterExt)]
    impl PeekIter {

        // Devour
        #[inline] fn devour_punct(&mut self,p:char) -> bool {self.next_if(|v|v.is_punct(p)).is_some()}
        //#[inline] fn devour_ident(self) -> bool {..}
        //#[inline] fn devour_group(self) -> bool {..}
        //#[inline] fn devour_literal(self) -> bool {..}

 
        fn collect_til_punct(&mut self,punct:char) -> Vec<TokenTree> {
            let mut out = vec![];
            while let Some(a) = self.next_if(|a|!a.is_punct(punct)){
                out.push(a);
            }
            out
        }

        /// Skip until hitting a punct in stoppers.
        fn skip_puncts(&mut self,stoppers:&str) -> Vec<TokenTree> {
            let mut out = vec![];
            while let Some(TokenTree::Punct(pnc)) = self.peek() {
                if stoppers.contains(pnc.as_char()) {return out}
                out.push(self.next().unwrap());
            }
            out
        }

        /// Consume and return the next token only if it's a numeric literal.
        fn next_if_num(&mut self) -> Option<Literal> {
            exit![if !self.peek().is_numeric()];
            self.next().map(|l|l.unwrap_literal())
        }
    
        /// Try parsing the next numeric literal as an f32.
        fn try_next_float(&mut self) -> Option<(f32,LitFloat)> {
            exit![lit = self.next_if_num()];
            let lit: LitFloat = lit.clone().into();
            kill![num = lit.base10_parse::<f32>()];
            Some((num,lit))
        }

        /// Peek the next punctuation character; returns `'n'` as sentinel if not a punct.
        fn peek_punct(&mut self) -> char {
            exit![>(peek = self.peek())'n'];
            exit![>(*TokenTree::Punct(p) = peek)'n'];
            p.as_char()
        }

        /// Split a token stream by a punctuation delimiter, skipping empty parts.
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
