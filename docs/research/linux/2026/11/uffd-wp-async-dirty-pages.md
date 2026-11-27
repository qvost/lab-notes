# Uffd Wp Async Dirty Pages

- **Domain:** `linux`
- **Created:** `2026-11-27 20:23:47 +0545`

## Summary
Tracking dirty page bitmaps via userfaultfd write-protection for sub-millisecond CoW restores.

## Invariants & Observations
- Verified memory ordering and cache-line alignment boundaries at `2026-11-27 20:23:47 +0545`.

### Revision (2026-11-28 02:52:41 +0545)
- Re-validated boundary conditions for `uffd-wp-async-dirty-pages` under contention.
