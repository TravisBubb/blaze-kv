/// Represents a storage-related error
#[derive(Debug)]
pub enum StoreError {
    NotFound,
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            StoreError::NotFound => write!(f, "Key not found"),
        }
    }
}

impl std::error::Error for StoreError {}
