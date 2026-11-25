use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

use super::AtomicLink;

/// One hazard-pointer publication slot.
///
/// `protect` uses a load-publish-validate loop. A reclamation owner must first
/// unlink a node, then scan every slot with the same sequentially consistent
/// protocol, and reclaim only nodes absent from that scan. This type does not
/// own a registry or free memory on its own.
pub struct HazardSlot<T> {
    protected: AtomicPtr<T>,
}

impl<T> HazardSlot<T> {
    pub const fn new() -> Self {
        Self {
            protected: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Publishes a pointer and retries if the source changed during publication.
    pub fn protect(&self, source: &AtomicLink<T>) -> *mut T {
        loop {
            let candidate = source.load();
            self.protected.store(candidate, Ordering::SeqCst);
            if source.load() == candidate {
                return candidate;
            }
        }
    }

    pub fn protects(&self, candidate: *mut T) -> bool {
        self.protected.load(Ordering::SeqCst) == candidate
    }

    pub fn clear(&self) {
        self.protected.store(ptr::null_mut(), Ordering::SeqCst);
    }
}

impl<T> Default for HazardSlot<T> {
    fn default() -> Self {
        Self::new()
    }
}
