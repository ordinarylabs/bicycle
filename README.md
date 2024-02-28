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

## Usage

A Bicycle schema is defined in a simple `.proto` file like so:

```proto
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

We don't distribute the binary yet but if you clone down this repository you can play around with it:

```bash
## clone
git clone git@github.com:ordinarylabs/bicycle.git && cd bicycle

## generate your `./out/server` and `./out/bicycle.proto`
cargo run -- create schema.proto

## (feel free to edit the `schema.proto`, locally, as your "playground")
```

That will create a server binary and proto file for your consuming services. So in the `out/` you'll have `server` and `bicycle.proto`.

Now, the `bicycle.proto` can be used to codegen the client in any language. 

## Running

Once RocksDB is finally done building, you should be able to run the server with:

```bash
./out/server
```

## Clients

When you run the `create` command, it will take in your `schema.proto` and produce an `./out/bicycle.proto` that looks something like this:

```proto
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

Because the database server is just a gRPC server, you can use all native gRPC libraries for any language you like.
and you can also roll over to your preferred gRPC GUI client, type in `localhost::50051`, _AND_ because we implement
server reflection, when you plug in the URL it will automatically load up all your available RPCs (assuming your client GUI supports that).

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
  "pk": "DOG#1",
  "name": "Rover",
  "age": 3,
  "breed": "Golden Retriever"
}' localhost:50051 bicycle.Bicycle.PutDog

## BatchPutDogs
grpcurl -plaintext -d '{
  "dogs": [
    {
      "pk": "DOG#2",
      "name": "Buddy",
      "age": 2,
      "breed": "Labrador"
    },
    {
      "pk": "DOG#3",
      "name": "Max",
      "age": 4,
      "breed": "Poodle"
    }
  ]
}' localhost:50051 bicycle.Bicycle.BatchPutDogs

## GetDogs
grpcurl -plaintext -d '{"begins_with": "DOG#"}' localhost:50051 bicycle.Bicycle.GetDogsByPk

## DeleteDogs
grpcurl -plaintext -d '{"eq": "DOG#3"}' localhost:50051 bicycle.Bicycle.DeleteDogsByPk
```
