use std::{collections::HashMap, sync::Arc};

use crate::{
    storage::{BoxedStorageFuture, StorageEngine},
    wal::{WalEntry, handle::WalHandle},
};

/// Represents an in-memory storage engine
pub struct MemoryStorageEngine {
    /// In-memory data store
    db: HashMap<String, Vec<u8>>,

    /// WAL handle
    writer: Arc<WalHandle>,
}

impl MemoryStorageEngine {
    pub fn new(writer: Arc<WalHandle>) -> Self {
        Self { db: HashMap::new(), writer }
    }
}

impl StorageEngine for MemoryStorageEngine {
    fn set(
        &mut self,
        key: String,
        data: Vec<u8>,
    ) -> BoxedStorageFuture<'_, ()> {
        Box::pin(async move {
            let entry = WalEntry::Set { key: key.clone(), data: data.clone() };
            self.writer.write(entry).await?;
            self.db.insert(key, data);
            Ok(())
        })
    }

    fn get<'a>(
        &'a self,
        key: &'a str,
    ) -> BoxedStorageFuture<'a, Option<Vec<u8>>> {
        Box::pin(async move { Ok(self.db.get(key).cloned()) })
    }

    fn delete<'a>(
        &'a mut self,
        key: &'a str,
    ) -> BoxedStorageFuture<'a, Option<Vec<u8>>> {
        Box::pin(async move { 
            let entry = WalEntry::Delete { key: key.to_string() };
            self.writer.write(entry).await?;
            Ok(self.db.remove(key)) 
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::wal::{
        BincodeEncoder, DiskWriter, config::WalConfig, spawn_wal_thread,
    };

    use super::*;

    #[tokio::test]
    async fn test_store_set_and_get_success()
    -> Result<(), Box<dyn std::error::Error>> {
        let writer = DiskWriter::new(BincodeEncoder::new(), "./wal")
            .await
            .expect("Failed to initialize DiskWriter");
        let handle =
            Arc::new(spawn_wal_thread(writer, &WalConfig { buffer_size: 16 }));
        let mut store = MemoryStorageEngine::new(handle.clone());
        let key = "key1".to_string();
        let value = b"hello world".to_vec();

        store
            .set(key.clone(), value.clone())
            .await
            .expect("set should succeed");
        let retrieved = store.get(&key).await.expect("get should succeed");

        assert_eq!(retrieved, Some(value));

        let _ = handle.shutdown().await;

        Ok(())
    }

    #[tokio::test]
    async fn test_store_get_not_found() -> Result<(), Box<dyn std::error::Error>>
    {
        let writer = DiskWriter::new(BincodeEncoder::new(), "./wal")
            .await
            .expect("Failed to initialize DiskWriter");
        let handle =
            Arc::new(spawn_wal_thread(writer, &WalConfig { buffer_size: 16 }));
        let store = MemoryStorageEngine::new(handle.clone());
        let key = "missing_key".to_string();

        let result = store.get(&key).await.expect("get should succeed");

        assert_eq!(result, None);

        let _ = handle.shutdown().await;

        Ok(())
    }

    #[tokio::test]
    async fn test_store_delete_success()
    -> Result<(), Box<dyn std::error::Error>> {
        let writer = DiskWriter::new(BincodeEncoder::new(), "./wal")
            .await
            .expect("Failed to initialize DiskWriter");
        let handle =
            Arc::new(spawn_wal_thread(writer, &WalConfig { buffer_size: 16 }));
        let mut store = MemoryStorageEngine::new(handle.clone());
        let key = "key1".to_string();
        let value = b"hello world".to_vec();
        store
            .set(key.clone(), value.clone())
            .await
            .expect("set should succeed");

        let result = store.get(&key).await.expect("get should succeed");
        assert_eq!(result, Some(value.clone()));

        let result = store.delete(&key).await.expect("delete should succeed");
        assert_eq!(result, Some(value.clone()));

        let result = store.get(&key).await.expect("get should succeed");
        assert_eq!(result, None);

        let _ = handle.shutdown().await;

        Ok(())
    }

    #[tokio::test]
    async fn test_store_overwrite_existing_key()
    -> Result<(), Box<dyn std::error::Error>> {
        let writer = DiskWriter::new(BincodeEncoder::new(), "./wal")
            .await
            .expect("Failed to initialize DiskWriter");
        let handle =
            Arc::new(spawn_wal_thread(writer, &WalConfig { buffer_size: 16 }));
        let mut store = MemoryStorageEngine::new(handle.clone());
        let key = "key1".to_string();
        let value1 = b"first".to_vec();
        let value2 = b"second".to_vec();

        store
            .set(key.clone(), value1.clone())
            .await
            .expect("first set should succeed");
        store
            .set(key.clone(), value2.clone())
            .await
            .expect("second set should succeed");

        let retrieved = store.get(&key).await.expect("get should succeed");
        assert_eq!(retrieved, Some(value2));

        let _ = handle.shutdown().await;

        Ok(())
    }

    #[tokio::test]
    async fn test_store_new_is_empty() -> Result<(), Box<dyn std::error::Error>>
    {
        let writer = DiskWriter::new(BincodeEncoder::new(), "./wal")
            .await
            .expect("Failed to initialize DiskWriter");
        let handle =
            Arc::new(spawn_wal_thread(writer, &WalConfig { buffer_size: 16 }));
        let store = MemoryStorageEngine::new(handle.clone());
        assert!(store.db.is_empty(), "new store should be empty");

        let _ = handle.shutdown().await;

        Ok(())
    }
}
