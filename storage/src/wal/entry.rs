use bincode::{Decode, Encode};

/// Represents a single WAL entry
#[derive(Debug, Encode, Decode, PartialEq)]
pub enum WalEntry {
    Set { key: String, data: Vec<u8> },
    Delete { key: String },
}
