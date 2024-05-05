use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Service)]
pub fn derive_service(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let struct_name = input.ident;
    let struct_name_str = struct_name.to_string();
    let event_enum_name = Ident::new(&format!("{struct_name_str}Event"), Span::call_site());

    let expanded = quote!(
        pub enum #event_enum_name {
            MyEvent
        }
    );

    expanded.into()
}
