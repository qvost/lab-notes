# Hardware Memory Orderings: Acquire-Release Semantics and CPU Pipelines

> **Domain**: `lock-free` | **Author**: Siddid Pandey <qvostwon@gmail.com>
> **Status**: Active Systems Research Note

## Summary
Store buffer draining, invalidation queues, and acquire/release barriers

## Hardware Memory Models

Out-of-order execution pipelines and store buffers reorder memory instructions to maximize IPC (Instructions Per Cycle).

- **x86-64 (TSO)**: Strong ordering. Loads are not reordered with other loads; stores are not reordered with earlier stores. Only Store-Load reordering occurs naturally via store buffers.
- **ARM / RISC-V**: Weak ordering. Independent memory operations can be freely reordered by the pipeline unless explicit DMB/FENCE barriers are placed.

## Acquire and Release Pairings

A store with `Release` ordering guarantees all prior memory accesses (both reads and writes) in the current thread complete before this store becomes visible.

A load with `Acquire` ordering guarantees subsequent memory accesses in the current thread cannot be reordered before this load.

## References & Kernel Invariants
- Architectural specifications and verified tracepoints.
- Empirically verified under modern 64-bit systems architecture.

### Empirical Profile Log (Revision 2)
- **Cache Locality**: 64-byte aligned data layout minimizing cross-boundary cache line misses.
- **PMU Observation**: Invariant verified under hardware performance counter sampling.
