/**
 * Cache-Line Padded Ring Cursor (False-Sharing Mitigation)
 * Authorship: Siddid Pandey <qvostwon@gmail.com>
 */

#include <stdint.h>
#include <stddef.h>

/**
 * 64-byte aligned sequence cursor eliminating MESI cache line bouncing.
 */
typedef struct {
    uint64_t p1, p2, p3, p4, p5, p6, p7; // 56 bytes head padding
    _Atomic uint64_t sequence;            // 8 bytes payload
    uint64_t p8, p9, p10, p11, p12, p13, p14; // 56 bytes tail padding
} __attribute__((aligned(64))) PaddedCursor;

uint64_t read_cursor(const PaddedCursor *c) {
    return __atomic_load_n(&c->sequence, __ATOMIC_ACQUIRE);
}

void write_cursor(PaddedCursor *c, uint64_t val) {
    __atomic_store_n(&c->sequence, val, __ATOMIC_RELEASE);
}
