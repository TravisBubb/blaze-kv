use crate::error::StorageError;

/// Represents a key-value storage engine
pub trait StorageEngine {
    fn set(&mut self, key: String, data: Vec<u8>) -> Result<(), StorageError>;
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError>;
    fn delete(&mut self, key: &str) -> Result<Option<Vec<u8>>, StorageError>;
}
