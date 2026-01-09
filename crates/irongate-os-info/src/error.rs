use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("System call failed: {0}")]
    SystemCall(String),

    #[error("Failed to parse OS information: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
