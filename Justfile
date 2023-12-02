watch-debug:
  cargo watch -c -w src -s 'cargo run --target wasm32-unknown-unknown --features inspector'

watch-release:
  cargo watch -c -w src -s 'cargo run --release --target wasm32-unknown-unknown --features inspector'

run-debug:
  cargo run --target wasm32-unknown-unknown --features inspector

run-release:
  cargo run --release --target wasm32-unknown-unknown --features inspector

build-debug:
  cargo build --features inspector

build-release:
  cargo build --release --target wasm32-unknown-unknown


