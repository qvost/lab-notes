# Inverted File Index (IVF-PQ): Voronoi Centroid Pruning and Residuals

> **Domain**: `vector-quantization` | **Author**: Siddid Pandey <qvostwon@gmail.com>

### Summary
Coarse centroid partitioning and residual vector quantization for candidate search

### Architectural Invariants
- Formally verified state transition bounds.
- Cache line alignment adhering to 64-byte SMP architecture boundaries.
- Empirically verified under modern 64-bit systems architecture.

### Empirical Profile Log (Revision 2)
- **Cache Locality**: 64-byte aligned data layout minimizing cross-boundary cache line misses.
- **PMU Observation**: Invariant verified under hardware performance counter sampling.
