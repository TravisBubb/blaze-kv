# Blaze-KV: A High-Performance Key-Value Store

## Overview

Blaze-KV is an educational, high-performance key-value store written in Rust.
The initial focus is to build a robust single-node store, setting the foundation
for a distributed version in later phases.

This project aims to practice system design, Rust development, and low-level
engineering while producing a resume-worthy artifact.

## Goals

- Build a single-node key-value store with CRUD operations.
- Support durable persistence to disk (so data survives restarts).
- Provide a simple client API for interaction (e.g., get, set, delete).
- Prioritize correctness, simplicity, and extensibility for future distributed
  features.

## High-Level Architecture

- **Storage Engine**:
    - In-memory `HashMap` to hold keys and values
- **Persistence**:
    - Write-ahead logging: before applying changes to memory, they are appended
      to a log file.
    - Snapshotting: periodically serialize the in-memory database to a compact
      file to allow faster-recovery.
- **Client Interface**:
    - Expose a simple CLI to interact with the store.
    - Future-proofing for client-server model via TCP or gRPC.
    
## Data Model

- **Key**: `String`
- **Value**:`Vec<u8>` (binary data blob)
- **Operations**:
    - `set(key, value)`
    - `get(key) -> Option<value>`
    - `delete(key)`

## Storage Engine Design

The storage engine for Blaze-KV (phase 1) will consist of two major components:

1. **In-Memory Index**
    - A simple `HashMap<String, ValueEntry>`
        - key: `String`
        - value: `ValueEntry` which contains:
            - the actual `Vec<u8>` data
            - metadata (e.g., timestamp, size, optional expiration)
2. **Persistence Layer**
    - Append-only log (Write-Ahead Log or WAL) that persists every mutation
      (`set`, `delete`) before updating the in-memory store.
    - Periodic full-database snapshot to allow faster recovery.

This two-layer design separates storage durability from query performance,
enabling quick reads while maintaining fault-tolerance.

## Persistence Design

Blaze-KV will use a binary format for both the WAL and snapshot files to
maximize speed and minimize storage size.

### Write-Ahead Log (WAL) Entry Format

Each entry will be encoded as:
|Field|Size|Description|
|-----|----|-----------|
|Operation Type|1 byte|`0x01` for Set, `0x02` for Delete|
|Key Length|4 bytes (u32)|Length of the key in bytes|
|Value Length|4 bytes (u32)|Length of the value in bytes (0 if Delete)|
|Key Bytes|Variable|UTF-8 encoded string bytes|
|Value Bytes|Variable|Raw bytes (if Set)|

Set Example:
```
[0x01][0x00 0x00 0x00 0x03][0x00 0x00 0x00 0x05][b'f' b'o' b'o'][b'v' b'a' b'l' b'u' b'e']
```

Delete Example:
```
[0x02][0x00 0x00 0x00 0x03][0x00 0x00 0x00 0x00][b'f' b'o' b'o']
```

Notes:
- All multi-byte numbers are stored in Big Endian order
- Delete entries have a `Value Length` of `0` and no value bytes.

## Snapshot File Format

The snapshot file will store the entire database at a point in time.

Each record inside the snapshot will look similar to a Set entry:
|Field|Size|Description|
|-----|----|-----------|
|Key Length|4 bytes (u32)|Length of the key in bytes|
|Value Length|4 bytes (u32)|Length of the value in bytes (0 if Delete)|
|Key Bytes|Variable|UTF-8 encoded string bytes|
|Value Bytes|Variable|Raw bytes (if Set)|

**At Load Time**:
The snapshot file is read record-by-record to rebuild the in-memory `HashMap`.

**Notes**:
- Snapshots are append-only during generation but read-sequential at load.
- No operation type needed -- snapshots always represent a Set.

## Recovery Strategy

On Startup:
1. Check for a snapshot file. If it exists, then load it fully into memory.
2. Check for a WAL file.
    - Replay every entry (Set/Delete) on top of the loaded snapshot.
    - Apply mutations in order to rebuild the latest state.

After Recovery:
- Truncate or archive the WAL.
- Resume normal operation.

## Error Handling Strategy

- Graceful error propogation using Rust's `Result<T, E>`.
- Custom error types for storage errors, IO errors, and application logic
  errors.
- Crash-consistency focus: data corruption must be prevented even in case of
  panic or crash during writes.

## Future Extensions (After Phase 1)

- Add TCP server and client communication.
- Implement basic replication (leader-follower model).
- Build consensus for distributed mode (e.g., lightweight Raft).
- Add partitioning and sharding support.
