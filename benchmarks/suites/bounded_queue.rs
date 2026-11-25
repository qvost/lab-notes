use std::hint::black_box;
use std::time::Instant;

use strata_core::core::BoundedQueue;

fn main() {
    const ITERATIONS: u64 = 1_000_000;
    let queue = BoundedQueue::new(1024).expect("non-zero capacity");
    let start = Instant::now();

    for value in 0..ITERATIONS {
        queue.try_push(black_box(value)).expect("queue has room");
        black_box(queue.try_pop().expect("just inserted"));
    }

    let elapsed = start.elapsed();
    let pairs_per_second = ITERATIONS as f64 / elapsed.as_secs_f64();
    println!(
        "iterations={ITERATIONS} elapsed_ns={} pairs_per_second={pairs_per_second:.0}",
        elapsed.as_nanos()
    );
}
