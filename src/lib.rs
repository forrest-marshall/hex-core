//! simple hexadecimal conversion helpers.
//!
//! the goal of the `hex-core` crate is to provide a set of common utilties
//! upon which higher-level hexadecimal systems can easily be built.
//!
#![warn(missing_docs)]
pub mod error;
pub mod util;
pub mod byte;
pub mod hex;


pub use error::ParseHexError;

pub use util::{
    as_str,
    as_str_upper,
};

pub use hex::{
    from,
    into,
    into_upper,
};


