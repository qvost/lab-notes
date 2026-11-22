//! Hazard Pointer Atomic Protection Slot
//! Authorship: Siddid Pandey <qvostwon@gmail.com>

use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

/// A single hazard pointer slot protecting a dynamically allocated node
/// from premature deallocation by concurrent reclaimer threads.
#[repr(align(64))]
pub struct HazardSlot<T> {
    hazard: AtomicPtr<T>,
}

impl<T> HazardSlot<T> {
    pub const fn new() -> Self {
        Self {
            hazard: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Publishes pointer `p` as active hazard to reader threads.
    #[inline(always)]
    pub fn protect(&self, p: *mut T) {
        self.hazard.store(p, Ordering::Release);
    }

    /// Clears hazard protection slot.
    #[inline(always)]
    pub fn clear(&self) {
        self.hazard.store(ptr::null_mut(), Ordering::Release);
    }

    /// Inspects slot value for scanning threads.
    #[inline(always)]
    pub fn get(&self) -> *mut T {
        self.hazard.load(Ordering::Acquire)
    }
}
