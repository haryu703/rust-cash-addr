use std::result;

use bech32;
use failure::Fail;

/// Alias of `Result` used by cash_addr.
pub type Result<T> = result::Result<T, Error>;

/// Errors
#[derive(Debug, Fail)]
pub enum Error {
    /// Invalid address format.
    /// # Arguments
    /// * Address.
    #[fail(display = "Invalid address format: {}", 0)]
    InvalidAddressFormat(String),

    /// Invalid bech32 char.
    /// # Arguments
    /// * Character.
    #[fail(display = "Invalid bech32 char: {}", 0)]
    InvalidChar(char),

    /// Mixed case.
    #[fail(display = "Mixed case")]
    MixedCase,

    /// Invalid hash size.
    /// # Arguments
    /// * Hash length
    #[fail(display = "Invalid hash size: {}", 0)]
    InvalidHashSize(usize),

    /// Invalid hash size bits.
    /// # Arguments
    /// * Hash size bits.
    #[fail(display = "Invalid hash size bits: {}", 0)]
    InvalidHashSizeBits(u8),

    /// Hash size mismatch.
    /// # Arguments
    /// * Hash length.
    #[fail(display = "Hash size mismatch: {}", 0)]
    HashSizeMismatch(usize),

    /// Invalid checksum
    /// # Arguments
    /// * Address.
    #[fail(display = "Invalid checksum: {}", 0)]
    InvalidChecksum(String),

    /// undefined address type bit
    /// # Arguments
    /// * Type bit.
    #[fail(display = "Invalid address type bit: {}", 0)]
    InvalidAddressTypeBit(u8),

    /// bech32 library's error.
    /// # Arguments
    /// * Error.
    #[fail(display = "Bech32 error: {}", 0)]
    Bech32(bech32::Error),
}

impl From<bech32::Error> for Error {
    fn from(err: bech32::Error) -> Error {
        Error::Bech32(err)
    }
}
