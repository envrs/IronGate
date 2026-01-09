use core::str::Utf8Error;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OperationError {
    #[error("Utf8 decode error")]
    DecodeUtf8Error,

    #[error("Decode error: {0}")]
    DecodeError(String),

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error("Unknown error")]
    Unknown,
}

impl From<Utf8Error> for OperationError {
    fn from(_value: Utf8Error) -> Self {
        OperationError::DecodeUtf8Error
    }
}

impl From<FromUtf8Error> for OperationError {
    fn from(_value: FromUtf8Error) -> Self {
        OperationError::DecodeUtf8Error
    }
}

#[cfg(feature = "encode")]
impl From<hex::FromHexError> for OperationError {
    fn from(_value: hex::FromHexError) -> Self {
        OperationError::DecodeError("Invalid Hex input".to_string())
    }
}

#[cfg(feature = "encode")]
impl From<data_encoding::DecodeError> for OperationError {
    fn from(value: data_encoding::DecodeError) -> Self {
        OperationError::DecodeError(value.to_string())
    }
}