# BicycleDB 🚲

[![ci](https://github.com//ordinarylabs/bicycle/actions/workflows/ci.yml/badge.svg)](https://github.com//ordinarylabs/bicycle/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/bicycle.svg)](https://crates.io/crates/bicycle)
[![docs.rs](https://docs.rs/bicycle/badge.svg)](https://docs.rs/bicycle/)
[![license](https://img.shields.io/github/license/ordinarylabs/bicycle.svg)](https://github.com/ordinarylabs/bicycle/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/ordinarylabs/bicycle/status.svg)](https://deps.rs/repo/github/ordinarylabs/bicycle)

Accelerating development and improving DevEx without sacrificing performance.

BicycleDB is a framework for compiling database servers whose application models and access patterns are built in at compile time; with protobuf as the transport/storage format, each database instance is a gRPC server compiled from Rust, backed by a RocksDB or SQLite storage engine.

## CLI

See the BicycleDB Manager CLI documentation [here](https://crates.io/crates/bicycle). For querying, any of [these](https://github.com/grpc-ecosystem/awesome-grpc?tab=readme-ov-file#cli) should work.

## GUI

BicycleDB servers support reflection, so any of [these](https://github.com/grpc-ecosystem/awesome-grpc?tab=readme-ov-file#gui) should work.

## Supported Features

- Key/Value storage and retrieval
- Range queries via `gte`, `lte` and `begins_with`
- Stored Procedures via WebAssembly (non-transactional)
- Embedding for offline or local storage
- Protobuf message nesting for document-like records

## Planned Features

- Built-in Relationships
- Events/Streaming
- Transactional SPROCs

## Examples

Check out stored procedure, offline and client usage examples [here](https://github.com/ordinarylabs/bicycle/tree/main/examples).

## License

[AGPL-v3](https://opensource.org/license/AGPL-v3)
