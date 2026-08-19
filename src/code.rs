use crate::crypto::Key;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;

use std::array::TryFromSliceError;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InvalidCodeError;

pub type Result<T> = std::result::Result<T, InvalidCodeError>;

fn checksum(bytes: &[u8]) -> u8 {
    let mut res = 0u8;
    for &b in bytes {
        res = res.wrapping_add(b);
    }
    res
}

pub(crate) fn bytes_to_code(mut bytes: Vec<u8>) -> String {
    bytes.push(checksum(&bytes[..]));
    BASE64.encode(bytes)
}

pub(crate) fn code_to_bytes(code: &str) -> Result<Vec<u8>> {
    let mut bytes = BASE64.decode(code).map_err(|_| InvalidCodeError)?;
    let cs = bytes.pop().ok_or(InvalidCodeError)?;

    if cs == checksum(&bytes[..]) {
        Ok(bytes)
    } else {
        Err(InvalidCodeError)
    }
}

/// represents an ability to convert oneself from and into a sequence of bytes
pub(crate) trait Serde: Sized {
    fn write_bytes(&self, buf: &mut Vec<u8>);

    fn from_bytes_prefix(bytes: &[u8]) -> Result<(Self, &[u8])>;

    fn into_bytes(&self) -> Vec<u8> {
        let mut v = Vec::new();
        self.write_bytes(&mut v);
        v
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let (res, rem) = Self::from_bytes_prefix(bytes)?;
        if rem.is_empty() {
            Ok(res)
        } else {
            Err(InvalidCodeError)
        }
    }
}

impl Serde for Key {
    fn write_bytes(&self, buf: &mut Vec<u8>) {
        buf.extend(self.0.to_le_bytes());
        buf.extend(self.1.to_le_bytes());
    }

    fn from_bytes_prefix(bytes: &[u8]) -> Result<(Self, &[u8])> {
        type R = std::result::Result<[u8; 16], TryFromSliceError>;
        let convert_err = |r: R| r.map_err(|_| InvalidCodeError);

        let first = convert_err(bytes[..16].try_into())?;
        let second = convert_err(bytes[16..32].try_into())?;

        let first = u128::from_le_bytes(first);
        let second = u128::from_le_bytes(second);

        Ok(((first, second), &bytes[32..]))
    }
}
