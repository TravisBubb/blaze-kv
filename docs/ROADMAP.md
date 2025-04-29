# Project Roadmap

This roadmap outlines the full development lifecycle for a distributed key-value
store written in Rust. The goal is to create a production-ready, horizontally
scalable, self-hosted system that supports data replication, consensus,
persistence, and observability. It will be Docker-deployable and suitable for
real-world deployment in small to medium-scale environments.

---

## 🧱 Phase 1: Single-Node Foundation

### 🎯 Objectives

- Implement core storage engine (in-memory + disk-based)
- Support persistence through a Write-Ahead Log (WAL)
- Provide a simple API to interact with the node

### 📋 Features

- `SET`, `GET`, `DELETE` operations
- In-memory store backed by `HashMap`
- Append-only Write-Ahead Log (WAL)
- WAL replay on startup
- Basic HTTP interface
- Simple configuration file
- Graceful shutdown and error handling
- Pluggable logging system

### 🛠 Tasks
- Define `StorageEngine` trait and default `MemoryEngine`
- Implement WAL format (e.g., bincode or protobuf)
- Create replay mechanism for recovery
- Build HTTP API
- Integrate logging
- Unit tests and integration tests
- Benchmarks

---

## 🗃️ Phase 2: Persistent Storage Abstraction

### 🎯 Objectives

- Modularize and enhance persistence
- Allow flexible storage backends (e.g., RocksDB, SSTables)

### 📋 Features

- Pluggable persistence engine
- WAL compaction
- Snapshotting mechanism

### 🛠 Tasks

- Abstract WAL and flushable storage backend
- Add log compaction and snapshot support
- Implement `RocksDbEngine` as optional backend
- Add benchmark suite to compare engines

## 🌐 Phase 3: Networking & Cluster Awareness

### 🎯 Objectives

- Enable communication between nodes
- Build cluster membership and topology awareness

### 📋 Features

- Unique node identity
- TCP/gRPC server on each node
- Cluster join/leave protocol
- Discovery via gossip, DNS, or static config
    - Probably starting with static config in this phase...
- Cluster metadata service

### 🛠 Tasks

- Define message format (protobuf)
- Implement gRPC or raw TCP messaging
- Build membership service with periodic heartbeats
- Create gossip-based cluster state propogator
- Add node health monitoring and state transitions

## ⚖️ Phase 4: Sharding & Key Distribution

### 🎯 Objectives

- Distribute keys across multiple nodes
- Support cluster scaling with data rebalancing

### 📋 Features

- Consistent hash ring
- Shard allocation
- Dynamic node join/leave with rebalancing
- Node-specific shard maps

### 🛠 Tasks

- Implement consistent hashing ring
- Map key ranges to shard responsibilities
- Create shard assignment module
- Implement data migration and balancing logic

## 🔁 Phase 5: Replication & Consensus

### 🎯 Objectives

- Ensure fault tolerance via replication
- Maintain consistency using Raft protocol

### 📋 Features

- Configurable replication factor (e.g., RF=3)
- Raft-based leader election and log replication
- Follower recovery and state sync
- Read and write quorum enforcement

### 🛠 Tasks

- Integrate Raft (try to implement own version of algorithm)
- Implement Raft state machine for log commits
- Design data replication flow (WAL replication, snapshots)
- Implement catch-up for out-of-sync followers
- Add failure detection and retry mechanisms

## 🛡️ Phase 6: Security & Access Control

### 🎯 Objectives

- Protect system access and node communication

### 📋 Features

- TLS encryption for all gRPC/TCP communication
- Basic Auth or JWT for API access
- Role-based Access Control (RBAC)

### 🛠 Tasks

- Enable mutual TLS between nodes
- Secure admin APIs with JWT tokens
- Design and enforce access control rules
- Integrate optional authentication layer

## 🧭 Phase 7: Routing Layer & Load Balancer

### 🎯 Objectives

- Build a gateway or router node for external access
- Support autoscaling and routing to correct nodes

### 📋 Features

- Client-facing router service
- Request routing based on consistent hash
- Health checks and circuit breakers
- Retry logic and failover routing

### 🛠 Tasks

- Build stateless router binary
- Integrate cluster membership awareness
- Implement key-based routing logic
- Handle request deduplication and timeouts

## 📦 Phase 8: Deployment & Dockerization

### 🎯 Objectives

- Provide an easy-to-deploy solution using Docker

### 📋 Features

- Dockerfiles for all components
- Docker Compose for development
- Helm chart or K8s manifests

### 🛠 Tasks

- Write Dockerfiles for core storage server and router applications
- Add volume mounting for persistent WAL data
- Compose file with 3-node cluster + router
- Write deployment guide and config examples

## 📊 Phase 9: Observability & Monitoring

### 🎯 Objectives

- Make system observable and debuggable

### 📋 Features

- Structured logs
- Prometheus metrics endpoint
- Distributed tracing
- Metrics for WAL, storage, Raft state, RPC latency

### 🛠 Tasks

- Add `metrics` or `prometheus` crate
- Instrument all subsystems (WAL, Raft, Router)
- Expose `/metrics` endpoint on all components
- Export logs in structured JSON format

## 🚀 Future

- Distributed transactions
- TTL for keys and automatic eviction
- Web dashboard for observability and management
- Backup and restore support
- Pub/sub and streaming capabilities
- API client SDKs
- Multi-datacenter support

