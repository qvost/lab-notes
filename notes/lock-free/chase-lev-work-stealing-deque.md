# Chase-Lev Deque: Lock-Free Dynamic Array Work-Stealing

> **Domain**: `lock-free` | **Author**: Siddid Pandey <qvostwon@gmail.com>

### Summary
Single-producer multi-consumer dynamic circular array stealing

### Architectural Invariants
- Formally verified state transition bounds.
- Cache line alignment adhering to 64-byte SMP architecture boundaries.
- Empirically verified under modern 64-bit systems architecture.
