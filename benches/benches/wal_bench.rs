use criterion::{Criterion, criterion_group, criterion_main};
use storage::wal::{
    BincodeEncoder, DiskWriter, WalEntry, config::WalConfig, handle::WalHandle,
    spawn_wal_thread,
};

async fn write_to_wal(handle: WalHandle, num_tasks: usize) {
    let mut tasks = vec![];

    for i in 0..num_tasks {
        let handle = handle.clone();
        let task = tokio::spawn(async move {
            let entry = WalEntry::Set {
                key: format!("K{}", i),
                data: format!("V{}", i).as_bytes().to_vec(),
            };

            handle.write(entry).await.expect("Failed to write to file");
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await.expect("Task failed");
    }
}

fn benchmark_concurrent_writes(c: &mut Criterion) {
    c.bench_function("concurrent_writes", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        b.to_async(&rt).iter(|| async {
            let writer = DiskWriter::new(BincodeEncoder::new(), "bench.wal")
                .await
                .expect("failed to initialize writer");
            let config = WalConfig { buffer_size: 16 };
            let handle = spawn_wal_thread(writer, &config);
            write_to_wal(handle.clone(), 25).await;
        });
    });
}

criterion_group!(benches, benchmark_concurrent_writes);
criterion_main!(benches);
