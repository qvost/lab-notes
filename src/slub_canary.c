/**
 * SLUB Allocator Freelist Pointer Obfuscation
 * Reference: Linux kernel mm/slub.c
 * Authorship: Siddid Pandey <qvostwon@gmail.com>
 */

#include <stdint.h>
#include <stddef.h>

static inline uint64_t swab64(uint64_t val) {
    return __builtin_bswap64(val);
}

/**
 * Obfuscates a freelist pointer before writing into an idle slab object.
 */
uintptr_t freelist_ptr_encode(uintptr_t ptr_val, uintptr_t ptr_addr, uint64_t random_canary) {
    return ptr_val ^ random_canary ^ swab64((uint64_t)ptr_addr);
}

/**
 * Recovers real pointer address during slab allocation.
 */
void *freelist_ptr_decode(uintptr_t encoded_val, uintptr_t ptr_addr, uint64_t random_canary) {
    return (void *)(encoded_val ^ random_canary ^ swab64((uint64_t)ptr_addr));
}
