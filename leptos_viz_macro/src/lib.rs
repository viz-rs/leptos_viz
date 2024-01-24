use proc_macro::TokenStream;
use server_fn_macro::server_macro_impl;
use syn::__private::ToTokens;

#[proc_macro_attribute]
pub fn server(args: TokenStream, s: TokenStream) -> TokenStream {
    match server_macro_impl(
        args.into(),
        s.into(),
        Some(syn::parse_quote!(::leptos::server_fn)),
        "/api",
        Some(syn::parse_quote!(::viz::Request)),
        Some(syn::parse_quote!(::viz::Response)),
    ) {
        Err(e) => e.to_compile_error().into(),
        Ok(s) => s.to_token_stream().into(),
    }
}
