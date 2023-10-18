//! Exponent Function.

use crate::{
    aliases_method_to_function::exp,
    float_type::float,
};


pub struct ExponentFunction {
    pub amplitude: float,
    pub tau: float,
    pub shift: float,
}

impl ExponentFunction {
    pub const AMPLITUDE_SCALE: float = 0.001;

    #[allow(dead_code)]
    #[deprecated = "explicitly use field names to not mix up them"]
    pub const fn new(amplitude: float, shift: float, tau: float) -> Self {
        Self { amplitude, shift, tau }
    }

    pub fn eval_at(&self, x: float) -> float {
        Self::eval_at_(self.amplitude, self.tau, self.shift, x)
    }

    /// This is to prevent memory "segmentation":
    /// [`ExponentFunction`] have 3 floats, but whole struct will be aligned to 4 floats (i guess?)
    /// + 1 float as arg => 5 floats in memory,
    /// whereas this method uses only 4 floats, as expected.
    ///
    /// Also this maybe improves cache locality a tiny bit (no extra ghost float in memory).
    ///
    /// Unfortunately, no performance gain was measured.
    pub fn eval_at_(amplitude: float, tau: float, shift: float, x: float) -> float {
        // "optimization" (i think it won't work): somehow precalc `1/tau`.
        let in_exp = -(x - shift) / tau;
        if in_exp <= 0. {
            Self::AMPLITUDE_SCALE * amplitude * exp(in_exp)
        } else {
            0.
        }
    }
}

