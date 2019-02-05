#[macro_use]
extern crate syn;

mod parse_attributes;
mod kv_option;
mod multiple_val;
mod parse_ext;
mod invoke_path;


pub use parse_attributes::ParseAttributes;
pub use parse_ext::ParseStreamExt;

/// Parseable types provided by Synattra
pub mod types {    
    pub use crate::kv_option::KVOption;
    pub use crate::multiple_val::{MultipleValArray, MultipleVal};

    /// Some extra-types not provided by Syn
    pub mod extra {
        pub use crate::invoke_path::InvokePath;
    }

}