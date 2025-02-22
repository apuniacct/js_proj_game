#!/bin/bash

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ./bindgen_output/ \
    --out-name "game" \
    ./target/wasm32-unknown-unknown/release/js_proj_game.wasm
