use std::path::PathBuf;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

pub trait OnDisk {

    pub fn dir() -> PathBuf { }

    pub fn path() -> PathBuf { }

}

pub trait Configuration: OnDisk {

    fn load() 

}

#[proc_macro_attribute]
pub fn derive_config_attribute() {

}
