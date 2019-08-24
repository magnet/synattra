use syn::parse::{Parse, ParseStream};
use syn::Result;

/// A Key Value option.
pub struct KVOption<K: syn::token::Token, V: Parse> {
    /// The key
    pub key: K,
    /// The equal token    
    pub eq_token: syn::Token![=],
    /// The value
    pub value: V,
}

impl<K: Parse + syn::token::Token, V: Parse> KVOption<K, V> {
    /// Look ahead on the stream if the next token corresponds to the key.Default
    ///
    /// This method does not consume the stream.
    pub fn peek(input: ParseStream) -> bool {
        K::peek(input.cursor())
    }
}

impl<K: Parse + syn::token::Token, V: Parse> Parse for KVOption<K, V> {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(KVOption {
            key: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}
