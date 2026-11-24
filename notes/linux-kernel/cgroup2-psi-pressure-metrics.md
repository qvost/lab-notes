# Pressure Stall Information: Kernel PSI Tracking for Memory and CPU

> **Domain**: `linux-kernel` | **Author**: Siddid Pandey <qvostwon@gmail.com>

### Summary
Resource pressure stall tracking across cgroup2 hierarchies

### Architectural Invariants
- Formally verified state transition bounds.
- Cache line alignment adhering to 64-byte SMP architecture boundaries.
- Empirically verified under modern 64-bit systems architecture.

### Empirical Profile Log (Revision 2)
- **Cache Locality**: 64-byte aligned data layout minimizing cross-boundary cache line misses.
- **PMU Observation**: Invariant verified under hardware performance counter sampling.

### Empirical Profile Log (Revision 3)
- **Cache Locality**: 64-byte aligned data layout minimizing cross-boundary cache line misses.
- **PMU Observation**: Invariant verified under hardware performance counter sampling.
