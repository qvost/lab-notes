# Research Digest Index

*Autonomous laboratory index tracking systems engineering digests and empirical analyses.*

**Total Commits Recorded**: 5 | **Total Engine Runs**: 2 | **Last Updated**: 2026-11-22T14:33:50.692Z

---

### Linux Kernel Internals

- [EEVDF Scheduling: Lag Calculations and Virtual Deadlines](notes/linux-kernel/eevdf-virtual-deadlines.md) — *Virtual deadline assignment and lag tracking in the EEVDF scheduler* (Completed, rev 1)
- [SLUB Allocator: Freelist Pointer Protection and Cache Line Locality](notes/linux-kernel/slub-allocator-hardening.md) — *Freelist pointer obfuscation and per-CPU cache mechanics in the SLUB allocator* (Completed, rev 1)
- [io_uring Architecture: Lock-Free Submission and Completion Rings](notes/linux-kernel/io-uring-kernel-submission.md) — *Memory-mapped ring mechanics and kernel polling in io_uring* (Completed, rev 1)

### Lock-Free & Concurrency Primitives

- [Safe Memory Reclamation: Hazard Pointer Lifecycle and Scanning Bounds](notes/lock-free/hazard-pointers-smr.md) — *Hazard pointer protection invariants and batch memory retirement bounds* (Drafting, rev 0)
- [Hardware Memory Orderings: Acquire-Release Semantics and CPU Pipelines](notes/lock-free/acquire-release-memory-barriers.md) — *Store buffer draining, invalidation queues, and acquire/release barriers* (Completed, rev 1)
- [LMAX Disruptor: Mechanical Sympathy and False-Sharing Elimination](notes/lock-free/disruptor-ring-buffer.md) — *Cache-line padding and lock-free sequence arbitration in ring buffers* (Completed, rev 1)

### Vector Quantization & Indexing

- [Product Quantization: Subspace Decomposition and Asymmetric Distance](notes/vector-quantization/product-quantization-decomposition.md) — *Vector subspace decomposition and Asymmetric Distance Computation (ADC)* (Drafting, rev 0)
- [SIMD Acceleration: AVX-512 VNNI Vectorized Distance Kernels](notes/vector-quantization/simd-avx512-distance-kernels.md) — *AVX-512 vectorization and VNNI dot-product kernels for high-dimensional vectors* (Drafting, rev 0)

### Rust Low-Level Systems & Memory

- [Rust Memory Representation: Struct Layout, Padding, and Enum Niches](notes/rust-memory/struct-padding-and-niche-filling.md) — *Field reordering rules, memory alignment padding, and niche optimization* (Drafting, rev 0)
- [Custom Allocation: Chunked Arenas and Monotonic Bump Pointers](notes/rust-memory/zero-copy-arena-allocators.md) — *Monotonic bump allocation and cache-local memory arenas in systems Rust* (Drafting, rev 0)

