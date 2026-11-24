# io_uring Architecture: Lock-Free Submission and Completion Rings

> **Domain**: `linux-kernel` | **Author**: Siddid Pandey <qvostwon@gmail.com>
> **Status**: Active Systems Research Note

## Summary
Memory-mapped ring mechanics and kernel polling in io_uring

## Ring Buffer Layout

The `io_uring` interface communicates through two memory-mapped circular queues shared between user-space and the kernel:

1. **Submission Queue (SQ)**: Userspace writes SQEs (Submission Queue Entries) at `sq.tail` and the kernel consumes at `sq.head`.
2. **Completion Queue (CQ)**: Kernel writes CQEs (Completion Queue Entries) at `cq.tail` and userspace consumes at `cq.head`.

Single-producer single-consumer semantics on each ring eliminate cross-boundary locking overhead.

## SQPOLL Kernel Polling Mode

When configured with `IORING_SETUP_SQPOLL`, a dedicated kernel thread continuously polls the SQ ring tail. Applications submit asynchronous I/O requests with zero system call context switches, achieving sub-microsecond submission overheads.

## References & Kernel Invariants
- Architectural specifications and verified tracepoints.
- Empirically verified under modern 64-bit systems architecture.

### Empirical Profile Log (Revision 2)
- **Cache Locality**: 64-byte aligned data layout minimizing cross-boundary cache line misses.
- **PMU Observation**: Invariant verified under hardware performance counter sampling.
