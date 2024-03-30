use proc_macro::TokenStream;
use quote::quote;

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}

pub(crate) fn test(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input: syn::ItemFn = match syn::parse(item.clone()) {
        Ok(it) => it,
        Err(e) => return token_stream_with_error(item, e),
    };

    let name = input.sig.ident.clone();
    let header = quote! {
       #[::core::prelude::v1::test]
    };

    let result = quote! {
        #header
        fn #name() {
            #input

            ::swtpm_test::run_with(#name);
        }
    };
    result.into()
}
