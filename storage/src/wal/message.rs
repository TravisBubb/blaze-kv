use tokio::sync::oneshot;

use super::{WalEntry, error::WalError};

/// Represents a message passed through the WAL channels
pub enum WalMessage {
    Write(WalEntry, Option<oneshot::Sender<Result<(), WalError>>>),
    Flush(Option<oneshot::Sender<Result<(), WalError>>>),
    Sync(Option<oneshot::Sender<Result<(), WalError>>>),
    Shutdown(Option<oneshot::Sender<Result<(), WalError>>>),
}
