//! Monotonic Cache-Aligned Bump Allocator
//! Authorship: Siddid Pandey <qvostwon@gmail.com>

use std::alloc::Layout;

/// Non-thread-safe linear bump allocator optimized for uniform lifecycles.
pub struct BumpArena {
    storage: *mut u8,
    cursor: usize,
    capacity: usize,
}

impl BumpArena {
    /// Creates a bump arena with pre-allocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let layout = Layout::from_size_align(capacity, 64).expect("valid layout");
        let storage = unsafe { std::alloc::alloc(layout) };
        assert!(!storage.is_null(), "memory allocation failed");
        Self {
            storage,
            cursor: 0,
            capacity,
        }
    }

    /// Allocates memory adhering to layout alignment requirements.
    #[inline(always)]
    pub fn alloc(&mut self, layout: Layout) -> Option<*mut u8> {
        let align = layout.align();
        let size = layout.size();
        let current_addr = (self.storage as usize) + self.cursor;
        let aligned_addr = (current_addr + align - 1) & !(align - 1);
        let new_cursor = (aligned_addr - (self.storage as usize)) + size;

        if new_cursor <= self.capacity {
            self.cursor = new_cursor;
            Some(aligned_addr as *mut u8)
        } else {
            None
        }
    }

    /// Resets bump pointer without deallocating backing buffer.
    #[inline(always)]
    pub fn reset(&mut self) {
        self.cursor = 0;
    }
}

impl Drop for BumpArena {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.capacity, 64).expect("valid layout");
        unsafe { std::alloc::dealloc(self.storage, layout) };
    }
}
