use std::collections::HashMap;

use crate::error::StoreError;

/// Represents the core storage struct
pub struct Store {
    /// In-memory data store
    db: HashMap<String, Vec<u8>>,
}

impl Store {
    /// Creates a new Store
    pub fn new() -> Store {
        Self { db: HashMap::new() }
    }

    /// Inserts or updates the entry with the provided key and value
    pub fn set(
        &mut self,
        key: String,
        data: Vec<u8>,
    ) -> Result<(), StoreError> {
        self.db.insert(key, data);
        Ok(())
    }

    /// Gets the value for the provided key
    pub fn get(&mut self, key: &str) -> Result<&Vec<u8>, StoreError> {
        if let Some(value) = self.db.get(key) {
            return Ok(value);
        }

        Err(StoreError::NotFound)
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_set_and_get_success() {
        let mut store = Store::new();
        let key = "key1".to_string();
        let value = b"hello world".to_vec();

        store.set(key.clone(), value.clone()).expect("set should succeed");
        let retrieved = store.get(&key).expect("get should succeed");

        assert_eq!(retrieved, &value);
    }

    #[test]
    fn test_store_get_not_found() {
        let mut store = Store::new();
        let key = "missing_key".to_string();

        let result = store.get(&key);
        assert!(matches!(result, Err(StoreError::NotFound)));
    }

    #[test]
    fn test_store_overwrite_existing_key() {
        let mut store = Store::new();
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
        assert_eq!(retrieved, &value2);
    }

    #[test]
    fn test_store_new_is_empty() {
        let store = Store::new();
        assert!(store.db.is_empty(), "new store should be empty");
    }
}
