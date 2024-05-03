use proc_macro::TokenStream;

#[proc_macro_derive(Service)]
pub fn derive_service(item: TokenStream) -> TokenStream {
    "".parse().unwrap()
}