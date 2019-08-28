# WASMI-String
Shows an example of how to pass String parameters to and from a WASM module (written in rust) 
from rust code, where wasm is interpreted using the WASMI wasm interpreter.

Since starting I have modified this to work without null-terminated strings sent and received from 
wasm. That was a first step to sending arbitrary data.

If you want to see an example of how to send arbitrary structure data across the native/wasm
boundary, using serde serialization and deserialization then you can look at another
repo of mine where I do that: https://github.com/andrewdavidmackenzie/wasm_explore

# Clone Repo
```
git clone https://github.com/andrewdavidmackenzie/wasmi-string
```

# Build and Run
This first builds the wasm module (in /wasm folder) from rust code there.
Then it builds and runs the program (in /main folder) that runs the wasm module using
WASMI.

```
$ cd wasmi-string
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
