extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::{quote};

enum Item {
    Fn(syn::ItemFn)
}

#[proc_macro_attribute]
pub fn myroute(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input : Item = syn::parse(item).unwrap();
    let expanded = quote! {
        #input
    };
    return expanded.into();
}
