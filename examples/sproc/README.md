# Tests for the Bicycle Framework

## SPROC

With the `./server` xor `./sproc` servers running you can test using the following command.

```
cargo run -- sproc oneoff ./examples/proc --lang rust --addr http://0.0.0.0:50051
```