#! /bin/bash

# Prerequisites:
# - `rustup target add wasm32-unknown-unknown`
# - `cargo install wasm-bindgen-cli`
# - `cargo install basic-http-server`

cargo build --release \
    -p shining-piano-web \
    --target wasm32-unknown-unknown

cp \
    target/wasm32-unknown-unknown/release/shining_piano_web.wasm \
    wasm-demo/target/

wasm-bindgen --out-name bindings \
    --out-dir wasm-demo/target \
    --target web \
    target/wasm32-unknown-unknown/release/shining_piano_web.wasm

basic-http-server wasm-demo

