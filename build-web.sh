#! /bin/bash

# Prerequisites:
# - `rustup target add wasm32-unknown-unknown`
# - `cargo install wasm-bindgen-cli`

cargo build --release \
    -p shining-piano-web \
    --target wasm32-unknown-unknown

wasm-bindgen --out-name shining-piano-web \
    --out-dir web/target \
    --target web \
    target/wasm32-unknown-unknown/release/shining-piano-web.wasm


