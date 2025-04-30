/// Represents a storage-related error
#[derive(Debug)]
pub enum StorageError {
    NotFound,
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            StorageError::NotFound => write!(f, "Key not found"),
        }
    }
}

impl std::error::Error for StorageError {}
