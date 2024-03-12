# Client Example

## Setup

(swap the `cargo run --manifest-path ../../Cargo.toml --` for `bicycle` command for typical usage)

```bash
## generate the initial __bicycle__ build
cargo run --manifest-path ../../Cargo.toml -- build schema.proto --engine sqlite

## build.rs will now handle any future schema.proto changes

## build the project with `__bicycle__/core` import now available
cargo build
```

## Run The Database Server

```bash
## use the CLI
cargo run --manifest-path ../../Cargo.toml -- start

## run the server from the binary
RUST_LOG=info ./__bicycle__/target/release/bicycle_server
```

## Test The Client

```bash
cargo run
```