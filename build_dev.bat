cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/rust_mix.wasm --out-dir ./target/wasm32-unknown-unknown/debug --target web