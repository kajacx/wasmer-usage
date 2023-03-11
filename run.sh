#!/usr/bin/sh

cd wasmer2-plugin && \
cargo build --target=wasm32-unknown-unknown && \
cd .. && \
echo "All done"
