use serde::{Deserialize, Serialize};

/// Represents a single WAL entry
#[derive(Debug, Serialize, Deserialize)]
pub enum WalEntry {
    Set { key: String, data: Vec<u8> },
    Delete { key: String },
}
