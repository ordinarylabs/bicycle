# BicycleDB Manager

[![ci](https://github.com//ordinarylabs/bicycle/actions/workflows/ci.yml/badge.svg)](https://github.com//ordinarylabs/bicycle/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/bicycle.svg)](https://crates.io/crates/bicycle)
[![docs.rs](https://docs.rs/bicycle/badge.svg)](https://docs.rs/bicycle/)
[![license](https://img.shields.io/github/license/ordinarylabs/bicycle.svg)](https://github.com/ordinarylabs/bicycle/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/ordinarylabs/bicycle/status.svg)](https://deps.rs/repo/github/ordinarylabs/bicycle)

The CLI and build functions for BicycleDB.

## CLI

Before installing `bicycle` you'll need to have [Rust](https://www.rust-lang.org/tools/install) and [protoc](https://grpc.io/docs/protoc-installation/) installed.

### Install

```bash
cargo install bicycle
```

### Building

With your schema, you can use the `build` command to generate your Bicycle components.

```bash
bicycle build schema.proto
```

### Engines

Bicycle's default storage engine is RocksDB but `librocksdb-sys` takes quite awhile for the initial build (subsequent builds should be quicker as you iterate on your schema). If you'd like a faster initial build or would prefer SQLite for other reasons you can also use the SQLite engine by supplying the `--engine` flag.

```bash
bicycle build schema.proto --engine sqlite
```

### Running the server

You can now start the server with the following command.

```bash
bicycle start
```

### Invoke and Deploy Biplane Functions (a.k.a SPROCs)

`bicycle fn` commands depend on `cargo-wasi` when compiling for `--lang rust`; the binary can be installed using `cargo install cargo-wasi` (details [here](https://bytecodealliance.github.io/cargo-wasi/install.html)).

#### Deploy

```bash
bicycle fn deploy \
  --addr http://0.0.0.0:50051 \
  --lang rust \
  --path ./path/to/fn \
  --name some-fn-name
```

#### Invoke Deployed

```bash
bicycle fn invoke \
  --addr http://0.0.0.0:50051 \
  --name some-fn-name \
  --args '{"some_key": "some_value"}'
```

#### One-off

```bash
bicycle fn invoke \
  --addr http://0.0.0.0:50051 \
  --lang rust \
  --path ./path/to/fn \
  --args '{"some_key": "some_value"}'
```

## Automated builds

The components used in the CLI executable are also exposed for usage in `build.rs` files.

```toml
# Cargo.toml
[build-dependencies]
bicycle = "x.x.x"
```

**NOTE:** if using path imports for `bicycle_shims` or `bicycle_core` will need to run `bicycle build schema.proto` prior to the initial build so that `cargo` has a `__bicycle__/core|shims/Cargo.toml` to reference. Subsequent changes to `schema.proto` should not require a re-run of the `bicycle build` command with the CLI.

```rust
// build.rs
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let schema_path = concat!(env!("CARGO_MANIFEST_DIR"), "/schema.proto");
    bicycle::build(schema_path, "rocksdb")
}
```

See [examples](https://github.com/ordinarylabs/bicycle/tree/main/examples) for more detailed usage.

## License

[AGPL-v3](https://opensource.org/license/AGPL-v3)
