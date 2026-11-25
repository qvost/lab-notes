# Strata Core: Systems Research & Engineering Notebook

Personal laboratory notebook, micro-benchmarks, and implementation notes on low-level systems architectures, Linux kernel internals, non-blocking concurrency, and cache-conscious memory layouts.

Maintained by **Siddid Pandey** ([qvostwon@gmail.com](mailto:qvostwon@gmail.com)) at **Shaligram Labs**.

The repository now includes a dependency-free Rust crate, `strata-core`, for
small correctness-first systems prototypes. Queue and pointer APIs document
their synchronization and reclamation boundaries; timing and quantization
helpers are deterministic; benchmark records are added only after measurement.

Run the integration suite with `cargo test` and the queue baseline with
`cargo bench --bench bounded_queue`.

---

### Focus Areas

#### 1. Linux Kernel Internals (`notes/linux-kernel/`)
- **Scheduler**: EEVDF (Earliest Eligible Virtual Deadline First) lag calculation, eligibility bounds, and virtual deadline scheduling slices.
- **Memory Subsystems**: SLUB allocator freelist pointer XOR obfuscation, per-CPU cache slabs, and Multi-Gen LRU (MGLRU) page table scanning.
- **Async I/O**: `io_uring` lock-free SQ/CQ ring mechanics, kernel worker threads, and `IORING_SETUP_SQPOLL` mode.
- **eBPF**: Ring buffer zero-copy reservations and verifier scalar bounds tracking.

#### 2. Lock-Free & Concurrency Primitives (`notes/lock-free/`)
- **Safe Memory Reclamation (SMR)**: Hazard pointer publishing and scanning bounds vs Epoch-Based Reclamation (EBR) mechanics.
- **Memory Models**: Acquire-Release semantics, store-buffer forwarding, and compiler barriers on x86-64 vs ARM64.
- **Queue Designs**: Michael-Scott lock-free FIFO queue and LMAX Disruptor cache-line padded ring buffers.
- **ABA Resolution**: Generation counters and 128-bit atomic double-word CAS (`cmpxchg16b`).

#### 3. High-Dimensional Vector Search (`notes/vector-quantization/`)
- **Product Quantization (PQ)**: Subspace decomposition and Asymmetric Distance Computation (ADC) lookup tables.
- **Inverted File Indexes**: Coarse Voronoi cell partitioning and residual quantization.
- **SIMD Kernels**: AVX-512 and ARM NEON vectorized Euclidean ($L_2$) and dot product kernels utilizing Intel VNNI (`_mm512_dpbusd_epi32`).
- **Graph Indexing**: HNSW multi-layer skip-graph traversal and ScaNN anisotropic loss formulations.

#### 4. Rust Systems & Memory Layout (`notes/rust-memory/`)
- **Layout & Alignment**: `repr(Rust)` field reordering, struct padding, and enum niche discriminant packing.
- **Custom Allocators**: Monotonic bump allocators and chunked arena designs.
- **Memory Safety**: `MaybeUninit<T>` initialization patterns and `Pin<P>` contracts for intrusive structures.

---

### Repository Layout

```text
.
├── notes/                     # Topic notes, formal bounds, and implementation specs
│   ├── linux-kernel/
│   ├── lock-free/
│   ├── vector-quantization/
│   └── rust-memory/
├── src/                       # Rust crate and existing C/Rust prototypes
│   ├── core/                  # Bounded queue, atomic link, hazard slot
│   ├── engine/                # Virtual time and drift model
│   └── quantization/          # Scalar int8 and greedy graph traversal
│   ├── arena.rs               # Cache-aligned monotonic bump allocator
│   ├── hazard.rs              # Existing atomic hazard pointer record
│   ├── disruptor_ring.c       # False-sharing padded ring cursor
│   └── slub_canary.c          # SLUB freelist XOR pointer encode/decode
├── tests/
│   ├── integration/           # Invariant and concurrent queue checks
│   └── fixtures/              # Scheduler trace input
├── docs/
│   ├── rfc/                   # Architecture proposals
│   └── research/              # Cross-topic research summaries
├── benchmarks/
│   ├── results/               # Measured, dated results only
│   └── suites/                # Benchmark executable and metadata
├── Cargo.toml                # `strata-core` crate manifest
├── CHANGELOG.md
├── benches/                   # Hardware-calibrated micro-benchmarks
│   └── simd_l2.rs             # Scalar vs AVX2 vs AVX-512 L2 distance
├── benchmarks/data/           # Hardware PMU logs and latency profiles
│   ├── zen4_pmu_counters.json
│   └── hazard_ptr_latency.csv
└── INDEX.md                   # Chronological research index
```

---

### Hardware Testbed & Reproducibility

Benchmarks and PMU performance counters are gathered across two reference machines:
- **Workstation**: AMD Ryzen 9 7950X (Zen 4, 16c/32t), 64GB DDR5-6000 CL30, Linux 6.8 (Ubuntu 24.04 LTS).
- **Server**: Intel Xeon Platinum 8480+ (Sapphire Rapids, AVX-512 + VNNI), Ubuntu 22.04 LTS.
- **Compiler Flags**: `RUSTFLAGS="-C target-cpu=native"` / `gcc -O3 -march=native -Wall -Wextra`.

---

### License

MIT License. See [LICENSE](LICENSE) for details.
