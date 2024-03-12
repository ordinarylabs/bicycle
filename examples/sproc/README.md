# SPROC Example

With `./server` running you can test with the following commands.

## Invoke One-off

```
cargo run -- fn invoke --addr http://0.0.0.0:50051 --lang rust --path ./examples/sproc --args '{"begins_with": ""}'
```

## Deploy Procedure

```
cargo run -- fn deploy --addr http://0.0.0.0:50051 --lang rust --path ./examples/sproc --name test  
```

## Invoke Deployed Procedure

```
cargo run -- fn invoke --addr http://0.0.0.0:50051 --name test --args '{"begins_with": ""}'  
```