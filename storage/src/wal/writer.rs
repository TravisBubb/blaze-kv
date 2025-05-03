use super::{encoder::WalEncoder, entry::WalEntry, error::WalError};

/// Represents an item that can write WAL entries
pub trait WalWriter {
    /// Appends an entry to the WAL
    fn append(
        &mut self,
        entry: WalEntry,
        encoder: Box<dyn WalEncoder>,
    ) -> Result<(), WalError>;
}
