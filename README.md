# Bicycle 🚲

[![ci](https://github.com//ordinarylabs/bicycle/actions/workflows/ci.yml/badge.svg)](https://github.com//ordinarylabs/bicycle/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/bicycle.svg)](https://crates.io/crates/bicycle)
[![docs.rs](https://docs.rs/bicycle/badge.svg)](https://docs.rs/bicycle/)
[![license](https://img.shields.io/github/license/ordinarylabs/bicycle.svg)](https://github.com/ordinarylabs/bicycle/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/ordinarylabs/bicycle/status.svg)](https://deps.rs/repo/github/ordinarylabs/bicycle)

Protobuf defined database framework.

## Install

Before installing `bicycle` you'll need to have [Rust](https://www.rust-lang.org/tools/install) and [protoc](https://grpc.io/docs/protoc-installation/) installed.

```bash
cargo install bicycle
```

## Usage

### Schema

A Bicycle schema is defined in a simple `.proto` file.

```protobuf
// schema.proto
syntax = "proto3";
package bicycle;

message Dog {
  string pk = 1;

  string name = 2;
  uint32 age = 3;
  string breed = 4;
}
```

### CLI

Now that you have your schema, you can run the `build` command to generate your Bicycle components.

```bash
bicycle build schema.proto
```

### Engines

The default engine is RocksDB but `rocksdblib-sys` takes quite awhile for the initial build (subsequent builds should be quicker). If you'd like a faster initial build or would prefer SQLite for other reasons you can also use the SQLite engine by supplying the `--engine` flag.

```bash
bicycle build schema.proto --engine sqlite
```

## Server

### Start

You can now start the server with the following command.

```bash
bicycle start
```

### Testing RPCs

To test some basic CRUD you can use [gRPCurl](https://github.com/fullstorydev/grpcurl)

```bash
## Put
grpcurl -plaintext -d '{
  "pk": "1",
  "name": "Rover",
  "age": 3,
  "breed": "Golden Retriever"
}' 0.0.0.0:50051 bicycle.Bicycle.PutDog

## BatchPut
grpcurl -plaintext -d '{
  "dogs": [
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

## GetByPk
grpcurl -plaintext -d '{"begins_with": ""}' 0.0.0.0:50051 bicycle.Bicycle.GetDogsByPk

## DeleteByPk
grpcurl -plaintext -d '{"eq": "3"}' 0.0.0.0:50051 bicycle.Bicycle.DeleteDogsByPk
```

## Client

### Rust

The Rust client code is generated by default and can be added to any Rust project as a dependency

```toml
# dogs-app/Cargo.toml
[dependencies]
bicycle = { package = "bicycle_core", path = "__bicycle__/core" }
tokio = { version = "1.36.0", features = ["rt", "macros", "rt-multi-thread"] }
```

Call the Bicycle server from your Rust app

```rust
// dogs-app/src/main.rs
use bicycle;
use bicycle::proto::{bicycle_client::BicycleClient, index_query::Expression, Dog, IndexQuery};
use bicycle::tonic::Request;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // establish connection to remote Bicycle server
    let mut client = BicycleClient::connect("http://0.0.0.0:50051").await?;

    // write a dog to remote Bicycle server
    client
        .put_dog(Request::new(Dog {
            pk: "4".to_string(),
            name: "Sam".to_string(),
            age: 6,
            breed: "Labrador".to_string(),
        }))
        .await?;

    // get dog back from remote Bicycle server
    let dogs = client
        .get_dogs_by_pk(Request::new(IndexQuery {
            expression: Some(Expression::Eq("4".to_string())),
        }))
        .await?;

    Ok(())
}
```

### Other Languages

You can also use the ` ./__bicycle__/proto/bicycle.proto` to codegen your own database clients for any other language. Because the Bicycle server is just a gRPC server, any language with gRPC support also has Bicycle client support.

### Desktop GUIs

Bicycle servers also implement [server reflection](https://github.com/grpc/grpc/blob/master/doc/server-reflection.md), so you can roll over to your preferred gRPC desktop client (i.e Postman, BloomRPC), type in `0.0.0.0::50051`, and they should be able to automatically load up all your RPCs.

## Embedding

### Rust

In addition to the gRPC server based implementation, you can also use the generated Rust `core` functions without using gRPC at all. The query/storage formats remain protobuf, but without the remote server interaction.

You can import the core functionality into your project by adding the generated `bicycle_core` as a dependency in your `Cargo.toml`

```toml
# embedded-dogs/Cargo.toml
[dependencies]
bicycle = { package = "bicycle_core", path = "__bicycle__/core" }
```

```rust
// embedded-dogs/src/main.rs
use bicycle;
use bicycle::proto::{index_query::Expression, Dog, IndexQuery};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // write a dog to local Bicycle
    bicycle::put_dog(Dog {
        pk: "0".to_string(),
        name: "Ollie".to_string(),
        age: 7,
        breed: "Pitty".to_string(),
    })?;

    // get that dog back from local Bicycle
    let dogs = bicycle::get_dogs_by_pk(IndexQuery {
        expression: Some(Expression::Eq("0".to_string())),
    })?;

    Ok(())
}
```

### Other Languages

Currently no other languages are officially supported for embedding, but it is the intention to add support via FFIs at some point in the future. Because it will mostly be passing encoded protobuf messages through as bytes it should be fairly straightforward to implement bindings for other languages with protobuf support.

## SPROCS

Stored procedures are supported in the form of *Biplane Functions* which can be written in Rust built for the `wasm32-wasi` target.

`bicycle fn` commands depend on `cargo-wasi` when compiling for `--lang rust`; it can be installed using `cargo install cargo-wasi` (details [here](https://bytecodealliance.github.io/cargo-wasi/install.html)).

### Definition

For this example we want to create a stored procedure that will return only the `Dog`'s names. To create a new SPROC we run the following

```bash
cargo new dog-names-fn
```

Some additional items need to be added to the `Cargo.toml`. The host shims are provided by the build output in `__bicycle__` and will need to be added as a dependency. You will also need to set your build target's name to `"proc"` and optionally adjust the release profile to produce smaller WASM binaries.

```toml
# dog-names-fn/Cargo.toml

## shims to interact more cleanly with the host functions
[dependencies]
bicycle = { package = "bicycle_shims", path = "__bicycle__/shims" }

## set the binary name to "proc" so the CLI can deploy it
[[bin]]
path = "src/main.rs"
name = "biplane_function"

## recommended for smaller binaries
## also see: https://github.com/johnthagen/min-sized-rust
[profile.release]
strip = true
lto = true
opt-level = 'z'
codegen-units = 1
```

For a basic SPROC example we have the following which uses the `recv_in` to get the dynamic input passed to the SPROC by the caller.

Once we have the "begins_with" argument from `recv_in` we use the `get_dogs_by_pk` shim to grab the requested `Dog`s from the host, map over just their names, and then send the output back to the host via `send_out`. Once the host captures the result of `send_out` they will forward it onto the caller.

**NOTE**: all SPROC I/O utilizes the [`Value`](https://protobuf.dev/reference/protobuf/google.protobuf/#value) protobuf type and `bicycle_shims` re-exports `prost-types` crate which provides the Rust implementation of the `Value` type for you.

```rust
// dog-names-fn/src/main.rs
use bicycle;
use bicycle::prost_types::{value::Kind, ListValue, Value};
use bicycle::proto::{index_query::Expression, Dogs, IndexQuery};
use bicycle::{recv_in, send_out};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // get input from the host Bicycle server context, sent by caller
    let val: Option<Value> = recv_in()?;

    let mut begins_with = "".to_string();

    // extract "begins_with" from `Value`
    if let Some(Value {
        kind: Some(Kind::StructValue(struct_val)),
    }) = val
    {
        if let Some(Kind::StringValue(val)) = struct_val
            .fields
            .get("begins_with")
            .map(|v| v.kind.as_ref())
            .flatten()
        {
            begins_with = val.clone()
        }
    }

    // get dogs from the host Bicycle server
    let Dogs { dogs } = bicycle::get_dogs_by_pk(IndexQuery {
        expression: Some(Expression::BeginsWith(begins_with)),
    })?;

    // build a list of dog names as `StringValue`s
    let names = dogs
        .into_iter()
        .map(|dog| Value {
            kind: Some(Kind::StringValue(dog.name)),
        })
        .collect::<Vec<Value>>();

    // set output for host Bicycle server to read in and send back to caller
    send_out(Some(Value {
        kind: Some(Kind::ListValue(ListValue { values: names })),
    }))
}
```

### Invoking

To test the procedure as a one-off against your Bicycle server

```bash
bicycle fn invoke \
  --addr http://0.0.0.0:50051 \
  --lang rust \
  --path ./dog-names-fn \
  --args '{"begins_with": ""}'
```

To store the procedure on your Bicycle server for future execution

```bash
bicycle fn deploy \
  --addr http://0.0.0.0:50051 \
  --lang rust \
  --path ./dog-names-fn \
  --name dog-names-fn
```

To execute a previously stored procedure on your Bicycle server

```bash
bicycle fn invoke \
  --addr http://0.0.0.0:50051 \
  --name dog-names-fn \
  --args '{"begins_with": ""}'
```

### Caveats

SPROCS are not yet transactional, so any error that causes the procedure to terminate prematurely can result in partial writes. It is the intention to make SPROCS transactional at some later date.

Only `stdio` is inherited from the host context and the additional WASI APIs are not supported (this means your `println!()`s will show up on the host but you don't have access to things like the file system).

Currently all SPROC invocations freshly compile the WebAssembly binary using the [wasmtime](https://docs.rs/wasmtime/latest/wasmtime/) `Module::new()` function; this operation makes up the majority of the overhead used by the SPROC invocation and can add 10s of milliseconds depending on the environment. This will be optimized away in the future by caching the modules.

## License

[AGPL-v3](https://opensource.org/license/AGPL-v3)
