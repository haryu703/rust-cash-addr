#![warn(missing_docs)]

//! cash_addr format implementation inspired by cashaddrjs.

mod error;
mod converter;
mod base32;

pub use error::{Error, Result};
pub use converter::{AddressType, encode, decode};
