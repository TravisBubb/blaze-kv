use super::{
    config::WalConfig, handle::WalHandle, message::WalMessage,
    writer::WalWriter,
};

/// Spawn the WAL consumer thread that handles writing WAL entries
pub fn spawn_wal_thread<W: WalWriter + Send + 'static>(
    mut writer: W,
    config: &WalConfig,
) -> WalHandle {
    let (tx, mut rx) = tokio::sync::mpsc::channel(config.buffer_size);

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            match msg {
                WalMessage::Write(entry, ack) => {
                    let result = writer.append(&entry).await;
                    if let Some(ack_tx) = ack {
                        let _ = ack_tx.send(result);
                    }
                }
                WalMessage::Flush(_ack) => todo!(),
                WalMessage::Sync(_ack) => todo!(),
                WalMessage::Shutdown(ack) => {
                    // TODO: Flush and Sync before shutting down
                    if let Some(ack_tx) = ack {
                        let _ = ack_tx.send(Ok(()));
                    }
                    return;
                }
            }
        }
    });

    WalHandle { tx }
}
