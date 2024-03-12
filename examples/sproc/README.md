# Biplane Function (SPROC) Example

## Setup

(swap the `cargo run --manifest-path ../../Cargo.toml --` for `bicycle` command for typical usage)

```bash
## generate the initial __bicycle__ build.
cargo run --manifest-path ../../Cargo.toml -- build schema.proto --engine sqlite

## build.rs will now handle any future schema.proto changes
```

## Run The Database Server

```bash
## use the CLI
cargo run --manifest-path ../../Cargo.toml -- start

## run the server from the binary
RUST_LOG=info ./__bicycle__/target/release/bicycle_server
```

## Seed

```bash
grpcurl -plaintext -d '{
  "dogs": [
    {
      "pk": "1",
      "name": "Rover",
      "age": 3,
      "breed": "Golden Retriever"
    },
    {
      "pk": "2",
      "name": "Buddy",
      "age": 2,
      "breed": "Labrador"
    },
    {
      "pk": "3",
      "name": "Max",
      "age": 4,
      "breed": "Poodle"
    }
  ]
}' 0.0.0.0:50051 bicycle.Bicycle.BatchPutDogs
```

## Test Biplane Function

### Deploy

```bash
cargo run --manifest-path ../../Cargo.toml -- fn deploy \
    --addr http://0.0.0.0:50051 \
    --lang rust \
    --path . \
    --name dog-names  
```

### Invoke Deployed

```bash
cargo run --manifest-path ../../Cargo.toml -- fn invoke \
    --addr http://0.0.0.0:50051 \
    --name dog-names \
    --args '{"begins_with": ""}'  
```

### Invoke One-off

```bash
cargo run --manifest-path ../../Cargo.toml -- fn invoke \
    --addr http://0.0.0.0:50051 \
    --lang rust \
    --path . \
    --args '{"begins_with": ""}'
```
