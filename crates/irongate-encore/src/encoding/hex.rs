use bstr::ByteSlice;
use hex;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Operation;
use crate::OperationError;

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HexDecode {
    prefix: Option<String>,
    delimiter: Option<String>,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HexFormat {
    Upper,
    Lower,
}

impl Operation for HexDecode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let mut input = input.to_vec();
        if let Some(p) = &self.prefix {
            input = input.replace(p, "");
        };
        if let Some(d) = &self.delimiter {
            input = input.replace(d, "");
        };
        Ok(hex::decode(input)?)
    }
}

impl HexDecode {
    pub const fn new(prefix: Option<String>, delimiter: Option<String>) -> Self {
        HexDecode { prefix, delimiter }
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HexEncode {
    format: HexFormat,
    prefix: Option<String>,
    delimiter: Option<String>,
}

impl Operation for HexEncode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let hex_string = match self.format {
            HexFormat::Lower => hex::encode(input),
            HexFormat::Upper => hex::encode_upper(input),
        };
        let mut output = vec![];
        let delimiter =
            self.delimiter.clone().and_then(|d| if d.is_empty() { None } else { Some(d) });
        let prefix = self.prefix.clone().and_then(|p| if p.is_empty() { None } else { Some(p) });
        let mut chunks = hex_string.as_bytes().chunks(2).peekable();
        while let Some(chunk) = chunks.next() {
            if let Some(p) = &prefix {
                output.extend_from_slice(p.as_bytes());
            }
            output.extend_from_slice(chunk);
            if let Some(d) = &delimiter {
                if chunks.peek().is_some() {
                    output.extend_from_slice(d.as_bytes());
                }
            }
        }
        Ok(output)
    }
}

impl HexEncode {
    pub const fn new(format: HexFormat, prefix: Option<String>, delimiter: Option<String>) -> Self {
        HexEncode { format, prefix, delimiter }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_decode_no_prefix_no_delimiter() {
        let encoder = HexDecode::new(None, None);
        let actual = encoder.execute("69726f6e67617465".as_bytes()).unwrap();
        let expected = "irongate".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_decode_prefix() {
        let encoder = HexDecode::new(Some("\\x".to_string()), None);
        let actual =
            encoder.execute("\\x69\\x72\\x6f\\x6e\\x67\\x61\\x74\\x65".as_bytes()).unwrap();
        let expected = "irongate".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_decode_delimiter() {
        let encoder = HexDecode::new(None, Some(",".to_string()));
        let actual = encoder.execute("69,72,6f,6e,67,61,74,65".as_bytes()).unwrap();
        let expected = "irongate".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_encode_prefix_upper() {
        let encoder = HexEncode::new(HexFormat::Upper, Some("\\x".to_string()), None);
        let actual = encoder.execute("irongate".as_bytes()).unwrap();
        let expected = "\\x69\\x72\\x6F\\x6E\\x67\\x61\\x74\\x65".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_encode_prefix_lower() {
        let encoder = HexEncode::new(HexFormat::Lower, Some("0x".to_string()), None);
        let actual = encoder.execute("irongate".as_bytes()).unwrap();
        let expected = "0x690x720x6f0x6e0x670x610x740x65".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_encode_delimiter_lower() {
        let encoder = HexEncode::new(HexFormat::Lower, None, Some("\n".to_string()));
        let actual = encoder.execute("irongate".as_bytes()).unwrap();
        let expected = "69\n72\n6f\n6e\n67\n61\n74\n65".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_encode_delimiter_upper() {
        let encoder = HexEncode::new(HexFormat::Upper, None, Some("\n".to_string()));
        let actual = encoder.execute("irongate".as_bytes()).unwrap();
        let expected = "69\n72\n6F\n6E\n67\n61\n74\n65".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }
}
