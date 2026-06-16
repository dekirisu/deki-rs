use std::collections::HashMap;
use std::str::FromStr;

use deki_proc::convert_case::Casing;
use deki_proc::{
    syn::{parse2, Generics, Index},
    Delimiter, Group, TokenStream, TokenTree,
    *,
};
use deki_proc::StringProcExt;
use maflow::*;
use proc_macro::TokenStream as CompilerTokens;
use quote::quote as qt;
use syn::{parse_macro_input, Data, DeriveInput, spanned::Spanned};

derive_preset::create!{
    hashable    "PartialEq,Eq,Hash,Clone,Copy"
    serde       "Serialize,Deserialize,Clone"
    serde_hash  "Serialize,Deserialize,PartialEq,Eq,Hash,Clone,Copy"
    deref       "drv::Deref,drv::DerefMut"
}

/// Generate `cycle_next()` and `cycle_prev()` for unit-variant enums.
#[proc_macro_derive(Cycle)]
pub fn cycle(input:CompilerTokens) -> CompilerTokens {
    let input = parse_macro_input!(input as DeriveInput);
    let DeriveInput { attrs: _, vis: _, ident, generics, data } = input;
    let (gimpl,gtype,gwhere) = generics.split_for_impl();

    match data {
        Data::Enum(enm) => {
            let (mut front,mut back) = (qt!{},qt!{});
            for (id,v) in enm.variants.iter().enumerate() {
                let that = &v.ident;
                let next = &enm.variants[(id+1)%enm.variants.len()].ident;
                front.extend(qt!(Self::#that => Self::#next,));
                back.extend(qt!(Self::#next => Self::#that,));
            }

            qt!{
                impl #gimpl ::deki_core::Cycle for #ident #gtype #gwhere {
                    fn cycle_next(&self) -> Self {match self {#front}}
                    fn cycle_prev(&self) -> Self {match self {#back}}
                }
            }.into()

        }
        _ => qt!().into()
    }
}

/// Generate `Default` by calling `.default()` on each field.
#[proc_macro_derive(ForceDefault)]
pub fn force_default (item:CompilerTokens) -> CompilerTokens {
    let input: DeriveInput = syn::parse(item).unwrap();
    let DeriveInput{attrs:_,vis:_,ident,generics,data} = input;
    let (imp,typ,wher) = generics.split_for_impl();
    let mut mults = vec![];
    if let Data::Struct(data) = data {
        for (idx,field) in data.fields.iter().enumerate() {
            let idx = Index::from(idx);
            let name = field.ident.clone()
                .map(|a|a.into_token_stream())
                .unwrap_or(qt![#idx]);
            mults.push(qt![#name:Default::default()]);
        }
    }
    qt!{impl #imp Default for #ident #typ #wher {
        fn default() -> Self {Self{#(#mults),*}}
    }}.into()
}

// Random Utils \\

    /// Replace every `bool` with `X` or `O` in a token tree: `true` → `X`, `false` → `O`.
    ///
    /// Meant for pattern-matching booleans in macros:
    /// ```rust
    /// use deki_macros::xoxo;
    /// let result = xoxo!{match [true,false,true] {
    ///     [O,O,O] => "nope",
    ///     [O,O,X] => "nope",
    ///     [X,O,X] => "YEP!",
    ///     [_,_,_] => "nope"
    /// }};
    /// assert_eq!(result, "YEP!");
    /// ```
    #[proc_macro]
    pub fn xoxo(item:CompilerTokens) -> CompilerTokens {
        TokenStream::from(item).replace_atoms(|t|match t {
            TokenTree::Ident(i) if i.to_string().as_str() == "X" => "true".ident_span(i.span()).into(),
            TokenTree::Ident(i) if i.to_string().as_str() == "O" => "false".ident_span(i.span()).into(),
            _ => t
        }).into()
    }

    /// Implement traits with a single required method.
    ///
    /// The trait must have exactly one method, and the impl body is named after the trait (snake_case).
    ///
    /// # Usage
    /// ```rust
    /// use deki_macros::quimp;
    ///
    /// struct Wrapper(i32);
    /// impl Wrapper { fn new(v:i32) -> Self { Self(v) } }
    ///
    /// quimp!{Wrapper
    ///    fn clone(&self) -> Self {Self::new(self.0)};
    ///    fn default() -> Self {Self::new(100)};
    /// }
    /// assert_eq!(Wrapper::default().0, 100);
    /// ```
    #[proc_macro]
    pub fn quimp (item:CompilerTokens) -> CompilerTokens {
        let stream: TokenStream = item.into();
        let mut iter = stream.peek_iter();
        let name = iter.next().unwrap();

        let mut gens = qt!();
        while let Some(tok) = iter.next_if(|a|!a.is_string("fn")) {
            gens.extend([tok]);
        }
        let gens: Generics = parse2(gens).unwrap();
        let (gen_impl,gen_typ,gen_where) = gens.split_for_impl();

        let mut split = iter.split_punct('|');
        let toki = split.remove(0);
        let iter = toki.peek_iter();

        let mut stream = qt!{};
        for func in iter.split_punct(';') {
            let mut fiter = func.peek_iter();
            fiter.next();
            let func = fiter.next().unwrap();
            let trai = func.to_string().to_case(Case::Pascal).ident();
            let stuff = TokenStream::from_iter(fiter);
            stream.extend(qt!(
                 impl #gen_impl #trai for #name #gen_typ #gen_where {
                    fn #func #stuff
                }
            ));
        }
        let implo = split.pop().map(|a|{
            TokenStream::from_iter(a)
        });
        qt!{
            #stream
            impl #gen_impl #name #gen_typ #gen_where {
                 #implo
            }
        }.into()
    }

    /// Add a method to a type or impl block:
    /// - `#[imp(Struct)]` on fn: for an owned type
    /// - `#[imp(*Struct)]` on fn: auto-create a unit struct
    /// - `#[imp(Struct|Trait)]` on fn: impl a single-method trait
    /// - `#[imp(Struct|*)]` on fn: foreign type (generates a new trait)
    /// - `#[imp(TraitName)] impl Type { ... }`: impl an existing trait
    /// - `#[imp(*NewTraitName)] impl Type { ... }`: generate trait + impl
    /// - `#[imp(*)] impl Type { ... }`: auto-generate trait name + impl
    #[proc_macro_attribute]
    pub fn imp (attr:CompilerTokens,item:CompilerTokens) -> CompilerTokens {
        let item: TokenStream = item.into();
        let attr: TokenStream = attr.into();
        deki_proc::imp(attr,item).into()
    }

    /// Define per-variant enum methods with unmatched variants falling back to Default::default().
    #[proc_macro]
    pub fn match_fns (item:CompilerTokens) -> CompilerTokens {
        let stream: TokenStream = item.into();
        let mut stream = stream.peek_iter();
        let name = stream.next().unwrap().unwrap_group().stream();
        let iter = stream.split_punct(';');

        let mut funcs = Vec::new();
        let mut matches = HashMap::<String,TokenStream>::new();
        let mut current = qt![];

        for tok in iter {
            let mut toki = tok.peek_iter();
            // Update Current Title
            let title = toki.peek().and_then(|t|{
                exit![*TokenTree::Group(g) = t];
                exit![*Delimiter::Bracket = g.delimiter()];
                Some(g.stream())
            });
            if let Some(title) = title {
                toki.next();
                current = title;
            }
            if current.is_empty() {
                funcs.push(TokenStream::from_iter(toki));
            } else {
                let [func,b] = toki.split_punct(':').try_into().unwrap();
                matches.entry(func.to_string()).or_default()
                    .extend(qt!{#name #current => #b,});
            }
        }

        let mut asdf = qt![];
        for a in funcs {
            let mut aiter = a.peek_iter();
            exit![bb = aiter.next()];
            exit![atr = aiter.next(),unwrap_group()];
            let atr = atr.stream().peek_iter().split_punct(',');
            next![mchs = matches.remove(&bb.to_string())];
            let more = TokenStream::from_iter(aiter);
            asdf.extend(qt!(
                pub fn #bb (&self #(,#atr)*) #more {
                    match self { #mchs _ => Default::default() }
                }
            ));
        }

        qt![impl #name {#asdf}].into()
    }


// Force Name \\

    fn foname_tree(t:&TokenTree) -> Option<TokenTree> {
        exit![*TokenTree::Group(g0) = t];
        exit![*Delimiter::Bracket = g0.delimiter()];
        let mut g0 = g0.stream().as_vec();
        exit![if g0.len()!=1];
        exit![*TokenTree::Group(g1) = g0.pop().unwrap()];
        exit![*Delimiter::Parenthesis = g1.delimiter()];
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

// Enum Field Count \\

    /// Generate `fn field_count(&self) -> usize` for enums.
    #[proc_macro_derive(EnumFieldCount)]
    pub fn enum_field_count(item:CompilerTokens) -> CompilerTokens {
        let input: DeriveInput = syn::parse(item).unwrap();
        let DeriveInput{attrs:_,vis:_,ident,generics,data} = input;
        let (imp,typ,wher) = generics.split_for_impl();
        let mut counts = qt!{};
        if let Data::Enum(data) = data {
            for var in &data.variants {
                let name = var.ident.clone();
                let count = var.fields.len();
                let fields = if count==0 {qt!{}} else {
                    let iter = var.fields.iter().enumerate().map(|(i,a)|{
                        let n = a.ident.clone().map(|a|qt!{#a}).unwrap_or({let i = Index::from(i);qt!{#i}});
                        qt!{#n:_}
                    });
                    qt!{{#(#iter),*}}
                };
                counts.extend(qt!{Self::#name #fields => #count,});
            }
        }
        qt!{impl #imp #ident #typ #wher {
            pub fn field_count(&self) -> usize {match self {
                #counts
            }}
        }}.into()
    }

// Derive From \\

    /// Generate `impl From<A> for B` from function signatures.
    #[proc_macro]
    pub fn derive_from(stream:CompilerTokens) -> CompilerTokens {
        let stream: TokenStream = stream.into();
        let mut iter = stream.peek_iter();
        let typ = iter.next().unwrap();
        let mut out = qt!{};
        for a in iter.split_punct(';') {
            let mut aiter = a.into_iter();
            let tya = aiter.next().unwrap();
            aiter.next();
            aiter.next();
            let rst = TokenStream::from_iter(aiter);
            out.extend(qt!{
                impl From<#tya> for #typ {
                    fn from(d:#tya) -> Self {Self::#rst}
                }
            });
        }
        out.into()
    }

// Derived Attribute \\

    /// Batch-apply derive macros by name.
    ///
    /// Supported presets: `_Serde`, `_Hashable`, `_Deref`, `_Math`, `_Id`, `_States`, `_SystemSet`, `_Payload`, `_SevyMelt`.
    ///
    /// # Usage
    /// ```rust
    /// use deki_macros::derived;
    ///
    /// #[derived(_Hashable)]
    /// struct Point { x: i32, y: i32 }
    /// ```
    #[proc_macro_attribute]
    pub fn derived(attr:CompilerTokens, item:CompilerTokens) -> CompilerTokens {
        let stream: TokenStream = item.into();
        let attr: TokenStream = attr.into();
        let mut derives = std::collections::HashSet::new();
        let mut addattr = qt!();
        for token in attr.into_iter() {
            next![*TokenTree::Ident(name) = token];
            let name = name.to_string();
            let list = match name.as_str() {
                "_Serde" => vec!["serde::Serialize", "serde::Deserialize"],
                "_Hashable" => vec!["PartialEq", "Eq", "Hash", "Clone", "Copy"],
                "_Deref" => vec!["drv::Deref", "drv::DerefMut"],
                "_Payload" => vec!["serde::Serialize", "serde::Deserialize", "Component", "Clone"],
                "_SevyMelt" => vec!["serde::Serialize", "serde::Deserialize", "Default", "Melt", "Component"],
                "_Id" => vec!["PartialEq", "Eq", "PartialOrd", "Ord", "Hash", "Clone", "Copy", "Default"],
                "_States" => vec!["PartialEq", "Eq", "Hash", "Clone", "Copy", "States", "Debug"],
                "_SystemSet" => vec!["PartialEq", "Eq", "PartialOrd", "Ord", "Hash", "Debug", "Clone", "Copy", "Default", "SystemSet"],
                "_Math" => vec!["drv::Add", "drv::Sub", "drv::Mul", "drv::Div"],
                _ => vec![name.as_str()],
            };
            derives.extend(list.into_iter().map(|v|v.to_string()));
            if name == "_Math" {
                addattr.extend(qt!{#[mul(forward)]});
            }
        }
        let derives = Vec::from_iter(derives.iter().map(|v|TokenStream::from_str(v).unwrap()));
        qt!{#[derive(#(#derives),*)] #addattr #stream}.into()
    }

// Derive Math \\

    /// Generate `impl Add/Sub/Mul/Div<A> for T` from function signatures.
    #[proc_macro]
    pub fn derive_math(stream:CompilerTokens) -> CompilerTokens {
        let stream: TokenStream = stream.into();
        let mut iter = stream.peek_iter();
        let typ = iter.next().unwrap();
        let mut out = qt!{};
        for a in iter.split_punct(';') {
            let mut aiter = a.peek_iter();
            let tya1 = aiter.next().unwrap();
            aiter.next();
            let tya2 = aiter.next().unwrap();
            aiter.next();
            aiter.next();
            aiter.next();
            let outp = aiter.next().unwrap();
            aiter.next();
            aiter.next();
            let is_mut = aiter.peek_punct() == '*';
            if is_mut { aiter.next(); }
            let muty = if is_mut {qt!{;self}} else {qt!{}};
            let rst = TokenStream::from_iter(aiter);
            let fnc = tya1.to_string().to_lowercase().ident();
            out.extend(qt!{
                impl #tya1<#tya2> for #typ {
                    type Output = #outp;
                    fn #fnc(mut self, rhs:#tya2) -> #outp {#rst #muty}
                }
            });
        }
        out.into()
    }

// EOF \\
