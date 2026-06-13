use proc_macro::TokenStream as CompilerTokens;
use deki_proc::{syn::Index, *};
use syn::{parse_macro_input, Data, DeriveInput};

derive_preset::create!{
    hashable    "PartialEq,Eq,Hash,Clone,Copy"
    serde       "Serialize,Deserialize,Clone"
    serde_hash  "Serialize,Deserialize,PartialEq,Eq,Hash,Clone,Copy"
    deref       "drv::Deref,drv::DerefMut"
}

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
                impl #gimpl Cycle for #ident #gtype #gwhere {
                    fn cycle_next(&self) -> Self {match self {#front}}
                    fn cycle_prev(&self) -> Self {match self {#back}}
                }
            }.into()

        }
        _ => qt!().into()
    }
}

 #[proc_macro_derive(ForceDefault)]
pub fn force_default (item:CompilerTokens) -> CompilerTokens {
    let input: DeriveInput = syn::parse(item).unwrap();
    let DeriveInput{attrs:_,vis:_,ident,generics,data} = input;
    let (imp,typ,wher) = generics.split_for_impl();
    let mut mults = vec![];
    match data {
        Data::Struct(data) => for (idx,field) in data.fields.iter().enumerate() {
            let idx = Index::from(idx);
            let name = field.ident.clone()
                .map(|a|a.into_token_stream())
                .unwrap_or(qt![#idx]);
            mults.push(qt![#name:default()]);
        }
        _ => {}
    }
    qt!{impl #imp Default for #ident #typ #wher {
        fn default() -> Self {Self{#(#mults),*}}
    }}.into()
}

use std::collections::HashMap;

use convert_case::Casing;
use deki_core::*;
use deki_proc::syn::{parse2, Generics};
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use syn::spanned::Spanned;

// Random Utils \\

    /// Replace every bool to a X or O:
    /// - true: X
    /// - false: O
    ///
    /// Meant to be used in bool matches:
    /// ```rust
    /// xoxo!{match [true,false,true] {
    ///     [O,O,O] => "nope",
    ///     [O,O,X] => "nope",
    ///     [X,O,X] => "YEP!",
    ///     [_,_,_] => "nope"
    /// }}
    /// ```
    #[proc_macro]
    pub fn xoxo(item:CompilerTokens) -> CompilerTokens {
        TokenStream::from(item).replace_atoms(|t|match t {
            TokenTree::Ident(i) if i.to_string().as_str() == "X" => "true".ident_span(i.span()).into(),
            TokenTree::Ident(i) if i.to_string().as_str() == "O" => "false".ident_span(i.span()).into(),
            _ => t
        }).into()
    }

    /// Quick Implementations:
    /// - the trait has to have 1 required method
    /// - ..which is named like tie trait (but snake-case)
    ///
    /// # Usage
    /// ```rust
    /// quimp!{StructName
    ///    fn clone(&self) -> Self {Self::new(self.0)};
    ///    fn default() -> Self {Self::new(100)};
    /// }
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
            TokenStream::from_iter(a.into_iter())
        });
        qt!{
            #stream
            impl #gen_impl #name #gen_typ #gen_where {
                 #implo
            }
        }.into()
    }

    /// Quickly add a method to a Type
    /// - `#[imp(Struct)]`: for a owned Type
    /// - `#[imp(Struct|Trait)]`: to impl a singe-method trait
    /// - `#[imp(Struct|*)]`: for a foreign Type (generates a new trait)
    #[proc_macro_attribute]
    pub fn imp (attr:CompilerTokens,item:CompilerTokens) -> CompilerTokens {
        let item: TokenStream = item.into();
        let attr: TokenStream = attr.into();
        deki_proc::imp(attr,item).into()
    }

    /// Alternative Syntax to attach functionality to type variants:
    /// - atm: Return type has to impl Default
    /// ```rust
    /// enum Object {RedSphere, GreenCube}
    /// match_fns!{
    ///
    ///     // 1. Define Methods - '&self' is assumed as parameter
    ///     [Object]
    ///     shape() -> &'static str;
    ///     color(brightness:f32) -> &'static str;
    ///
    ///     // 2. Add Code
    ///     [::RedSphere]
    ///     shape: "sphere";
    ///     color: if brightness > 0.5 {"bright-red"} else {"red"};
    ///
    ///     [::GreenCube]
    ///     shape: "cube";
    ///     color: "just-green";
    ///
    /// }
    /// ```
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
                exit!{*TokenTree::Group(g) = t}
                exit!{*Delimiter::Bracket = g.delimiter()}
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
            exit!{bb = aiter.next()}
            exit!{atr = aiter.next(),unwrap_group()}
            let atr = atr.stream().peek_iter().split_punct(',');
            next!{mchs = matches.remove(&bb.to_string())}
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
