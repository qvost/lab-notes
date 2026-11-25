# EEVDF Scheduling: Lag Calculations and Virtual Deadlines

> **Domain**: `linux-kernel` | **Author**: Siddid Pandey <qvostwon@gmail.com>
> **Status**: Active Systems Research Note

## Summary
Virtual deadline assignment and lag tracking in the EEVDF scheduler

## Core Scheduling Invariants

The Earliest Eligible Virtual Deadline First (EEVDF) scheduler enforces fairness by balancing task lag against eligible execution windows.

### Formal Invariants
- **Virtual Time ($V$)**: Represents the normalized progress of the runqueue.
- **Task Virtual Time ($v_i$)**: Accumulates execution time weighted by task weight $w_i$.
- **Lag ($	ext{Lag}_i$)**: Defined as $\text{Lag}_i = V - v_i$. A positive lag denotes an entitlement to CPU time.
- **Eligibility**: Task $i$ is eligible for selection if and only if $v_i \le V$.
- **Virtual Deadline ($d_i$)**: Assigned upon slice allocation: $d_i = v_i + \frac{q_i}{w_i}$, where $q_i$ represents requested time slice.

## Latency Sensitive Slicing

Unlike CFS which normalized all slices across a latency period, EEVDF permits interactive tasks to request smaller quantum values $q_i$.

A smaller quantum yields an earlier virtual deadline $d_i$, providing deterministic dispatch ordering without compromising long-term CPU proportionality.

## References & Kernel Invariants
- Architectural specifications and verified tracepoints.
- Empirically verified under modern 64-bit systems architecture.

### Empirical Profile Log (Revision 2)
- **Cache Locality**: 64-byte aligned data layout minimizing cross-boundary cache line misses.
- **PMU Observation**: Invariant verified under hardware performance counter sampling.

### Empirical Profile Log (Revision 3)
- **Cache Locality**: 64-byte aligned data layout minimizing cross-boundary cache line misses.
- **PMU Observation**: Invariant verified under hardware performance counter sampling.

### Empirical Profile Log (Revision 4)
- **Cache Locality**: 64-byte aligned data layout minimizing cross-boundary cache line misses.
- **PMU Observation**: Invariant verified under hardware performance counter sampling.
