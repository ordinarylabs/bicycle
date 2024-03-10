# Bicycle 🚲

[![ci](https://github.com//ordinarylabs/bicycle/actions/workflows/ci.yml/badge.svg)](https://github.com//ordinarylabs/bicycle/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/bicycle.svg)](https://crates.io/crates/bicycle)
[![docs.rs](https://docs.rs/bicycle/badge.svg)](https://docs.rs/bicycle/)
[![license](https://img.shields.io/github/license/ordinarylabs/bicycle.svg)](https://github.com/ordinarylabs/bicycle/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/ordinarylabs/bicycle/status.svg)](https://deps.rs/repo/github/ordinarylabs/bicycle)

A protobuf defined database framework.

## Install

Before installing `bicycle` you'll need to have [Rust](https://www.rust-lang.org/tools/install) and [protoc](https://grpc.io/docs/protoc-installation/) installed.

```bash
cargo install bicycle
```

## Usage

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
### Create

Run the `create` command to generate your Bicycle server binary and protobuf definition.

```bash
bicycle create schema.proto
```

## Server

You can now run the server binary with the following command.

```bash
./__bicycle__/target/release/bicycle_server
```

To test some basic CRUD you can use [gRPCurl](https://github.com/fullstorydev/grpcurl)

```bash
## PutDog
grpcurl -plaintext -d '{
  "pk": "1",
  "name": "Rover",
  "age": 3,
  "breed": "Golden Retriever"
}' 0.0.0.0:50051 bicycle.Bicycle.PutDog

## BatchPutDogs
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

## GetDogs
grpcurl -plaintext -d '{"begins_with": ""}' 0.0.0.0:50051 bicycle.Bicycle.GetDogsByPk

## DeleteDogs
grpcurl -plaintext -d '{"eq": "3"}' 0.0.0.0:50051 bicycle.Bicycle.DeleteDogsByPk
```

## Client

You can also use the ` ./__bicycle__/proto/bicycle.proto` (see example output below) to codegen your own database clients. Because the Bicycle server is just a gRPC server, any language with gRPC support also has Bicycle client support. 

Additionally, Bicycle servers implement [server reflection](https://github.com/grpc/grpc/blob/master/doc/server-reflection.md), so you can also roll over to your preferred gRPC desktop client (i.e Postman), type in `0.0.0.0::50051`, and it will automatically load up all your available RPCs.

```protobuf
//  ./__bicycle__/proto/bicycle.proto
syntax = "proto3";
package bicycle;

message Dogs { 
  repeated Dog dogs = 1; 
}
message Dog {
  string pk = 1;
  string name = 2;
  uint32 age = 3;
  string breed = 4;
}

message IndexQuery {
  oneof expression {
    string eq = 1;
    string gte = 2;
    string lte = 3;
    string begins_with = 4;
  }
}

message Empty {}

service Bicycle {
  rpc GetDogsByPk(IndexQuery) returns (Dogs) {}
  rpc DeleteDogsByPk(IndexQuery) returns (Empty) {}
  rpc PutDog(Dog) returns (Empty) {}
  rpc BatchPutDogs(Dogs) returns (Empty) {}
}
```

## Embedding

In addition to the gRPC server based implementation, you can also use the generated `core` functions without using gRPC at all. The storage and request format remain protobuf, but without the remote server interaction.

You can import the core functionality into your project by adding the generated `bicycle_core` as a dependency in your `Cargo.toml`

```toml
# embedded-dogs/Cargo.toml
bicycle = { package = "bicycle_core", path = "__bicycle__/core" }
```

```rust
// embedded-dogs/src/main.rs
use bicycle::models::dog;
use bicycle::proto::{index_query::Expression, Example, IndexQuery};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dog::put(Example {
        pk: "DOG#1".to_string(),
        name: "Leia".to_string(),
        age: 3,
        breed: "Pitty".to_string(),
    })?;

    let dog = dog::get_by_pk(IndexQuery {
        expression: Some(Expression::Eq("DOG#1".to_string())),
    })?;

    println!("{:#?}", dog);

    Ok(())
}
```

## SPROCS

Stored procedures are supported and can be written in Rust built for the `wasm32-wasi` target. Currently only `stdio` is inherited from the host context and the additional WASI APIs are not supported (this means your `println!()`s will show up on the host but you don't have access to things like the file system).

`bicycle sproc ...` commands depend on `cargo-wasi` which can be installed using `cargo install cargo-wasi` (details [here](https://bytecodealliance.github.io/cargo-wasi/install.html)).

For this example we want to create a stored procedure that will return us only the `Dog`'s names. To create a new SPROC we run the following

```bash
cargo new dog-names-proc
```

Some additional items need to be added to the `Cargo.toml`. The host shims are provided by the build output in `__bicycle__` and will need to be added as a dependency. You will also need to set your build target's name to `"proc"` and optionally adjust the release profile to produce smaller WASM binaries.

```toml
# dog-names-proc/Cargo.toml

## shims to interact more ergonomically with the host functions
host = { package = "bicycle_shims", path = "__bicycle__/shims" }

## set the binary name to "proc" so the CLI can deploy it
[[bin]]
path = "src/main.rs"
name = "proc"

## recommended for smaller binaries
[profile.release]
lto = true
opt-level = 'z'
```

For a basic SPROC example we have the following which just uses the `get_input` to get the pull in the dynamic input passed in at call time, and pass back that same value as output. All SPROC I/O uses the [`Value`](https://protobuf.dev/reference/protobuf/google.protobuf/#value) protobuf message (`bicycle_shims` re-exports `prost-types` which provides the Rust implementation of the `Value` type).

```rust
// dog-names-proc/src/main.rs
use host::prost_types::{value::Kind, ListValue, Value};
use host::{get_input, set_output};

use host::models::dog;
use host::proto::{index_query::Expression, Dogs, IndexQuery};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let val: Option<Value> = get_input()?;

    let val = match val {
        Some(val) => match val.kind {
            Some(Kind::StringValue(val)) => val,
            _ => "".to_string(),
        },
        None => "".to_string(),
    };

    let Dogs { dogs } = dog::get_by_pk(IndexQuery {
        expression: Some(Expression::BeginsWith(val)),
    })?;

    let names = dogs
        .into_iter()
        .map(|dog| Value {
            kind: Some(Kind::StringValue(dog.name)),
        })
        .collect::<Vec<Value>>();

    set_output(Some(Value {
        kind: Some(Kind::ListValue(ListValue { values: names })),
    }))?;

    Ok(())
}
```

To test the procedure as a one-off against your Bicycle server

```bash
bicycle sproc oneoff ./dog-names-proc --addr http://0.0.0.0:50051 --lang rust --args ""
```

To store the procedure on your Bicycle server for future execution

```bash
bicycle sproc deploy ./dog-names-proc --addr http://0.0.0.0:50051 --name dog-names-proc --lang rust
```

To execute a previously stored procedure on your Bicycle server

```bash
bicycle sproc exec --addr http://0.0.0.0:50051 --name dog-names-proc --args ""
```

## License

[AGPL-v3](https://opensource.org/license/AGPL-v3)
