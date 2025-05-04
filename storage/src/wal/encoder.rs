use bincode::config::standard;
use once_cell::sync::Lazy;

use super::{entry::WalEntry, error::WalError};

/// Represents an item that can encode and decode WAL entries
pub trait WalEncoder {
    /// Encodes the provided WAL entry into a binary format
    fn encode(&self, entry: &WalEntry) -> Result<Vec<u8>, WalError>;

    /// Decodes the provided binary data into a WAL entry
    fn decode(&self, data: &[u8]) -> Result<WalEntry, WalError>;
}

/// Represents a WalEncoder that uses bincode
pub struct BincodeEncoder {}

impl BincodeEncoder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for BincodeEncoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Default configuration for bincode
static BINCODE_CONFIG: Lazy<bincode::config::Configuration> =
    Lazy::new(standard);

impl WalEncoder for BincodeEncoder {
    fn encode(&self, entry: &WalEntry) -> Result<Vec<u8>, WalError> {
        let bytes = bincode::encode_to_vec(entry, *BINCODE_CONFIG)?;
        Ok(bytes)
    }

    fn decode(&self, data: &[u8]) -> Result<WalEntry, WalError> {
        let (entry, _): (WalEntry, usize) =
            bincode::decode_from_slice(data, *BINCODE_CONFIG)?;
        Ok(entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_wal_entry() {
        let entry = WalEntry::Set { key: "key1".into(), data: vec![1, 2, 3] };

        let bytes =
            BincodeEncoder::new().encode(&entry).expect("encoding failed");

        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_decode_wal_entry() {
        let entry = WalEntry::Set { key: "key1".into(), data: vec![1, 2, 3] };

        let bytes =
            BincodeEncoder::new().encode(&entry).expect("encoding failed");

        let decoded_entry: WalEntry =
            BincodeEncoder::new().decode(&bytes).expect("decoding failed");

        assert_eq!(entry, decoded_entry);
    }

    #[test]
    fn test_decode_error_handling() {
        let invalid_bytes: Vec<u8> = vec![0, 1, 2, 3];

        let result = BincodeEncoder::new().decode(&invalid_bytes);

        assert!(matches!(result, Err(WalError::Decode(_))));
    }
}
