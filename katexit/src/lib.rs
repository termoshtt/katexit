use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_attribute]
pub fn katexit(attr: TokenStream, item: TokenStream) -> TokenStream {
    katexit2(attr.into(), item.into()).into()
}

fn katexit2(_attr: TokenStream2, _item: TokenStream2) -> TokenStream2 {
    todo!()
}
