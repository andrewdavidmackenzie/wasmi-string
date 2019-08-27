run: wasm/target/wasm32-unknown-unknown/debug/test.wasm
	cargo run

target/wasm32-unknown-unknown/debug/test.wasm: wasm/src/test.rs
	cd wasm;cargo build