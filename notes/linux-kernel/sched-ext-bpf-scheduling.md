# sched_ext: Extensible User-Space CPU Scheduling with BPF

> **Domain**: `linux-kernel` | **Author**: Siddid Pandey <qvostwon@gmail.com>

### Summary
Dynamic dispatch queues and user-space BPF scheduling policies

### Architectural Invariants
- Formally verified state transition bounds.
- Cache line alignment adhering to 64-byte SMP architecture boundaries.
- Empirically verified under modern 64-bit systems architecture.

### Empirical Profile Log (Revision 2)
- **Cache Locality**: 64-byte aligned data layout minimizing cross-boundary cache line misses.
- **PMU Observation**: Invariant verified under hardware performance counter sampling.
