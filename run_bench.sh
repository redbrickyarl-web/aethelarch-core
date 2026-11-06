#!/bin/bash
set -euo pipefail

export RUSTFLAGS="-C target-cpu=native -C target-feature=+neon"
cargo build --release

# Bind to performance cores if available (adjust for device)
if command -v taskset >/dev/null 2>&1; then
    taskset -c 4-7 ./target/release/aethelarch-core
else
    ./target/release/aethelarch-core
fi
