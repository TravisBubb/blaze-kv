use storage::wal::{
    BincodeEncoder, DiskWriter, WalEntry, config::WalConfig, spawn_wal_thread,
};

#[tokio::main]
async fn main() {
    let encoder = BincodeEncoder::new();
    let writer = DiskWriter::new(encoder, "blazekv.wal")
        .await
        .expect("failed to create writer");
    let config = WalConfig { buffer_size: 16 };

    let handle = spawn_wal_thread(writer, &config);

    let entry =
        WalEntry::Set { key: "K1".to_string(), data: "V1".as_bytes().to_vec() };

    let _ = handle.write(entry).await;

    let _ = handle.shutdown().await;
}
