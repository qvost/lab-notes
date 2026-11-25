/// A symmetric signed-int8 vector and its dequantization scale.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantizedVector {
    values: Vec<i8>,
    scale: f32,
}

impl QuantizedVector {
    pub fn quantize(values: &[f32]) -> Self {
        let max_abs = values
            .iter()
            .fold(0.0_f32, |max, value| max.max(value.abs()));
        let scale = if max_abs == 0.0 { 1.0 } else { max_abs / 127.0 };
        let values = values
            .iter()
            .map(|value| (value / scale).round().clamp(-127.0, 127.0) as i8)
            .collect();
        Self { values, scale }
    }

    pub fn values(&self) -> &[i8] {
        &self.values
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn dequantize(&self) -> Vec<f32> {
        self.values
            .iter()
            .map(|value| f32::from(*value) * self.scale)
            .collect()
    }
}

/// Returns squared Euclidean distance, or `None` for different dimensions.
pub fn squared_l2(left: &[f32], right: &[f32]) -> Option<f32> {
    if left.len() != right.len() {
        return None;
    }
    Some(
        left.iter()
            .zip(right)
            .map(|(a, b)| {
                let delta = a - b;
                delta * delta
            })
            .sum(),
    )
}
