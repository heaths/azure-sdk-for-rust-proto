extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, ItemStruct};

#[proc_macro_derive(ClientBuilder, attributes(options))]
pub fn client_builder_derive(item: TokenStream) -> TokenStream {
    let builder = parse_macro_input!(item as ItemStruct);
    let builder_name = builder.ident;
    let mut field_name: Option<&syn::Ident> = None;

    if let syn::Fields::Named(ref fields) = builder.fields {
        for field in fields.named.iter() {
            if let Some(attr) = field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("options"))
            {
                if attr.meta.require_path_only().is_err() {
                    return syn::Error::new(attr.meta.span(), "`options` accepts no arguments")
                        .to_compile_error()
                        .into();
                }
                field_name = field.ident.as_ref();
                break;
            }
            if field.ident.as_ref().is_some_and(|f| f == "options") {
                field_name = field.ident.as_ref();
            }
        }

        if field_name.is_none() {
            return syn::Error::new(
                builder_name.span(),
                "no field named `options` or attributed with `#[options]`",
            )
            .to_compile_error()
            .into();
        }
    } else {
        return syn::Error::new(
            builder_name.span(),
            "ClientBuilder valid only on struct with named fields",
        )
        .into_compile_error()
        .into();
    }

    quote! {
        impl ::azure_core::ClientBuilder for #builder_name {
            fn options(&mut self) -> &mut ::azure_core::ClientOptions {
                &mut self.#field_name
            }
        }

        impl #builder_name {
            pub fn with_context(&mut self, context: ::azure_core::Context) -> &mut Self {
                self.options().context = context;
                self
            }

            pub fn with_retry(&mut self, retry: impl ::core::convert::Into<::azure_core::RetryOptions>) -> &mut Self {
                self.options().retry = retry.into();
                self
            }

            pub fn with_transport(&mut self, transport: impl ::core::convert::Into<::azure_core::TransportOptions>) -> &mut Self {
                self.options().transport = transport.into();
                self
            }
            }
    }
    .into()
}
