build-wasm:
  cargo build -p complogic-gates --target wasm32-unknown-unknown

run: build-wasm
  cargo run
