use bech32::u5;
use super::error::{Error, Result};

pub fn encode(data: &[u5]) -> String {
    // https://github.com/rust-bitcoin/rust-bech32/blob/master/src/lib.rs
    const CHARSET: [char; 32] = [
        'q','p','z','r','y','9','x','8',
        'g','f','2','t','v','d','w','0',
        's','3','j','n','5','4','k','h',
        'c','e','6','m','u','a','7','l'
    ];

    data.iter().map(|p| CHARSET[p.to_u8() as usize]).collect()
}

fn is_lower(c: char) -> Option<bool> {
    if c.is_ascii_digit() { None } else { Some(c.is_ascii_lowercase()) }
}

pub fn decode(data: &str) -> Result<Vec<u5>> {
    // https://github.com/rust-bitcoin/rust-bech32/blob/master/src/lib.rs
    const CHARSET_REV: [i8; 128] = [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        15, -1, 10, 17, 21, 20, 26, 30,  7,  5, -1, -1, -1, -1, -1, -1,
        -1, 29, -1, 24, 13, 25,  9,  8, 23, -1, 18, 22, 31, 27, 19, -1,
        1,  0,  3, 16, 11, 28, 12, 14,  6,  4,  2, -1, -1, -1, -1, -1,
        -1, 29, -1, 24, 13, 25,  9,  8, 23, -1, 18, 22, 31, 27, 19, -1,
        1,  0,  3, 16, 11, 28, 12, 14,  6,  4,  2, -1, -1, -1, -1, -1
    ];

    if data.is_empty() || !data.is_ascii() {
        return Err(Error::InvalidAddressFormat(data.to_string()));
    }

    let lower = data.chars().find_map(is_lower);

    data.chars().map(|c| {
        if let Some(case) = is_lower(c) {
            if case != lower.ok_or_else(|| Error::InvalidChar(c))? {
                return Err(Error::MixedCase);
            }
        }

        let num = CHARSET_REV[c as usize];
        if num < 0 || num > 31 {
            return Err(Error::InvalidChar(c));
        }

        Ok(u5::try_from_u8(num as u8)?)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_str = "qr6m7j9njldwwzlg9v7v53unlr4jkmx6eylep8ekg2";

        let decoded = decode(test_str).unwrap();
        let encoded = encode(&decoded);

        assert_eq!(encoded, test_str);
    }
}
