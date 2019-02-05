use syn::parse::{Parse, ParseStream};
use syn::Result;

/// A Key Value option.
pub struct KVOption<K: Parse + syn::token::Token, V: Parse> {
    pub key: K,
    pub colon_token: Option<syn::Token![:]>,
    pub eq_token: syn::Token![=],
    pub value: V,
}

impl<K: Parse + syn::token::Token, V: Parse> KVOption<K, V> {
    pub fn peek(input: ParseStream) -> bool {
        K::peek(input.cursor())
    }
}

impl<K: Parse + syn::token::Token, V: Parse> Parse for KVOption<K, V> {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(KVOption {
            key: input.parse()?,
            colon_token: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}
