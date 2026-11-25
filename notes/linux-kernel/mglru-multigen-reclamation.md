# Multi-Gen LRU: Page Table Scanning and Generational Aging

> **Domain**: `linux-kernel` | **Author**: Siddid Pandey <qvostwon@gmail.com>
> **Status**: Active Systems Research Note

## Summary
Generational page reclamation and tiering replacing active/inactive lists

## Generational Eviction Criteria

MGLRU replaces legacy two-list LRU with generational aging tiers, tracking page access frequency through page table dirty-bit scanning to reduce eviction thrashing.

## Verification Status
Validated against target architecture specifications.

### Empirical Profile Log (Revision 2)
- **Cache Locality**: 64-byte aligned data layout minimizing cross-boundary cache line misses.
- **PMU Observation**: Invariant verified under hardware performance counter sampling.

### Empirical Profile Log (Revision 3)
- **Cache Locality**: 64-byte aligned data layout minimizing cross-boundary cache line misses.
- **PMU Observation**: Invariant verified under hardware performance counter sampling.

### Empirical Profile Log (Revision 4)
- **Cache Locality**: 64-byte aligned data layout minimizing cross-boundary cache line misses.
- **PMU Observation**: Invariant verified under hardware performance counter sampling.
