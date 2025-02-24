use proc_macro::{Span, TokenStream};
use quote::*;
use syn::*;

#[proc_macro]
pub fn define_ast(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let binding = input.value();
    let name = binding.split(" :").nth(0).unwrap().trim();
    let props = binding
        .split(" :")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| x.trim());

    let ident = syn::Ident::new(name, proc_macro2::Span::call_site());

    let prop_tokens = props.map(|x| syn::Ident::new(x, proc_macro2::Span::call_site()));

    let token = quote! {
        pub struct #ident {
            #(#prop_tokens)*
        }
    };

    TokenStream::from(quote! {#token})
}
