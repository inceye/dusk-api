use proc_macro::TokenStream;

#[proc_macro]
pub fn register_callable (
    input: TokenStream,
) -> TokenStream {
    input
}
