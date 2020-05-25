use proc_macro_hack::proc_macro_hack;
use proc_macro::TokenStream;

#[proc_macro_hack]
pub fn query(input: TokenStream) -> TokenStream {
    input
}
