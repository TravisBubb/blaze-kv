use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

use super::{encoder::WalEncoder, entry::WalEntry, error::WalError};

/// Represents an item that can write WAL entries
pub trait WalWriter {
    /// Appends an entry to the WAL
    fn append(&mut self, entry: &WalEntry) -> Result<(), WalError>;

    /// Flushes the in-memory buffer of file writes
    fn flush(&mut self) -> Result<(), WalError>;

    /// Flushes the OS file write buffer (hardware cache)
    fn sync(&mut self) -> Result<(), WalError>;
}

/// Represents a WalWriter that writes to a file on disk
pub struct DiskWriter<E: WalEncoder> {
    encoder: E,
    file: File,
}

impl<E: WalEncoder> DiskWriter<E> {
    pub fn new(encoder: E, path: impl AsRef<Path>) -> Result<Self, WalError> {
        Ok(Self {
            encoder,
            file: OpenOptions::new().create(true).append(true).open(path)?,
        })
    }
}

impl<E: WalEncoder> WalWriter for DiskWriter<E> {
    fn append(&mut self, entry: &WalEntry) -> Result<(), WalError> {
        let bytes = self.encoder.encode(entry)?;
        self.file.write_all(&bytes)?;
        Ok(())
    }

    fn flush(&mut self) -> Result<(), WalError> {
        todo!()
    }

    fn sync(&mut self) -> Result<(), WalError> {
        todo!()
    }
}
