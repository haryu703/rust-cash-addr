use std::result;

use bech32;
use failure::Fail;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Invalid address format: {}", 0)]
    InvalidAddressFormat(String),

    #[fail(display = "Invalid bech32 char: {}", 0)]
    InvalidChar(char),

    #[fail(display = "Mixed case")]
    MixedCase,

    #[fail(display = "Invalid hash size: {}", 0)]
    InvalidHashSize(usize),

    #[fail(display = "Invalid hash size bits: {}", 0)]
    InvalidHashSizeBits(u8),

    #[fail(display = "Hash size mismatch: {}", 0)]
    HashSizeMismatch(usize),

    #[fail(display = "Invalid checksum: {}", 0)]
    InvalidChecksum(String),

    #[fail(display = "Invalid address type bit: {}", 0)]
    InvalidAddressTypeBit(u8),

    #[fail(display = "Bech32 error: {}", 0)]
    Bech32(bech32::Error),
}

impl From<bech32::Error> for Error {
    fn from(err: bech32::Error) -> Error {
        Error::Bech32(err)
    }
}
