use bech32::{u5, CheckBase32};

use super::error::{Error, Result};
use super::base32;

/// Address type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AddressType {
    /// P2PKH address.
    P2PKH = 0,
    /// P2SH address.
    P2SH = 8,
}

const SEPARATOR: char = ':';

/// Encode hash to cash_addr format.
/// # Arguments
/// * `prefix` - address prefix.
/// * `address_type` - address type.
/// * `hash` - hashed publickey.
/// # Returns
/// * cash_addr format address.
/// # Example
/// ```
/// use cash_addr::{encode, AddressType};
/// 
/// let data = [0xF5, 0xBF, 0x48, 0xB3, 0x97, 0xDA, 0xE7, 0x0B, 0xE8, 0x2B, 0x3C, 0xCA, 0x47, 0x93, 0xF8, 0xEB, 0x2B, 0x6C, 0xDA, 0xC9];
/// let prefix = "bitcoincash";
/// let addr_type = AddressType::P2PKH;
/// 
/// let address = encode(prefix, addr_type, &data).unwrap();
/// assert_eq!(address, "bitcoincash:qr6m7j9njldwwzlg9v7v53unlr4jkmx6eylep8ekg2");
/// ```
pub fn encode(prefix: &str, address_type: AddressType, hash: &[u8]) -> Result<String> {
    let prefix_data = parse_prefix(prefix)?;

    let address_type = address_type as u8;
    let version_byte = address_type + get_hash_size_bits(hash)?;

    let payload = [&[version_byte], hash].concat();
    let payload_data = bech32::convert_bits(&payload, 8, 5, true)?.check_base32()?;

    let checksum_data = [&prefix_data[..], &payload_data[..], &[0; 8].check_base32()?].concat();
    let checksum = polymod(&checksum_data)?;
    let payload = [payload_data, checksum].concat();

    let payload = base32::encode(&payload);

    Ok(format!("{}{}{}", prefix, SEPARATOR, payload))
}

/// Decode cash_addr.
/// # Arguments
/// * `address` - cash_addr format address
/// # Resurns
/// * Prefix.
/// * Address type.
/// * hashed publickey.
/// # Example
/// ```
/// use cash_addr::{decode, AddressType};
/// 
/// let data = [0xF5, 0xBF, 0x48, 0xB3, 0x97, 0xDA, 0xE7, 0x0B, 0xE8, 0x2B, 0x3C, 0xCA, 0x47, 0x93, 0xF8, 0xEB, 0x2B, 0x6C, 0xDA, 0xC9];
/// let address = "bitcoincash:qr6m7j9njldwwzlg9v7v53unlr4jkmx6eylep8ekg2";
/// let (prefix, addr_type, hash) = decode(address).unwrap();
/// 
/// assert_eq!(prefix, "bitcoincash");
/// assert_eq!(addr_type, AddressType::P2PKH);
/// assert_eq!(hash, data);
/// ```
pub fn decode(address: &str) -> Result<(String, AddressType, Vec<u8>)> {
    let pieces: Vec<&str> = address.split(SEPARATOR).collect();

    if pieces.len() != 2 {
        return Err(Error::InvalidAddressFormat(address.to_string()));
    }

    let prefix = pieces[0];
    let payload = base32::decode(pieces[1])?;
    if payload.len() < (8 + 1 + 1) { // checksum + version + hash
        return Err(Error::InvalidAddressFormat(address.to_string()));
    }
    if !validate_checksum(prefix, &payload)? {
        return Err(Error::InvalidChecksum(address.to_string()));
    }

    let payload = &payload[..payload.len() - 8]; // remove checksum
    let payload_data = bech32::convert_bits(&payload, 5, 8, false)?;
    let version_byte = payload_data[0];
    let hash = &payload_data[1..];
    if get_hash_size(version_byte)? != hash.len() {
        return Err(Error::HashSizeMismatch(hash.len()));
    }
    let address_type = get_address_type(version_byte)?;

    Ok((prefix.to_string(), address_type, hash.to_vec()))
}

fn parse_prefix(prefix: &str) -> Result<Vec<u5>> {
    let mut result: Vec<u5> = Vec::with_capacity(prefix.len() + 1);

    for c in prefix.chars() {
        let c = c as u8 & 31;
        result.push(u5::try_from_u8(c)?);
    }
    result.push(u5::try_from_u8(0)?);

    Ok(result)
}

fn get_hash_size_bits(hash: &[u8]) -> Result<u8> {
    match hash.len() * 8 {
        160 => Ok(0),
        192 => Ok(1),
        224 => Ok(2),
        256 => Ok(3),
        320 => Ok(4),
        384 => Ok(5),
        448 => Ok(6),
        512 => Ok(7),
        e   => Err(Error::InvalidHashSize(e))
    }
}

fn get_hash_size(version_byte: u8) -> Result<usize> {
    let len_bits = match version_byte & 7 {
        0 => Ok(160),
        1 => Ok(192),
        2 => Ok(224),
        3 => Ok(256),
        4 => Ok(320),
        5 => Ok(384),
        6 => Ok(448),
        7 => Ok(512),
        e => Err(Error::InvalidHashSizeBits(e))
    }?;

    Ok(len_bits / 8)
}

fn get_address_type(version_byte: u8) -> Result<AddressType> {
    match version_byte & 8 {
        0 => Ok(AddressType::P2PKH),
        8 => Ok(AddressType::P2SH),
        e => Err(Error::InvalidAddressTypeBit(e)),
    }
}

fn polymod(data: &[u5]) -> Result<Vec<u5>> {
    let mut c = 1;
    for d in data {
        let c0 = c >> 35;
        c = ((c & 0x0007_ffff_ffff) << 5) ^ u64::from(d.to_u8());

        const GENERATOR: [u64; 5] = [
            0x0098_f2bc_8e61,
            0x0079_b76d_99e2,
            0x00f3_3e5f_b3c4,
            0x00ae_2eab_e2a8,
            0x001e_4f43_e470
        ];
        for (i, g) in GENERATOR.iter().enumerate() {
            if (c0 & (1 << i)) != 0 {
                c ^= g;
            }
        }
    }
    let mut c = c ^ 1;

    // convert to u5
    let mut result = [0; 8].check_base32()?;
    for v in result.iter_mut().rev() {
        *v = u5::try_from_u8((c & 31) as u8)?;
        c >>= 5;
    }

    Ok(result)
}

fn validate_checksum(prefix: &str, payload: &[u5]) -> Result<bool> {
    let prefix_data = parse_prefix(prefix)?;
    let data = [&prefix_data, payload].concat();

    Ok(polymod(&data)?.iter().all(|&el| el.to_u8() == 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_test() {
        let data = [0xF5, 0xBF, 0x48, 0xB3, 0x97, 0xDA, 0xE7, 0x0B, 0xE8, 0x2B, 0x3C, 0xCA, 0x47, 0x93, 0xF8, 0xEB, 0x2B, 0x6C, 0xDA, 0xC9];

        let prefix = "bitcoincash";
        let addr_type = AddressType::P2PKH;
        let address = encode(prefix, addr_type, &data).unwrap();
        assert_eq!(address, "bitcoincash:qr6m7j9njldwwzlg9v7v53unlr4jkmx6eylep8ekg2");

        let prefix = "bchtest";
        let addr_type = AddressType::P2SH;
        let address = encode(prefix, addr_type, &data).unwrap();
        assert_eq!(address, "bchtest:pr6m7j9njldwwzlg9v7v53unlr4jkmx6eyvwc0uz5t");
    }

    #[test]
    fn decode_test() {
        let data = [0xF5, 0xBF, 0x48, 0xB3, 0x97, 0xDA, 0xE7, 0x0B, 0xE8, 0x2B, 0x3C, 0xCA, 0x47, 0x93, 0xF8, 0xEB, 0x2B, 0x6C, 0xDA, 0xC9];

        let address = "bitcoincash:qr6m7j9njldwwzlg9v7v53unlr4jkmx6eylep8ekg2";
        let (prefix, addr_type, hash) = decode(address).unwrap();
        assert_eq!(prefix, "bitcoincash");
        assert_eq!(addr_type, AddressType::P2PKH);
        assert_eq!(hash, data);

        let address = "bchtest:pr6m7j9njldwwzlg9v7v53unlr4jkmx6eyvwc0uz5t";
        let (prefix, addr_type, hash) = decode(address).unwrap();
        assert_eq!(prefix, "bchtest");
        assert_eq!(addr_type, AddressType::P2SH);
        assert_eq!(hash, data);
    }
}
