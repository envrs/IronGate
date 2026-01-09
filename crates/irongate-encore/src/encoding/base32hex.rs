use data_encoding::BASE32HEX;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Operation;
use crate::OperationError;

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Base32HexDecode {}

impl Operation for Base32HexDecode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        Ok(BASE32HEX.decode(input)?)
    }
}

impl Base32HexDecode {
    pub const fn new() -> Self {
        Base32HexDecode {}
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Base32HexEncode {}

impl Operation for Base32HexEncode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        Ok(BASE32HEX.encode(input).into())
    }
}

impl Base32HexEncode {
    pub const fn new() -> Self {
        Base32HexEncode {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base32hex_decode() {
        let encoder = Base32HexDecode::new();
        let actual = encoder.execute("D5P6URJ7C5Q6A===".as_bytes()).unwrap();
        let expected = "irongate".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn base32hex_encode() {
        let encoder = Base32HexEncode::new();
        let actual = encoder.execute("irongate".as_bytes()).unwrap();
        let expected = "D5P6URJ7C5Q6A===".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }
}
