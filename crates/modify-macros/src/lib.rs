use proc_macro::TokenStream;

mod modification;

#[proc_macro_derive(Modification, attributes(modify))]
pub fn derive_modification(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);
    modification::parse(derive_input)
        .map(|output| output.into())
        .unwrap_or_else(|err| err.to_compile_error().into())
}
