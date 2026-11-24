# ARM NEON Distance Kernels: 128-Bit Vector Dot Products for aarch64

> **Domain**: `vector-quantization` | **Author**: Siddid Pandey <qvostwon@gmail.com>

### Summary
Vectorized float32x4_t and sdot int8 kernels for ARM Cortex / Apple Silicon

### Architectural Invariants
- Formally verified state transition bounds.
- Cache line alignment adhering to 64-byte SMP architecture boundaries.
- Empirically verified under modern 64-bit systems architecture.
