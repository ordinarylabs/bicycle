# Bicycle 🚲

The high level goals of this project are to create a database which is, simple and fast. In order to achieve these goals 
it will be built in Rust, atop RocksDB and exist as a gRPC server.

For more information and a better writeup, look here: https://problemchild.engineering/2023-08-19-the-database

For some really imprecise (just run on my local with [ghz](https://ghz.sh)), initial benchmarks you can also look here: https://hachyderm.io/@seanwatters/110948629015247923 

## Usage

Usage is super clunky rn bc this lib only has one consumer (me). But if you're interested in trying it,
run the following:

```bash
cargo run --package bicycle_cli -- create cli/test.proto
```

That will create a server binary and proto file for your consuming services. So in the `cli/out/` you'll have `server` and `bicycle.proto`.

The `bicycle.proto` is what any developer who is familiar with gRPC can use to code-gen and build a client to the database. Right now, the database
is _very_ light weight and has _no_ administration infrastructure, permissions or auth; I get away with this because I'm only ever running it in private
subnets within the same VPCs on AWS and stuff. But there is always room for evolution. 

## Running

Once RocksDB is finally done building (holy fuck that takes way too long and I need to figure out how to like cache it or some do some 
other Rustacean magic to make it stop), you should be able to run the server with:

```bash
./out/server
```

## Clients

```bash
./out/bicycle.proto
```

Because the database server is just a gRPC server, you can use all native gRPC libraries for any language you like.
and you can also roll over to your preferred gRPC GUI client, type in `localhost::50051`, _AND_ because we implement
server reflection, when you plug in the URL it will automatically load up all your available RPCs (assuming your client GUI supports that).

## Example

> from blog post

The simplest example here (because right now it's "just a kv store" and I'm not gonna explain STD to everyone rn) is just 1 model,
so we're gonna pick `Dog`.

### Schema

```proto
// cli/test.proto
syntax = "proto3";
package bicycle;

message Dog {
  string pk = 1;

  string name = 2;
  uint32 age = 3;
  string breed = 4;
}
```

If we run our above commands to generate the `./out/server` and `./out/bicycle.proto` we can get the actually useful proto definition for our client (`./out/bicycle.proto`):


```bash
cargo run --package bicycle_cli -- create cli/test.proto
```

## Proto

When we run our script with the `schema.proto`, it barfs out this new proto (`bicycle.proto`). In this new proto, are the actual primitives for database
interaction and has all the fun stuff we need to do in application-land. 

This file shouldn't ever need to be modified by the developer directly and could break a lotta stuff. 

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

## Client

I was gonna put together a full on code example for every language but that's exhausting and `grpcurl` gives you the idea.

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

## Contributing

there isn't really a thing right now. it's kinda just one person rn with a few others maybe joining. we gotta set up a CLA or something i think. but check back in.
