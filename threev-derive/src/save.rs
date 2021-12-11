//! Allows for deriive attribute "Saves" to be placed on an entity
//! link fact (sequence) etc. of your choice
#[macro_use]
use proc_macro2::{self, TokenStream, parse::Parse};
use quote::quote;
use syn::{parse_macro_input, Fields, DataEnum, DataUnion, DeriveInput, FieldsNamed};


pub trait Saves<Dbx> {

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
            syn::Fields::Unnamedk(FieldsUnnamed { unnamed, .. }) => {
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
    outp.into()
}
