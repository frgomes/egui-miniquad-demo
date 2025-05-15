#!/bin/bash -eux

EXAMPLE_NAME="demo"

rustup target add wasm32-unknown-unknown

# Release:
cargo build --release --example ${EXAMPLE_NAME} --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/examples/${EXAMPLE_NAME}.wasm docs/

# # Debug:
# cargo build --example ${EXAMPLE_NAME} --target wasm32-unknown-unknown
# cp target/wasm32-unknown-unknown/debug/examples/${EXAMPLE_NAME}.wasm docs/

# brew install wabt # to get wasm-strip
wasm-strip docs/${EXAMPLE_NAME}.wasm
