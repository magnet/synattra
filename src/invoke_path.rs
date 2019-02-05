use syn::parse::{Parse, ParseStream};
use syn::Result;

/// An invocation path that may be `bar::foo` or `bar::foo!`
pub struct InvokePath {
    pub path: syn::Path,
    pub bang: Option<syn::Token![!]>,
}

impl Parse for InvokePath {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(InvokePath {
            path: input.parse()?,
            bang: input.parse()?,
        })
    }
}
