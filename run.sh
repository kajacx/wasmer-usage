#!/usr/bin/sh

cd wasmer2-plugin && \
cargo build --target=wasm32-unknown-unknown && \
cd .. && \
\
cd wasmer3-runtime && \
cargo run && \
cd .. && \
\
echo "All done"
