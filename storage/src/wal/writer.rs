use std::{path::Path, pin::Pin};

use tokio::{
    fs::{File, OpenOptions},
    io::AsyncWriteExt,
};

use super::{encoder::WalEncoder, entry::WalEntry, error::WalError};

/// Represents an item that can write WAL entries
pub trait WalWriter {
    /// Appends an entry to the WAL
    fn append<'a>(
        &'a mut self,
        entry: &'a WalEntry,
    ) -> Pin<Box<dyn Future<Output = Result<(), WalError>> + Send + 'a>>;

    /// Flushes the in-memory buffer of file writes
    fn flush<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<(), WalError>> + Send + 'a>>;

    /// Flushes the OS file write buffer (hardware cache)
    fn sync<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<(), WalError>> + Send + 'a>>;
}

/// Represents a WalWriter that writes to a file on disk
pub struct DiskWriter<E: WalEncoder> {
    encoder: E,
    file: File,
}

impl<E: WalEncoder> DiskWriter<E> {
    pub async fn new(
        encoder: E,
        path: impl AsRef<Path>,
    ) -> Result<Self, WalError> {
        Ok(Self {
            encoder,
            file: OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .await?,
        })
    }
}

impl<E: WalEncoder + Send> WalWriter for DiskWriter<E> {
    fn append<'a>(
        &'a mut self,
        entry: &'a WalEntry,
    ) -> Pin<Box<dyn Future<Output = Result<(), WalError>> + Send + 'a>> {
        Box::pin(async move {
            let bytes = self.encoder.encode(entry)?;
            self.file.write_all(&bytes).await?;
            Ok(())
        })
    }

    fn flush<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<(), WalError>> + Send + 'a>> {
        Box::pin(async move {
            self.file.flush().await?;
            Ok(())
        })
    }

    fn sync<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<(), WalError>> + Send + 'a>> {
        Box::pin(async move {
            self.file.sync_all().await?;
            Ok(())
        })
    }
}
