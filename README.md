# Shaligram Labs: Systems Research Digest

[![Domain: Systems Engineering](https://img.shields.io/badge/domain-systems_research-0f172a.svg)](https://github.com/qvost/lab-notes)
[![Focus: Kernel & Concurrency](https://img.shields.io/badge/focus-kernel%20%7C%20concurrency%20%7C%20simd-047857.svg)](https://github.com/qvost/lab-notes)
[![Stack: Rust / C / ASM](https://img.shields.io/badge/stack-Rust%20%7C%20C%20%7C%20ASM%20%7C%20eBPF-black.svg)](https://github.com/qvost/lab-notes)
[![Status: Active](https://img.shields.io/badge/status-active_research-2563eb.svg)](https://github.com/qvost/lab-notes)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A living technical digest and systems engineering notebook documenting low-level architectures, hardware-software co-design, non-blocking synchronization, high-dimensional vector search, and cache-conscious memory layouts.

Curated by **Siddid Pandey** ([qvostwon@gmail.com](mailto:qvostwon@gmail.com)) at **Shaligram Labs**.

---

## Research Streams

### 1. Linux Kernel Internals (`notes/linux-kernel/`)
- **Scheduler Mechanics**: EEVDF (Earliest Eligible Virtual Deadline First) virtual deadline calculations, task lag tracking ($v - V$), and latency-sensitive scheduling slices.
- **Memory & Allocator Subsystems**: SLUB freelist pointer obfuscation (`freelist_ptr_encode`), per-CPU slab caches, and page cache tiering with Multi-Gen LRU (MGLRU).
- **Asynchronous I/O Interfaces**: `io_uring` lock-free submission and completion rings, memory-mapped ring buffers, kernel polling threads (`IORING_SETUP_SQPOLL`), and fixed registered buffers.
- **Kernel Observability & Safety**: eBPF verifier abstract interpretation, 32/64-bit scalar bounds tracking, and BPF CO-RE (Compile Once – Run Everywhere).

### 2. Lock-Free & Non-Blocking Concurrency (`notes/lock-free/`)
- **Safe Memory Reclamation (SMR)**: Hazard pointer publishing and scanning bounds vs Epoch-Based Reclamation (EBR) mechanics.
- **Hardware Memory Models**: x86-64 TSO vs ARM/RISC-V weak memory ordering, store buffer forwarding, invalidation queues, and acquire/release barriers.
- **Concurrent Ring Buffers & Queues**: LMAX Disruptor cache-line padding (false sharing prevention), power-of-two mask indexing, and Michael-Scott lock-free FIFO queues.
- **ABA Problem Resolution**: Tagged generation counters, 128-bit atomic double-word compare-and-swap (`cmpxchg16b`), and pointer packing.

### 3. High-Dimensional Vector Quantization (`notes/vector-quantization/`)
- **Product Quantization (PQ)**: Orthogonal subspace decomposition, codebook generation via Lloyd's k-means, and Asymmetric Distance Computation (ADC).
- **Inverted File Indexes (IVF-PQ)**: Coarse Voronoi cell partitioning, residual vector quantization, and dynamic `nprobe` candidate pruning.
- **SIMD Distance Kernels**: AVX-512 and ARM NEON vectorized Euclidean ($L_2$) and dot product kernels utilizing Intel VNNI (`_mm512_dpbusd_epi32`).
- **Graph & Anisotropic Search**: Hierarchical Navigable Small World (HNSW) heuristic routing and ScaNN directional quantization loss formulation.

### 4. Rust Low-Level Systems & Memory Layouts (`notes/rust-memory/`)
- **Data Representation & Alignment**: `repr(Rust)` field reordering, `repr(C)` ABI compatibility, struct field alignment padding, and `repr(packed)` hardware trap hazards.
- **Niche Exploitation**: Zero-cost `Option<T>` representation using non-null pointer niches and custom multi-variant niche optimization.
- **Custom Memory Allocators**: Monotonic bump allocators, chunked slab arenas, and scoped memory lifecycles.
- **Zero-Cost Abstractions & Inline Assembly**: `core::arch::asm!`, register operand constraints, and uninitialized memory safety with `MaybeUninit<T>`.

---

## Directory Organization

```text
lab-notes/
├── notes/
│   ├── linux-kernel/          # Kernel architecture specifications and tracepoints
│   ├── lock-free/             # Non-blocking synchronization and SMR invariants
│   ├── vector-quantization/   # High-dimensional quantization and SIMD kernels
│   └── rust-memory/           # Low-level layout, padding, and allocator designs
├── benchmarks/                # Hardware PMU logs and micro-benchmark observations
├── INDEX.md                   # Full categorized research index & revision tracking
├── README.md                  # Project overview & research streams
└── LICENSE                    # MIT License
```

---

## Contributing & Research Standards

All notes adhere to structured systems documentation standards:
1. **Mathematical / Invariant Formalization**: Clear bounds, state transitions, and concurrency guarantees.
2. **Empirical Calibration**: Hardware PMU cycle counts, cache miss profiles, and assembly verification (`objdump -M intel`).
3. **Reference Implementations**: C23, Rust, and ASM micro-kernels verified on modern 64-bit platforms.

---

## License

This research repository is licensed under the [MIT License](LICENSE).
