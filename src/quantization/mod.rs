//! Scalar quantization and graph-search prototypes.

mod hnsw;
mod scalar;

pub use hnsw::{greedy_layer, SearchError};
pub use scalar::{squared_l2, QuantizedVector};
