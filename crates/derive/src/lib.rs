use proc_macro::TokenStream as CompileToken;
use deki_proc::*;
use syn::{parse_macro_input, Data, DeriveInput};

derive_preset::create!{
    hashable    "PartialEq,Eq,Hash,Clone,Copy"
    serde       "Serialize,Deserialize,Clone"
    serde_hash  "Serialize,Deserialize,PartialEq,Eq,Hash,Clone,Copy"
    deref       "drv::Deref,drv::DerefMut"
}

#[proc_macro_derive(Cycle)]
pub fn cycle(input:CompileToken) -> CompileToken {
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


