use std::collections::HashSet;
use std::sync::Arc;
use std::thread;

use strata_core::core::{BoundedQueue, QueueError};
use strata_core::engine::{ClockError, VirtualClock};
use strata_core::quantization::{greedy_layer, squared_l2, QuantizedVector};

#[test]
fn bounded_queue_preserves_fifo_and_capacity() {
    let queue = BoundedQueue::new(2).unwrap();
    assert_eq!(queue.try_pop(), None);
    assert_eq!(queue.try_push(10), Ok(()));
    assert_eq!(queue.try_push(20), Ok(()));
    assert_eq!(queue.try_push(30), Err(30));
    assert_eq!(queue.try_pop(), Some(10));
    assert_eq!(queue.try_pop(), Some(20));
    assert_eq!(queue.try_pop(), None);
    assert_eq!(queue.capacity(), 2);
}

#[test]
fn bounded_queue_rejects_zero_capacity() {
    assert!(matches!(
        BoundedQueue::<u8>::new(0),
        Err(QueueError::ZeroCapacity)
    ));
}

#[test]
fn concurrent_queue_has_no_lost_or_duplicate_values() {
    const PRODUCERS: usize = 3;
    const ITEMS_PER_PRODUCER: usize = 500;
    let queue = Arc::new(BoundedQueue::new(31).unwrap());
    let mut producers = Vec::new();

    for producer in 0..PRODUCERS {
        let queue = Arc::clone(&queue);
        producers.push(thread::spawn(move || {
            for offset in 0..ITEMS_PER_PRODUCER {
                let value = producer * ITEMS_PER_PRODUCER + offset;
                let mut pending = value;
                loop {
                    match queue.try_push(pending) {
                        Ok(()) => break,
                        Err(value) => {
                            pending = value;
                            thread::yield_now();
                        }
                    }
                }
            }
        }));
    }

    let expected = PRODUCERS * ITEMS_PER_PRODUCER;
    let mut observed = Vec::with_capacity(expected);
    while observed.len() < expected {
        if let Some(value) = queue.try_pop() {
            observed.push(value);
        } else {
            thread::yield_now();
        }
    }
    for producer in producers {
        producer.join().unwrap();
    }

    let unique: HashSet<_> = observed.iter().copied().collect();
    assert_eq!(observed.len(), expected);
    assert_eq!(unique.len(), expected);
}

#[test]
fn virtual_clock_applies_drift_and_rejects_stopped_time() {
    let clock = VirtualClock::new(500_000).unwrap();
    assert_eq!(clock.apply(2_000_000), 3_000_000);
    assert!(matches!(
        VirtualClock::new(-1_000_000),
        Err(ClockError::DriftWouldStopTime)
    ));
}

#[test]
fn scalar_quantization_round_trips_with_bounded_error() {
    let source = [-2.0, -0.5, 0.0, 0.75, 2.0];
    let quantized = QuantizedVector::quantize(&source);
    assert_eq!(quantized.values().first(), Some(&-127));
    assert_eq!(quantized.values().last(), Some(&127));

    let restored = quantized.dequantize();
    for (actual, expected) in restored.iter().zip(source) {
        assert!((actual - expected).abs() <= quantized.scale() / 2.0 + f32::EPSILON);
    }
    assert_eq!(squared_l2(&[0.0, 1.0], &[0.0]), None);
}

#[test]
fn greedy_layer_moves_to_the_nearest_reachable_node() {
    let vectors = vec![vec![0.0], vec![3.0], vec![2.0]];
    let neighbors = vec![vec![1], vec![0, 2], vec![1]];
    assert_eq!(greedy_layer(&vectors, &neighbors, &[2.1], 0, 8), Ok(2));
}
