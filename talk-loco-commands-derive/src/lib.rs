/*
 * Created on Tue Dec 01 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Ident};

#[proc_macro_derive(BsonData)]
pub fn bson_data(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_bson_data(&ast)
}

fn impl_bson_data(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let method = Ident::new(&name.to_string().to_uppercase(), name.span());

    let gen = quote! {
        impl crate::BsonData for #name {
            
            fn method() -> &'static str {
                &stringify!(#method)
            }

        }
    };
    
    gen.into()
}