# Seccomp Unotify Openat Filter

- **Domain:** `engine`
- **Created:** `2026-11-27 20:23:47 +0545`

## Summary
Intercepting filesystem metadata syscalls in userspace without ptrace stop-the-world overhead.

## Invariants & Observations
- Verified memory ordering and cache-line alignment boundaries at `2026-11-27 20:23:47 +0545`.
