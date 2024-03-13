# Embedded Example

## Setup

(swap the `cargo run --manifest-path ../../Cargo.toml --` for `bicycle` command for typical usage)

```bash
## generate the initial __bicycle__ build.
cargo run --manifest-path ../../Cargo.toml -- build schema.proto --engine sqlite

## build.rs will now handle any future schema.proto changes
```

## Test The Database Interaction

```bash
## run the project with `__bicycle__/core` import now available.
cargo run
```