use proc_macro2::TokenStream;
use syn::parse::{Parse};
use syn::Result;

/// A trait to parse attributes
pub trait ParseAttributes {
    /// The type of the attributes to parse
    type Type: Parse;

    /// Parse the attribute
    /// 
    /// The default implementation should work out of the box
    fn parse_attributes(attrs: TokenStream) -> Result<Self::Type> {
        Ok(syn::parse2(attrs)?)
    }

    /// The name of the attribute to parse
    /*const*/
    fn fn_attr_name() -> &'static str;
}