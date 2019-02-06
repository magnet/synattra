use syn::parse::{Parse, ParseStream};
use syn::Result;
use crate::parse_ext::*;
use auto_enums::auto_enum;
/// Single or [MultipleA, MultipleB] values.
pub enum MultipleVal<T: Parse> {
    /// A Single value
    Single(T),
    /// Multiple values
    Multiple(MultipleValArray<T>),
}

impl<T: Parse> MultipleVal<T> {
    /// Returns an iterator over the values
    #[auto_enum(Iterator)]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        match self {
            MultipleVal::Single(val) => std::iter::once(val),
            MultipleVal::Multiple(arr) => arr.values.iter(),
        }
    }
}

impl<T: Parse> Parse for MultipleVal<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(input
            .try_parse_as(MultipleVal::Single)
            .or_else(|_| input.parse_as(MultipleVal::Multiple))?)
    }
}

/// An array containing several values, [A, B, C]
pub struct MultipleValArray<T: Parse> {
    /// The bracket tocken '[' and ']'
    pub bracket_token: syn::token::Bracket,
    /// The comma-separated values
    pub values: syn::punctuated::Punctuated<T, Token![,]>,
}

impl<T: Parse> Parse for MultipleValArray<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(MultipleValArray {
            bracket_token: bracketed!(content in input),
            values: content.parse_terminated(T::parse)?,
        })
    }
}