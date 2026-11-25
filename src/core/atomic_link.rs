use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

/// An atomic raw-pointer link for lock-free data-structure prototypes.
///
/// The type only publishes and compares pointer values; it does not own or
/// dereference the pointed-to object. Callers must arrange safe reclamation
/// before freeing an object that may still be observed by another thread.
pub struct AtomicLink<T> {
    pointer: AtomicPtr<T>,
}

impl<T> AtomicLink<T> {
    pub const fn null() -> Self {
        Self {
            pointer: AtomicPtr::new(ptr::null_mut()),
        }
    }

    pub fn from_raw(pointer: *mut T) -> Self {
        Self {
            pointer: AtomicPtr::new(pointer),
        }
    }

    pub fn load(&self) -> *mut T {
        self.pointer.load(Ordering::SeqCst)
    }

    pub fn compare_exchange(&self, current: *mut T, next: *mut T) -> Result<*mut T, *mut T> {
        self.pointer
            .compare_exchange(current, next, Ordering::SeqCst, Ordering::SeqCst)
    }
}
