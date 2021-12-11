#[macro_use]
mod models;
#[macro_use]
mod data;
#[macro_use]
mod config;
#[macro_use]
mod util;
#[macro_use]
mod save;
#[macro_use]

use proc_macro::TokenStream;
use proc_macro2::{self, TokenStream, parse::Parse};
use quote::quote;
use syn::{parse_macro_input, Fields, DataEnum, DataUnion, DeriveInput, FieldsNamed, FieldsUnnamed};
use proc_macro_error::*;
use proc_macro_error::{abort, diagnostic, emit_error};

// triple value == fact???

#[proc_macro_derive(Fact, attributes())]
#[proc_macro_error]
pub fn derive_hello(input: TokenStream) -> TokenStream { 
    // Parse Phase
    let derive_input = parse_macro_input!(input as DeriveInput);
    let ident = &derive_input.ident;
    let name = derive_input.ident.to_string();
    
    // Generate Phase
    (quote! {
        impl Hello for #ident {
            fn hello() {
                println!("Hello from {}", #name);
            }
        }
    }).into()
}


//- ----


trait Saves<Dbx> {

    fn save(&self) -> Option<Dbx>;

}


#[proc_macro_derive(Saves, attributes(helper))]
pub fn derive_saves(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput ( ident, data, .. ) = parse_macro_input!(input);

    let desc = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let idents = named.iter().map(|f| &f.ident);
                format!("a struct with named fields {}",
                    quote! {#(#idents)*}
                )
            },
            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let num_fields = unnamed.iter().count();
                format!("a struct with {} uinnamed fields", num_fields)
            },
            syn::Fields::Unit => format!("unit struct"),
        },
        syn::Data::Enum(DataEnum { variants, ..}) => {
            let vs = variants.iter().map(|v| &v.ident);
            format!("an enum with these variants: {}", quote! {#(#vs),*})
        },
        syn::Data::Union(DataUnion { fields: FieldsNamed { named, .. }}) => {
            let idents = named.iter().map(|f| &f.ident);
            format!("a union with these named fields: {}", quote! {#(#idents),*})
        }
    };
    let outp = quote! { 
        impl #ident {
            fn desc() {
                println!("{} is {}", stringify!(#ident), #description);
            }
        }
    };
    return op.into()


}
