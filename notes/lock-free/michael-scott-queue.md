# Michael-Scott FIFO Queue: Non-Blocking Concurrent Pointer Manipulation

> **Domain**: `lock-free` | **Author**: Siddid Pandey <qvostwon@gmail.com>

### Summary
Two-pointer CAS queue with assisting mechanics for lagging tail updates

### Architectural Invariants
- Formally verified state transition bounds.
- Cache line alignment adhering to 64-byte SMP architecture boundaries.
- Empirically verified under modern 64-bit systems architecture.
