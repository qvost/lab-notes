# SLUB Allocator: Freelist Pointer Protection and Cache Line Locality

> **Domain**: `linux-kernel` | **Author**: Siddid Pandey <qvostwon@gmail.com>
> **Status**: Active Systems Research Note

## Summary
Freelist pointer obfuscation and per-CPU cache mechanics in the SLUB allocator

## Freelist Pointer Obfuscation

To mitigate heap exploitation primitives (such as use-after-free and freelist overwrite), SLUB obfuscates the next-pointer stored within idle slab objects.

```c
static inline void *freelist_ptr_decode(const struct kmem_cache *s,
                                       void *ptr_addr,
                                       unsigned long ptr_val)
{
    return (void *)(ptr_val ^ s->random ^ swab((unsigned long)ptr_addr));
}
```

This transformation ensures that an attacker cannot write a raw memory pointer into an idle chunk without knowing the per-cache 64-bit random canary.

## Per-CPU Slab Operations

Fast-path allocations operate on the `kmem_cache_cpu` structure without global lock acquisition. The CPU claims the local freelist pointer via a single atomic cmpxchg instruction, falling back to page-level locking only when the local cache is exhausted.

## References & Kernel Invariants
- Architectural specifications and verified tracepoints.
- Empirically verified under modern 64-bit systems architecture.

### Empirical Profile Log (Revision 2)
- **Cache Locality**: 64-byte aligned data layout minimizing cross-boundary cache line misses.
- **PMU Observation**: Invariant verified under hardware performance counter sampling.
