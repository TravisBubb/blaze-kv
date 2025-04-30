use std::collections::HashMap;

use crate::storage::StorageEngine;

/// Represents an in-memory storage engine
pub struct MemoryStorageEngine {
    /// In-memory data store
    db: HashMap<String, Vec<u8>>,
}

impl MemoryStorageEngine {
    pub fn new() -> Self {
        Self { db: HashMap::new() }
    }
}

impl Default for MemoryStorageEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl StorageEngine for MemoryStorageEngine {
    fn set(
        &mut self,
        key: String,
        data: Vec<u8>,
    ) -> Result<(), crate::error::StorageError> {
        self.db.insert(key, data);
        Ok(())
    }

    fn get(
        &self,
        key: &str,
    ) -> Result<Option<Vec<u8>>, crate::error::StorageError> {
        Ok(self.db.get(key).cloned())
    }

    fn delete(
        &mut self,
        key: &str,
    ) -> Result<Option<Vec<u8>>, crate::error::StorageError> {
        Ok(self.db.remove(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_set_and_get_success() {
        let mut store = MemoryStorageEngine::new();
        let key = "key1".to_string();
        let value = b"hello world".to_vec();

        store.set(key.clone(), value.clone()).expect("set should succeed");
        let retrieved = store.get(&key).expect("get should succeed");

        assert_eq!(retrieved, Some(value));
    }

    #[test]
    fn test_store_get_not_found() {
        let store = MemoryStorageEngine::new();
        let key = "missing_key".to_string();

        let result = store.get(&key).expect("get should succeed");
        assert_eq!(result, None);
    }

    #[test]
    fn test_store_delete_success() {
        let mut store = MemoryStorageEngine::new();
        let key = "key1".to_string();
        let value = b"hello world".to_vec();
        store.set(key.clone(), value.clone()).expect("set should succeed");

        let result = store.get(&key).expect("get should succeed");
        assert_eq!(result, Some(value.clone()));

        let result = store.delete(&key).expect("delete should succeed");
        assert_eq!(result, Some(value.clone()));

        let result = store.get(&key).expect("get should succeed");
        assert_eq!(result, None);
    }

    #[test]
    fn test_store_overwrite_existing_key() {
        let mut store = MemoryStorageEngine::new();
        let key = "key1".to_string();
        let value1 = b"first".to_vec();
        let value2 = b"second".to_vec();

        store
            .set(key.clone(), value1.clone())
            .expect("first set should succeed");
        store
            .set(key.clone(), value2.clone())
            .expect("second set should succeed");

        let retrieved = store.get(&key).expect("get should succeed");
        assert_eq!(retrieved, Some(value2));
    }

    #[test]
    fn test_store_new_is_empty() {
        let store = MemoryStorageEngine::new();
        assert!(store.db.is_empty(), "new store should be empty");
    }
}
