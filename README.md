# CLI Commands

## Watch Dev With Inspector Tools

```sh
cargo watch -c -w src -s 'cargo run --target wasm32-unknown-unknown --features inspector'
```

## Watch Release With Inspector Tools

```sh
cargo watch -c -w src -s 'cargo run --target wasm32-unknown-unknown --release --features inspector'
```

## Watch Dev

```sh
cargo watch -c -w src -s 'cargo run --target wasm32-unknown-unknown'
```

## Watch Release

```sh
cargo watch -c -w src -s 'cargo run --target wasm32-unknown-unknown --release'
```

## Run Dev

```sh
cargo run --target wasm32-unknown-unknown
```

## Run Release

```sh
cargo run --target wasm32-unknown-unknown --release
```

## Build Dev

```sh
cargo build --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/bevy-game.wasm
```

## Build Release

```sh
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/bevy-game.wasm
```
