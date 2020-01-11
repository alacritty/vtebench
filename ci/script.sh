#!/bin/bash

# Run clippy checks
if [ "$CLIPPY" == "true" ]; then
    cargo clippy
    exit
fi

# Run clippy rustfmt
if [ "$RUSTFMT" == "true" ]; then
    cargo fmt -- --check
    exit
fi

# Run tests
cargo test
