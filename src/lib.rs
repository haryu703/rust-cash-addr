#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(unused)]
#![warn(nonstandard_style)]
#![warn(rust_2018_idioms)]

//! cash_addr format implementation inspired by cashaddrjs.
//! # Example
//! ```rust
//! use cash_addr::{encode, decode, AddressType};
//! 
//! let data = [0xF5, 0xBF, 0x48, 0xB3, 0x97, 0xDA, 0xE7, 0x0B, 0xE8, 0x2B, 0x3C, 0xCA, 0x47, 0x93, 0xF8, 0xEB, 0x2B, 0x6C, 0xDA, 0xC9];
//! let prefix = "bitcoincash";
//! let addr_type = AddressType::P2PKH;
//! 
//! let address = encode(prefix, addr_type, &data).unwrap();
//! assert_eq!(address, "bitcoincash:qr6m7j9njldwwzlg9v7v53unlr4jkmx6eylep8ekg2");
//! 
//! let (prefix, addr_type, hash) = decode(&address).unwrap();
//! assert_eq!(prefix, "bitcoincash");
//! assert_eq!(addr_type, AddressType::P2PKH);
//! assert_eq!(hash, data);
//! ```

mod error;
mod converter;
mod base32;

pub use error::{Error, Result};
pub use converter::{AddressType, encode, decode};
