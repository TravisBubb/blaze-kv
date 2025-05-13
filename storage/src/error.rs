use crate::wal::error::WalError;

/// Represents a storage-related error
#[derive(Debug)]
pub enum StorageError {
    Wal(WalError),
    NotFound,
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::Wal(e) => write!(f, "WAL error: {}", e),
            StorageError::NotFound => write!(f, "Key not found"),
        }
    }
}

impl std::error::Error for StorageError {}

impl From<WalError> for StorageError {
    fn from(value: WalError) -> Self {
        Self::Wal(value)
    }
}
