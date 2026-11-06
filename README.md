# Aethelarch Core

High-performance bit-sliced ternary SIMD engine for ARM64 NEON.

## Kernel
- `aethelarch_matvec_ternary`: 16-byte NEON vectorized ternary mat-vec (mask + sign).
- `sbe_attention_coincidence`: sparsity-gated bit coincidence primitive.

## Build & Bench
```bash
chmod +x run_bench.sh
./run_bench.sh
```

Requires aarch64 host with NEON. Release profile forces LTO + single codegen unit + abort-on-panic.

## C-FFI
Header: `include/aethelarch.h`  
Link against `libaethelarch` (cdylib or staticlib).
