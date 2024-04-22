//! Inital values.

use rand::rngs::ThreadRng;

use crate::types::{
    float::float,
    linalg::DVect,
    named_wrappers::{DeconvolvedV, ParamsG, ParamsV},
};

use super::types::value_and_domain::ValueAndDomain;


// Here `T` is `float` or `ValueAndDomain`.
pub trait InitialValuesGeneric<T> {
    /// Number of initial values (don't depend on `self` => static)
    const LEN: usize;

    /// Number of initial values (depends on `self` => dynamic)
    fn len(&self) -> usize {
        Self::LEN
    }

    /// From vector
    fn from_vec(params: &ParamsG<T>) -> Self;
    // fn from_vec_v(params: &DVect) -> Self;

    /// To vector
    fn to_vec(&self) -> ParamsG<T>;

    /// Convert params to points
    ///
    /// `self` here needed just for `var.params_to_points()` instead of `Type::params_to_points()`,
    /// which prevents from mistakes (accidentaly using wrong type and getting wrong result).
    fn params_to_points_v(&self, params: &ParamsV, points_len: usize, x_start_end: (float, float)) -> DeconvolvedV;
}


// TODO(refactor): remake into auto fns, like in [`crate::load::LoadAutoImplFns`].
pub trait InitialValuesVAD
where Self: Sized + InitialValuesGeneric<ValueAndDomain>
{
    /// Check if given params are satisfying conditions
    fn is_params_ok_v(&self, params: &ParamsV) -> bool {
        self.to_vec().0.iter()
            .zip(&params.0)
            .all(|(vad, &value)| vad.contains(value))
    }

    /// Get randomized initial values with given `ThreadRng`
    fn get_randomized_with_rng_v(&self, initial_values_random_scale: float, rng: &mut ThreadRng) -> ParamsV {
        let v = self.to_vec();
        ParamsV(
            DVect::from_iterator(
                v.0.len(),
                v.0.iter().map(|vad| vad.get_randomized_with_rng(initial_values_random_scale, rng))
            )
        )
    }
}

