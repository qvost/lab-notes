const PARTS_PER_MILLION: i64 = 1_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockError {
    DriftWouldStopTime,
}

/// Applies a fixed parts-per-million drift to nominal nanosecond timestamps.
#[derive(Debug, Clone, Copy)]
pub struct VirtualClock {
    drift_ppm: i32,
}

impl VirtualClock {
    pub fn new(drift_ppm: i32) -> Result<Self, ClockError> {
        if i64::from(drift_ppm) <= -PARTS_PER_MILLION {
            return Err(ClockError::DriftWouldStopTime);
        }
        Ok(Self { drift_ppm })
    }

    pub fn drift_ppm(self) -> i32 {
        self.drift_ppm
    }

    /// Converts nominal nanoseconds into virtual nanoseconds without float math.
    pub fn apply(self, nominal_ns: u64) -> u64 {
        let scale = (PARTS_PER_MILLION + i64::from(self.drift_ppm)) as u128;
        let adjusted = (u128::from(nominal_ns) * scale) / PARTS_PER_MILLION as u128;
        adjusted.min(u128::from(u64::MAX)) as u64
    }
}
