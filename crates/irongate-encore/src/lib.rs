#![allow(clippy::new_without_default)]

#[cfg(feature = "encode")]
pub mod encoding;
pub mod errors;
#[cfg(feature = "hash")]
pub mod hash;

#[cfg(feature = "encode")]
pub use encoding::base32hex::*;
#[cfg(feature = "encode")]
pub use encoding::base64::*;
#[cfg(feature = "encode")]
pub use encoding::hex::*;
#[cfg(feature = "encode")]
pub use encoding::html::*;
#[cfg(feature = "encode")]
pub use encoding::url::*;

pub use errors::OperationError;

#[cfg(feature = "hash")]
pub use hash::*;

pub trait Operation {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError>;
}
