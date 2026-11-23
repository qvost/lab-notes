# LMAX Disruptor: Mechanical Sympathy and False-Sharing Elimination

> **Domain**: `lock-free` | **Author**: Siddid Pandey <qvostwon@gmail.com>
> **Status**: Active Systems Research Note

## Summary
Cache-line padding and lock-free sequence arbitration in ring buffers

## False Sharing and Cache Line Alignment

Modern CPU architectures move memory in 64-byte cache lines. When two cores concurrently mutate independent variables sharing the same 64-byte line, false sharing triggers costly MESI cache-coherence bus invalidations.

The Disruptor ring buffer eliminates false sharing by surrounding critical cursor sequence numbers with 56 bytes of explicit padding (7 `u64` integers).

## Power-of-Two Index Masking

Array index lookup uses bitwise masking instead of modulo arithmetic:
`index = sequence & (BUFFER_SIZE - 1)`
This translates to a single CPU clock cycle logical AND operation, avoiding the multicycle latency of the hardware division unit.

## References & Kernel Invariants
- Architectural specifications and verified tracepoints.
- Empirically verified under modern 64-bit systems architecture.

### Empirical Profile Log (Revision 2)
- **Cache Locality**: 64-byte aligned data layout minimizing cross-boundary cache line misses.
- **PMU Observation**: Invariant verified under hardware performance counter sampling.
