# RFC-004: Hazard pointer reclamation boundary

**Status:** Draft  
**Scope:** Single-object pointer protection for lock-free structure prototypes

## Summary

Hazard pointers let a reader announce which removed object it may still access.
A reclamation owner unlinks retired objects, scans published hazard slots, and
reclaims only objects that no slot protects. This RFC defines the boundary
between pointer publication and memory ownership for the prototypes in
`src/core/`.

## Invariants

1. A reader loads a candidate pointer, publishes it to its hazard slot, then
   reloads the source link and retries if the source changed.
2. A writer removes a node before adding it to the retired set.
3. Reclamation occurs only after a complete hazard scan finds no matching
   published pointer.
4. A hazard slot remains published until the reader finishes all accesses to
   the protected object.
5. A pointer publication primitive does not itself own or free the object.

The prototype currently uses sequentially consistent atomics for the source
link and hazard slot. This keeps the ordering contract explicit while the
reclamation registry is still out of scope. Any weaker ordering needs a
separate proof covering both the reader validation loop and writer scan.

## API boundary

`AtomicLink<T>` publishes and compares raw pointer values. `HazardSlot<T>`
implements the reader's load-publish-validate loop. Neither type dereferences
or reclaims memory. A future registry must own the full retired-list scan
protocol before callers can safely free nodes.

## Risks and open work

- A single slot protects only one pointer at a time.
- Address reuse can create ABA unless the enclosing structure adds tagged
  pointers or another generation mechanism.
- The current API is a teaching prototype; it is not a complete safe-memory
  reclamation implementation.
