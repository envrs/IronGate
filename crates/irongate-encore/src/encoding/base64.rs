use data_encoding;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Operation;
use crate::OperationError;

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Base64Format {
    Standard,
    Url,
    Mime,
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Base64Decode {
    format: Base64Format,
    pad: bool,
}

impl Operation for Base64Decode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        Ok(match (&self.format, self.pad) {
            (Base64Format::Standard, false) => data_encoding::BASE64_NOPAD.decode(input)?,
            (Base64Format::Standard, true) => data_encoding::BASE64.decode(input)?,
            (Base64Format::Url, false) => data_encoding::BASE64URL_NOPAD.decode(input)?,
            (Base64Format::Url, true) => data_encoding::BASE64URL.decode(input)?,
            (Base64Format::Mime, _) => data_encoding::BASE64_MIME.decode(input)?,
        })
    }
}

impl Base64Decode {
    pub const fn new(format: Base64Format, pad: bool) -> Self {
        Base64Decode { format, pad }
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Base64Encode {
    format: Base64Format,
    pad: bool,
}

impl Operation for Base64Encode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let encoded = match (&self.format, self.pad) {
            (Base64Format::Standard, false) => data_encoding::BASE64_NOPAD.encode(input),
            (Base64Format::Standard, true) => data_encoding::BASE64.encode(input),
            (Base64Format::Url, false) => data_encoding::BASE64URL_NOPAD.encode(input),
            (Base64Format::Url, true) => data_encoding::BASE64URL.encode(input),
            (Base64Format::Mime, _) => data_encoding::BASE64_MIME.encode(input),
        };
        Ok(encoded.into())
    }
}

impl Base64Encode {
    pub const fn new(format: Base64Format, pad: bool) -> Self {
        Base64Encode { format, pad }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_decode() {
        let encoder = Base64Decode::new(Base64Format::Standard, true);
        let actual = encoder.execute("aXJvbmdhdGU=".as_bytes()).unwrap();
        let expected = "irongate".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn base64_encode() {
        let encoder = Base64Encode::new(Base64Format::Standard, true);
        let actual = encoder.execute("irongate".as_bytes()).unwrap();
        let expected = "aXJvbmdhdGU=".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }
}