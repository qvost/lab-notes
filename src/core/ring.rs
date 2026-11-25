use std::collections::VecDeque;
use std::sync::Mutex;

/// A fixed-capacity FIFO queue with a mutex-protected ring-like backing store.
///
/// This implementation is safe for multiple producers and consumers. It is
/// deliberately not lock-free; use it as a correctness baseline when
/// comparing a later atomic SPSC implementation.
pub struct BoundedQueue<T> {
    capacity: usize,
    items: Mutex<VecDeque<T>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueError {
    ZeroCapacity,
}

impl<T> BoundedQueue<T> {
    pub fn new(capacity: usize) -> Result<Self, QueueError> {
        if capacity == 0 {
            return Err(QueueError::ZeroCapacity);
        }
        Ok(Self {
            capacity,
            items: Mutex::new(VecDeque::with_capacity(capacity)),
        })
    }

    /// Adds an item unless the queue has reached its configured capacity.
    pub fn try_push(&self, item: T) -> Result<(), T> {
        let mut items = self
            .items
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        if items.len() == self.capacity {
            return Err(item);
        }
        items.push_back(item);
        Ok(())
    }

    /// Removes the oldest item, or returns `None` when the queue is empty.
    pub fn try_pop(&self) -> Option<T> {
        self.items
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .pop_front()
    }

    pub fn len(&self) -> usize {
        self.items
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
