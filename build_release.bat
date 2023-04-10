cargo build --release --target wasm32-unknown-unknown
wasm-bindgen ./target/wasm32-unknown-unknown/release/rust_mix.wasm --out-dir ./target/wasm32-unknown-unknown/release --target web