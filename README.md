# WASM interpreter (Working with String)

# Build & Test

```
git clone https://github.com/hoangpq/wasmi-string
cd wasmi-string
make
```

# Result
```
$ make
cd wasm;cargo build
   Compiling test v0.1.0 (/Users/andrew/workspace/wasmi-string/wasm)
    Finished dev [unoptimized + debuginfo] target(s) in 0.56s
cargo run
   Compiling byteorder v1.3.1
   Compiling memory_units v0.3.0
   Compiling parity-wasm v0.31.3
   Compiling wasmi v0.4.3
   Compiling wasmi-string v0.1.0 (/Users/andrew/workspace/wasmi-string/main)
    Finished dev [unoptimized + debuginfo] target(s) in 10.36s
     Running `target/debug/wasmi-string`
Result is `42`
```
