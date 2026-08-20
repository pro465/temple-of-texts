use crate::crypto::Key;

use base64::Engine;
use base64::engine::general_purpose::STANDARD_NO_PAD as BASE64;

use std::array::TryFromSliceError;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ParseError {
    ChecksumError,
    IllegalByteError,
    IllegalCharError,
    BytePastEndError,
    EarlyEndError,
}

pub type Result<T> = std::result::Result<T, ParseError>;

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
    let mut bytes = BASE64.decode(code).map_err(|_| ParseError::IllegalCharError)?;
    let cs = bytes.pop().ok_or(ParseError::EarlyEndError)?;

    if cs == checksum(&bytes[..]) {
        Ok(bytes)
    } else {
        Err(ParseError::ChecksumError)
    }
}

/// represents an ability to convert oneself from and into a sequence of bytes
pub(crate) trait SerdeBytes: Sized {
    fn write_bytes(&self, buf: &mut Vec<u8>);

    fn from_bytes_prefix(bytes: &[u8]) -> Result<(Self, &[u8])>;

    fn to_bytes(&self) -> Vec<u8> {
        let mut v = Vec::new();
        self.write_bytes(&mut v);
        v
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let (res, rem) = Self::from_bytes_prefix(bytes)?;
        if rem.is_empty() {
            Ok(res)
        } else {
            Err(ParseError::BytePastEndError)
        }
    }
}

impl SerdeBytes for Key {
    fn write_bytes(&self, buf: &mut Vec<u8>) {
        buf.extend(self.0.to_le_bytes());
        buf.extend(self.1.to_le_bytes());
    }

    fn from_bytes_prefix(bytes: &[u8]) -> Result<(Self, &[u8])> {
        type R = std::result::Result<[u8; 16], TryFromSliceError>;
        let convert_err = |r: R| r.map_err(|_| ParseError::EarlyEndError);

        let first = convert_err(bytes[..16].try_into())?;
        let second = convert_err(bytes[16..32].try_into())?;

        let first = u128::from_le_bytes(first);
        let second = u128::from_le_bytes(second);

        Ok(((first, second), &bytes[32..]))
    }
}

pub trait Serde: Sized {
    fn to_code(&self) -> String;
    fn from_code(code: &str) -> Result<Self>;
}

impl<T: SerdeBytes> Serde for T {
    fn to_code(&self) -> String {
        let bytes = self.to_bytes();
        bytes_to_code(bytes)
    }

    fn from_code(code: &str) -> Result<Self> {
        let bytes = code_to_bytes(code)?;
        Self::from_bytes(&bytes)
    }
}
