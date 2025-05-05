use tokio::sync::{mpsc::Sender, oneshot};

use super::{WalEntry, error::WalError, message::WalMessage};

#[derive(Clone)]
pub struct WalHandle {
    pub tx: Sender<WalMessage>,
}

impl WalHandle {
    pub async fn write(&self, entry: WalEntry) -> Result<(), WalError> {
        let (ack_tx, ack_rx) = tokio::sync::oneshot::channel();
        self.tx.send(WalMessage::Write(entry, Some(ack_tx))).await?;
        ack_rx.await?
    }

    pub async fn shutdown(&self) -> Result<(), WalError> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(WalMessage::Shutdown(Some(tx))).await?;
        rx.await?
    }

    // TODO: add flush and sync functions
}
