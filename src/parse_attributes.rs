use proc_macro2::TokenStream;
use syn::parse::{Parse};
use syn::Result;

pub trait ParseAttributes {
    type Type: Parse;

    fn parse_attributes(attrs: TokenStream) -> Result<Self::Type> {
        Ok(syn::parse2(attrs)?)
    }

    /*const*/
    fn fn_attr_name() -> &'static str;
}