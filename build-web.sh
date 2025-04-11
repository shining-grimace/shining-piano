#! /bin/bash

CARGO_PROFILE="wasm-release"

# Prerequisites
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo install basic-http-server

# Build WASM binary
cargo build \
    --profile "$CARGO_PROFILE" \
    -p shining-piano-web \
    --target wasm32-unknown-unknown

# Move WASM binary to the web app directory
cp \
    "target/wasm32-unknown-unknown/$CARGO_PROFILE/shining_piano_web.wasm" \
    wasm-demo/target/

# Generate JS bindings for the WASM binary
wasm-bindgen --out-name bindings \
    --out-dir wasm-demo/target \
    --target web \
    target/wasm32-unknown-unknown/release/shining_piano_web.wasm

# Copy assets into the web app directory
rm -rf wasm-demo/assets
cp -r assets/ wasm-demo/assets

basic-http-server wasm-demo

