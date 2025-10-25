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


