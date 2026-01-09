use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShellError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Process execution failed: {0}")]
    ExecutionFailed(String),
}
