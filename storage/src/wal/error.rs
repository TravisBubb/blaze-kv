use bincode::error::{DecodeError, EncodeError};

/// Represents an error in the WAL layer
#[derive(Debug)]
pub enum WalError {
    Encode(EncodeError),
    Decode(DecodeError),
}

impl std::fmt::Display for WalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WalError::Encode(e) => write!(f, "Encode error: {}", e),
            WalError::Decode(e) => write!(f, "Decode error: {}", e),
        }
    }
}

impl std::error::Error for WalError {}

impl From<EncodeError> for WalError {
    fn from(err: EncodeError) -> Self {
        WalError::Encode(err)
    }
}

impl From<DecodeError> for WalError {
    fn from(err: DecodeError) -> Self {
        WalError::Decode(err)
    }
}
