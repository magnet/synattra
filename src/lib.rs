//! # A Syn Attribute Parser Toolkit
//!
//! Synattra extends Syn and provides structures to easily parse custom attributes.
//!
//! Notably, Synattra provides a `KVOption<K, V>` type that allows parsing attributes in the form `key = value` where `key` can be any token or custom keyword (including Rust keywords!) and value any type that can be parsed from a `TokenStream`. Synattra was extracted from the [Metered project](https://github.com/magnet/metered-rs), please check it for actual usage.
//!
//! Synattra also supports single or multiple values, that can take the shape of `Foo` or `[Foo, Bar]`.
//!
//! Finally Synattra provides some extra types, such as `InvokePath` which represents any invocation handle, macro or not (e.g `foo` or `println!`).
//!
//! By reusing Syn's design, Synattra parsers are very robust and when they compile, they usually work :-).

#![deny(missing_docs)]
#![deny(warnings)]

#[macro_use]
extern crate syn;

mod invoke_path;
mod kv_option;
mod multiple_val;
mod parse_attributes;
mod parse_ext;

pub use parse_attributes::ParseAttributes;
pub use parse_ext::ParseStreamExt;

/// Parseable types provided by Synattra
pub mod types {
    pub use crate::kv_option::KVOption;
    pub use crate::multiple_val::{MultipleVal, MultipleValArray};

    /// Some extra-types not provided by Syn
    pub mod extra {
        pub use crate::invoke_path::InvokePath;
    }
}
