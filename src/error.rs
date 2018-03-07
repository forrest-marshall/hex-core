//! hexadecimal parsing errors.
//!
use std::ops::Deref;
use std::{fmt,error};


/// errors produced during hexadecimal string parsing.
///
#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq,PartialOrd,Ord)]
pub enum ParseHexError {
    /// encountered a non-hexadecimal byte
    NonHexByte,
    /// hex string was not of valid size
    InvalidSize,
}


impl ParseHexError {

    /// get value of error as static string slice.
    ///
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match *self {
            ParseHexError::NonHexByte => "encountered non-hexadecimal byte",
            ParseHexError::InvalidSize => "hex string was not of valid size",
        }
    }
}


impl Deref for ParseHexError {

    type Target = str;

    fn deref(&self) -> &'static Self::Target { self.as_str() }
}


impl fmt::Display for ParseHexError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}


impl error::Error for ParseHexError {

    fn description(&self) -> &'static str { self.as_str() }
}



