use proc_macro2::{self, TokenStream, parse::Parse};
use quote::quote;
use syn::{parse_macro_input, Fields, DataEnum, DataUnion, DeriveInput, FieldsNamed};


pub trait Saves<Dbx> {

    fn save(&self) -> Option<Dbx>;

}

#[proc_macro_derive]
pub fn derive_saves(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let inp = parse_macro_inputk!(input as DeriveInput);

}
