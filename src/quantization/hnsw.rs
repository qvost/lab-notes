use super::squared_l2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchError {
    EmptyGraph,
    InvalidEntry,
    DimensionMismatch,
    InvalidNeighbor,
}

/// Greedy descent within one HNSW layer, bounded by `max_steps` expansions.
///
/// `neighbors[node]` contains the node IDs reachable from `node` at this
/// layer. The function is intentionally a single-layer primitive; upper-layer
/// routing and the `ef` candidate set belong to the full index implementation.
pub fn greedy_layer(
    vectors: &[Vec<f32>],
    neighbors: &[Vec<usize>],
    query: &[f32],
    entry: usize,
    max_steps: usize,
) -> Result<usize, SearchError> {
    if vectors.is_empty() || vectors.len() != neighbors.len() {
        return Err(SearchError::EmptyGraph);
    }
    if entry >= vectors.len() {
        return Err(SearchError::InvalidEntry);
    }
    if vectors.iter().any(|vector| vector.len() != query.len()) {
        return Err(SearchError::DimensionMismatch);
    }
    if neighbors
        .iter()
        .flatten()
        .any(|&node| node >= vectors.len())
    {
        return Err(SearchError::InvalidNeighbor);
    }

    let mut current = entry;
    let mut current_distance =
        squared_l2(&vectors[current], query).ok_or(SearchError::DimensionMismatch)?;

    for _ in 0..max_steps {
        let mut next = current;
        let mut next_distance = current_distance;
        for &candidate in &neighbors[current] {
            let distance =
                squared_l2(&vectors[candidate], query).ok_or(SearchError::DimensionMismatch)?;
            if distance < next_distance || (distance == next_distance && candidate < next) {
                next = candidate;
                next_distance = distance;
            }
        }
        if next == current {
            break;
        }
        current = next;
        current_distance = next_distance;
    }

    Ok(current)
}
