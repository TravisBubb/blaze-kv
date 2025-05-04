use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use super::{encoder::WalEncoder, entry::WalEntry, error::WalError};

/// Represents an item that can write WAL entries
pub trait WalWriter {
    /// Appends an entry to the WAL
    fn append(
        &mut self,
        entry: &WalEntry,
        encoder: Box<dyn WalEncoder>,
    ) -> Result<(), WalError>;

    /// Flushes the in-memory buffer of file writes
    fn flush() -> Result<(), WalError>;

    /// Flushes the OS file write buffer (hardware cache)
    fn sync() -> Result<(), WalError>;
}

/// Represents a WalWriter that writes to a file on disk
pub struct DiskWriter {
    file: File,
}

impl DiskWriter {
    pub fn new() -> Self {
        Self {
            file: OpenOptions::new()
                .create(true)
                .append(true)
                .open("blaze.wal")
                .expect("cannot open file"),
        }
    }
}

impl Default for DiskWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl WalWriter for DiskWriter {
    fn append(
        &mut self,
        entry: &WalEntry,
        encoder: Box<dyn WalEncoder>,
    ) -> Result<(), WalError> {
        let bytes = encoder.encode(entry)?;
        self.file.write(&bytes)?;
        Ok(())
    }

    fn flush() -> Result<(), WalError> {
        todo!()
    }

    fn sync() -> Result<(), WalError> {
        todo!()
    }
}
