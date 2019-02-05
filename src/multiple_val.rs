use syn::parse::{Parse, ParseStream};
use syn::Result;
use crate::parse_ext::*;

/// Single or [MultipleA, MultipleB] values.
pub enum MultipleVal<T: Parse> {
    Single(T),
    Multiple(MultipleValArray<T>),
}

impl<T: Parse> MultipleVal<T> {
    pub fn iter(&self) -> Vec<&T> {
        match self {
            MultipleVal::Single(val) => vec![val],
            MultipleVal::Multiple(arr) => arr.values.iter().collect(),
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

pub struct MultipleValArray<T: Parse> {
    pub bracket_token: syn::token::Bracket,
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