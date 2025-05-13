use std::pin::Pin;

use crate::error::StorageError;

pub type BoxedStorageFuture<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, StorageError>> + Send + 'a>>;

/// Represents a key-value storage engine
pub trait StorageEngine {
    fn set(
        &mut self,
        key: String,
        data: Vec<u8>,
    ) -> BoxedStorageFuture<'_, ()>;
    fn get<'a>(
        &'a self,
        key: &'a str,
    ) -> BoxedStorageFuture<'a, Option<Vec<u8>>>;
    fn delete<'a>(
        &'a mut self,
        key: &'a str,
    ) -> BoxedStorageFuture<'a, Option<Vec<u8>>>;
}
