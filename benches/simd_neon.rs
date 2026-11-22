//! ARM NEON 128-bit SIMD Distance Kernel
//! Reference: ARM Cortex-A78 / Apple Silicon vector pipeline
//! Authorship: Siddid Pandey <qvostwon@gmail.com>

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

/// 128-bit vectorized float32x4_t Euclidean distance for ARM64
#[cfg(target_arch = "aarch64")]
pub unsafe fn l2_neon(a: &[f32], b: &[f32]) -> f32 {
    let mut sum = vdupq_n_f32(0.0);
    let chunks = a.len() / 4;

    for i in 0..chunks {
        let va = vld1q_f32(a.as_ptr().add(i * 4));
        let vb = vld1q_f32(b.as_ptr().add(i * 4));
        let diff = vsubq_f32(va, vb);
        sum = vfmaq_f32(sum, diff, diff);
    }

    vaddvq_f32(sum)
}
