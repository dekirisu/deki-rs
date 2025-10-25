use convert_case::Casing;
use deki_core::*;
use deki_proc::{syn::{parse2, Data, DeriveInput, Generics, Index}, *};
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use proc_macro::TokenStream as CompilerTokens;
use syn::spanned::Spanned;

// Random Utils \\

    #[proc_macro]
    pub fn xoxo(item:CompilerTokens) -> CompilerTokens {
        TokenStream::from(item).replace_atoms(|t|match t {
            TokenTree::Ident(i) if i.to_string().as_str() == "X" => "true".ident_span(i.span()).into(),
            TokenTree::Ident(i) if i.to_string().as_str() == "O" => "false".ident_span(i.span()).into(),
            _ => t
        }).into()
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
        let stream: TokenStream = item.into();
        let attr: TokenStream = attr.into();
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
