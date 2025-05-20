use thiserror::Error;

/// Unified error type for the crate
#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Topology error: {0}")]
    Topology(String),

    #[error("Numeric error: {0}")]
    Numeric(String),

    #[error("Unsupported operation: {0}")]
    Unsupported(String),
}

/// Simplified Result type
pub type Result<T> = std::result::Result<T, Error>;