mod new;

use proc_macro::TokenStream;
use crate::new::new_macro;

#[proc_macro_derive(New, attributes(default))]
pub fn new_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    new_macro(&ast).into()
}
