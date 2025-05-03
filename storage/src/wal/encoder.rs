use super::{entry::WalEntry, error::WalError};

/// Represents an item that can encode and decode WAL entries
pub trait WalEncoder {
    /// Encodes the provided WAL entry into a binary format
    fn encode(&self, entry: &WalEntry) -> Result<Vec<u8>, WalError>;

    /// Decodes the provided binary data into a WAL entry
    fn decode(&self, data: &Vec<u8>) -> Result<WalEntry, WalError>;
}
