use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

mod modification;

#[proc_macro_derive(Modification)]
pub fn derive_modification(input: TokenStream) -> TokenStream {
    todo!()
}