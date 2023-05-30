#!/usr/bin/env bash

curl -vL https://github.com/bytecodealliance/wasmtime/releases/download/dev/wasi_snapshot_preview1.reactor.wasm --output wasi_snapshot_preview1.reactor.wasm
cargo build --release --target wasm32-wasi -p wasm_lib
cargo run --release

curl https://wasmtime.dev/install.sh -sSf | bash
cargo build --release --target wasm32-wasi -p wasm_module
~/.wasmtime/bin/wasmtime target/wasm32-wasi/release/wasm_module.wasm