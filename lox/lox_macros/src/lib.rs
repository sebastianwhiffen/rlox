use proc_macro::{Span, TokenStream};
use quote::*;
use syn::*;

// #[proc_macro]
// pub fn define_ast(input: TokenStream) -> TokenStream {
//     let expr_trait = quote! {
//         pub trait Expr {
//             fn get_left(&mut self) -> &Box<dyn Expr>;
//             fn get_operator(&mut self) -> &Token;
//             fn get_right(&mut self) -> &Box<dyn Expr>;
//         }
//     };

//     let input = parse_macro_input!(input as LitStr);
//     let binding = input.value();

//     let ast_structs: Vec<_> = binding
//         .split('|')
//         .map(|ast_struct_strings| {
//             let name_props_split = ast_struct_strings.split(':');
//             let ast_struct_name_ident = syn::Ident::new(
//                 name_props_split.nth(0).unwrap(),
//                 proc_macro2::Span::call_site(),
//             );
//             let ast_struct_property_idents = name_props_split
//                 .nth(1)
//                 .unwrap()
//                 .split(',')
//                 .map(|prop| syn::Ident::new(prop, proc_macro2::Span::call_site()));

//             quote! {
//                 pub struct #ast_struct_name_ident {
//                     #(#ast_struct_property_idents)*
//                 }
//             };
//         })
//         .collect();

//         return TokenStream::from(quote! {
//             #expr_trait
//             #(#ast_structs)*
//         });

#[proc_macro]
pub fn define_ast(input: TokenStream) -> TokenStream {
    let expr_trait = quote! {
        pub trait Expr {
        }
    };

    let input = parse_macro_input!(input as LitStr);
    let binding = input.value();

    let ast_structs: Vec<_> = binding
        .split('|')
        .filter(|s| !s.trim().is_empty())
        .map(|ast_struct_string| {
            let mut parts = ast_struct_string.splitn(2, ':');
            let name = parts.next().unwrap().trim();
            let props_str = parts.next().unwrap_or("").trim();
            let ast_struct_name = syn::Ident::new(name, proc_macro2::Span::call_site());

            let properties: Vec<(syn::Ident, syn::Type)> = if props_str.is_empty() {
                vec![]
            } else {
                props_str.split(',')
                    .filter(|p| !p.trim().is_empty())
                    .map(|prop| {
                        let mut prop_parts = prop.splitn(2, ':');
                        let field_name = prop_parts.next().unwrap().trim();
                        let field_type = prop_parts.next().unwrap().trim();
                        let ident = syn::Ident::new(field_name, proc_macro2::Span::call_site());
                        let ty = syn::parse_str::<syn::Type>(field_type).unwrap();
                        (ident, ty)
                    })
                    .collect()
            };

            let (field_names, field_types): (Vec<_>, Vec<_>) = properties.into_iter().unzip();

            quote! {
                pub struct #ast_struct_name {
                    #(pub #field_names: #field_types,)*
                }

                impl Expr for #ast_struct_name
                {
                }
            }
        })
        .collect();
    
    TokenStream::from(quote! {
        #expr_trait
        #(#ast_structs)*
    })
}



    // let name = binding.split(" :").nth(0).unwrap().trim();
    // let props = binding
    //     .split(" :")
    //     .nth(1)
    //     .unwrap()
    //     .split(",")
    //     .map(|x| x.trim());

    // let ident = syn::Ident::new(name, proc_macro2::Span::call_site());

    // let prop_tokens = props.map(|x| syn::Ident::new(x, proc_macro2::Span::call_site()));

    // let token = quote! {
    //     pub struct #ident {
    //         #(#prop_tokens)*
    //     }
    // };

