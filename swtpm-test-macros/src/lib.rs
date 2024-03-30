use proc_macro::TokenStream;

mod entry;

#[proc_macro_attribute]
pub fn test(args: TokenStream, item: TokenStream) -> TokenStream {
    entry::test(args, item)
}
