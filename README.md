# Bicycle 🚲

[![ci](https://github.com//ordinarylabs/bicycle/actions/workflows/ci.yml/badge.svg)](https://github.com//ordinarylabs/bicycle/actions/workflows/ci.yml)
[![license](https://img.shields.io/github/license/ordinarylabs/bicycle.svg)](https://github.com/ordinarylabs/bicycle/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/ordinarylabs/bicycle/status.svg)](https://deps.rs/repo/github/ordinarylabs/bicycle)

Bicycle is a framework for defining database schemas with protobuf such that access patterns are generated as code and compiled into the database server itself. The goal is to reduce the overhead of query/response parsing at run time by using a binary serialization format and empowering the compiler do query planning ahead of time.

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

## Running

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

## Clients

You can also use the ` ./__bicycle__/proto/bicycle.proto` (see example output below) to build your database clients.

Because the Bicycle server is just a gRPC server, you can use the gRPC libraries for any language you like. Additionally, Bicycle servers implement [server reflection] you can also roll over to your preferred gRPC GUI client (i.e Postman), type in `0.0.0.0::50051`, and it will automatically load up all your available RPCs.

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

## SPROCS

Stored procedures are supported and can be written in Rust with built with the `wasm32-wasi` target. Currently only `stdio` is inherited from the host context and the additional WASI APIs are not yet supported.

`bicycle sproc ...` commands depend on `cargo-wasi` which can be installed using `cargo install cargo-wasi`.

To create a new SPROC run the following

```bash
cargo new my-proc
```

Some additional items need to be added to the `Cargo.toml`. The host shims are provided in the build output and will need to be added as a dependency, as well as setting your build target's name to a fixed `"proc"` and adjusting the release profile to produce smaller build sizes.

```toml
# my-proc/Cargo.toml

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

For a basic SPROC example we have the following which just uses the `get_input` to get the pull in the dynamic input passed in at call time, and pass back that same value as output. All SPROC I/O uses the [`Value`](https://protobuf.dev/reference/protobuf/google.protobuf/#value) protobuf message type.

```rust
use host::{get_input, set_output, Value};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let val: Option<Value> = get_input()?;

    if let Some(val) = val {
        set_output(Some(val))?;
    }

    Ok(())
}
```

To test the procedure as a one-off against your Bicycle server

```bash
bicycle sproc oneoff ./my-proc --lang rust --addr http://0.0.0.0:50051
```

To store the procedure on your Bicycle server for future execution

```bash
bicycle sproc deploy ./my-proc --name my-proc --lang rust --addr http://0.0.0.0:50051
```

To execute a previously stored procedure on your Bicycle server

```bash
bicycle sproc deploy --name my-proc --addr http://0.0.0.0:50051
```
