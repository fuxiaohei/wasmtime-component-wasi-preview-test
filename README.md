# wasmtime-component-wasi-preview-test

wasmtime component wasi preview2 test code

## How to run

```bash
curl -vL https://github.com/bytecodealliance/preview2-prototyping/releases/download/latest/wasi_snapshot_preview1.wasm --output wasi_snapshot_preview1.wasm
cargo build --release --target wasm32-wasi -p wasm_lib
cargo run --release
```

It should not crash.
