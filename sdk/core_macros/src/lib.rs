use proc_macro::TokenStream;
use quote::quote;
use std::borrow::Borrow;
use syn::{
    parse::{Nothing, Parser},
    parse_macro_input, ItemStruct,
};

#[proc_macro_attribute]
pub fn client_builder(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(attrs as Nothing);
    let mut builder = parse_macro_input!(item as ItemStruct);
    let builder_name = builder.ident.borrow();
    let mut field_name: Option<syn::Ident> = None;

    if let syn::Fields::Named(ref fields) = builder.fields {
        for field in fields.named.iter() {
            // BUGBUG: No way to define helper attributes for attribute macros?
            // if let Some(attr) = field
            //     .attrs
            //     .iter()
            //     .find(|attr| attr.path().is_ident("options"))
            // {
            //     if attr.meta.require_path_only().is_err() {
            //         return syn::Error::new(attr.meta.span(), "`options` accepts no arguments")
            //             .to_compile_error()
            //             .into();
            //     }
            //     field_name = field.ident.clone();
            //     break;
            // }
            if field.ident.as_ref().is_some_and(|f| f == "options") {
                field_name = field.ident.clone();
            }
        }
    }

    if field_name.is_none() {
        let field = syn::Field::parse_named
            .parse2(quote! { __options: ::azure_core::ClientOptions })
            .unwrap();
        if let syn::Fields::Named(ref mut fields) = builder.fields {
            fields.named.push(field);
            field_name = fields.named.last().and_then(|f| f.ident.clone());
        }
    }

    quote! {
        #builder

        impl ::azure_core::ClientBuilder for #builder_name {
            fn options(&mut self) -> &mut ::azure_core::ClientOptions {
                &mut self.#field_name
            }
        }

        impl ::std::convert::From<#builder_name> for ::azure_core::ClientOptions {
            fn from(value: #builder_name) -> Self {
                value.#field_name
            }
        }

        impl ::std::convert::From<&#builder_name> for ::azure_core::ClientOptions {
            fn from(value: &#builder_name) -> Self {
                value.#field_name.clone()
            }
        }
    }
    .into()
}
