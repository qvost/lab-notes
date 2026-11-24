# Product Quantization: Subspace Decomposition and Asymmetric Distance

> **Domain**: `vector-quantization` | **Author**: Siddid Pandey <qvostwon@gmail.com>

### Summary
Vector subspace decomposition and Asymmetric Distance Computation (ADC)

### Subspace Orthogonal Decomposition
High-dimensional space $\mathbb{R}^D$ is partitioned into $M$ orthogonal subspaces, encoding sub-vectors as compact 8-bit indices.

### Invariants
Validated against target 64-bit systems architecture specifications.

### Empirical Profile Log (Revision 2)
- **Cache Locality**: 64-byte aligned data layout minimizing cross-boundary cache line misses.
- **PMU Observation**: Invariant verified under hardware performance counter sampling.
