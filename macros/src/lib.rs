use proc_macro::TokenStream;
use syn::spanned::Spanned as _;

#[proc_macro_attribute]
pub fn test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.sig.output;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;

    if input.sig.asyncness.is_none() {
        return TokenStream::from(quote::quote_spanned! { input.span() =>
            compile_error!("the async keyword is missing from the function declaration"),
        });
    }

    let result = quote::quote! {
        #[::core::prelude::v1::test]
        #(#attrs)*
        #vis fn #name() #ret {
            ::async_dispatcher::set_dispatcher(::async_dispatcher::current_thread_dispatcher());
            ::async_dispatcher::block_on(async { #body })
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.sig.output;
    let body = &input.block;
    let attrs = &input.attrs;

    if input.sig.asyncness.is_none() {
        return TokenStream::from(quote::quote_spanned! { input.span() =>
            compile_error!("the async keyword is missing from the function declaration"),
        });
    }

    let result = quote::quote! {
        #(#attrs)*
        fn main() #ret {
            ::async_dispatcher::set_dispatcher(::async_dispatcher::current_thread_dispatcher());
            ::async_dispatcher::block_on(async { #body })
        }
    };

    result.into()
}
