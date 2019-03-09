#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(unused)]
#![warn(nonstandard_style)]
#![warn(rust_2018_idioms)]

//! cash_addr format implementation inspired by cashaddrjs.

mod error;
mod converter;
mod base32;

pub use error::{Error, Result};
pub use converter::{AddressType, encode, decode};
