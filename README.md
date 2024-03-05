# Bicycle 🚲

[![ci](https://github.com//ordinarylabs/bicycle/actions/workflows/ci.yml/badge.svg)](https://github.com//ordinarylabs/bicycle/actions/workflows/ci.yml)
[![license](https://img.shields.io/github/license/ordinarylabs/bicycle.svg)](https://github.com/ordinarylabs/bicycle/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/ordinarylabs/bicycle/status.svg)](https://deps.rs/repo/github/ordinarylabs/bicycle)

Bicycle is a framework for defining database schemas with protobuf such that access patterns are generated as code and compiled into the database server itself. The goal is to reduce bandwidth and the overhead of query/response parsing at run time by using a binary serialization format and empowering the compiler do query planning ahead of time.

## Why the name?

The Bicycle is a metaphor for useful complexity, and one of the more influential inventions in history. 
It is also an interesting analogy for the anatomy of the framework...

- Wheels (transport): gRPC
- Frame (storage engine): RocksDB
- Pedals, gears, handlebars, breaks, etc. (logic): Rust

## Install

Before installing `bicycle` you'll need to have [Rust](https://www.rust-lang.org/tools/install) and [protoc](https://grpc.io/docs/protoc-installation/) installed.

```bash
cargo install bicycle
```

## CLI Usage

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

Now in the `out/` directory you'll have `server` and `bicycle.proto`.

### Plugins

Plugins allow you to mix functionality into your Bicycle server. Plugins are just basic Rust libraries that export a gRPC file descriptor, `Server` and `Service` generated with [Tonic](https://github.com/hyperium/tonic); they can be added from the [crates.io](https://crates.io) registry, a local path, or a git repository.

Formatting for `--plugins` flag:

- `crates.io:plugin-name@0.0.0` 
- `path:plugin-name@../plugin-path` 
- `git:plugin-name@https:://github.com/user/plugin-name.git#rev:4c59b707|branch:next|tag:0.0.0`

```bash
## using the example from /plugin
bicycle create schema.proto --plugins crates.io:bicycle-plugin@0.1.1
```

## Running

You can now run the server binary with the following command.

```bash
./out/server
```

## Clients

You can also use the `./out/bicycle.proto` (see example output below) to build your database clients.

Because the Bicycle server is just a gRPC server, you can use the gRPC libraries for any language you like. Additionally, Bicycle servers implement [server reflection] you can also roll over to your preferred gRPC GUI client (i.e Postman), type in `0.0.0.0::50051`, and it will automatically load up all your available RPCs.

```protobuf
// out/bicycle.proto
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

## Example

Basically we have 4 RPCs for each model:

- `GetXByPk`
- `DeleteXByPk`
- `PutX`
- `BatchPutX`

And then you have the `IndexQuery` helper which basically allows you to do key-range queries. 

Here are the really basic examples:

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
