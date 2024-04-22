//! Deconvolution types

pub mod value_and_domain;

// functions:
pub mod exponents;
pub mod per_points;
#[allow(non_snake_case)]
pub mod sat_exp__dec_exp;
#[allow(non_snake_case)]
pub mod sat_exp__dec_exp_plus_const;
#[allow(non_snake_case)]
pub mod sat_exp__two_dec_exp;
#[allow(non_snake_case)]
pub mod sat_exp__two_dec_exp__constrained_consts;
#[allow(non_snake_case)]
pub mod sat_exp__two_dec_exp__separate_consts;
#[allow(non_snake_case)]
pub mod sat_exp__two_dec_exp_plus_const;
#[allow(non_snake_case)]
pub mod sigmoid__two_dec_exp__constrained_consts;
#[allow(non_snake_case)]
pub mod two__sat_exp__dec_exp;

use rand::rngs::ThreadRng;

use crate::types::{
    float::float,
    linalg::DVect,
    named_wrappers::{Deconvolved, DeconvolvedV, Params, ParamsG, ParamsV},
};

mod i_to_x;

use self::value_and_domain::ValueAndDomain;



pub(super) trait Function {
    /// Human readable name, used for output file.
    const NAME: &'static str;

    const FORMAT_FOR_DESMOS: &'static str;

    const FORMAT_FOR_ORIGIN: &'static str;

    fn to_plottable_function(&self, params: &Params, significant_digits: u8, format: &'static str) -> String;
}

pub(super) trait FunctionAutoImplFns {
    fn to_desmos_function(&self, params: &Params, significant_digits: u8) -> String;
    fn to_origin_function(&self, params: &Params, significant_digits: u8) -> String;
}

impl<T: Function> FunctionAutoImplFns for T {
    fn to_desmos_function(&self, params: &Params, significant_digits: u8) -> String {
        self.to_plottable_function(params, significant_digits, Self::FORMAT_FOR_DESMOS)
    }

    fn to_origin_function(&self, params: &Params, significant_digits: u8) -> String {
        self.to_plottable_function(params, significant_digits, Self::FORMAT_FOR_ORIGIN)
    }
}



// `T` is `float` or `ValueAndDomain`.
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

    // TODO:
    // /// From array of ValueAndDomain
    // fn from_array<const N: usize>(params: [T; N]) -> Self;

    /// Convert params to points
    ///
    /// `self` here needed just for `var.params_to_points()` instead of `Type::params_to_points()`,
    /// which prevents from mistakes (accidentaly using wrong type and getting wrong result).
    fn params_to_points(&self, params: &Params, points_len: usize, x_start_end: (float, float)) -> Deconvolved;
    fn params_to_points_v(&self, params: &ParamsV, points_len: usize, x_start_end: (float, float)) -> DeconvolvedV;
}


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
    fn get_randomized_with_rng(&self, initial_values_random_scale: float, rng: &mut ThreadRng) -> Params {
        ParamsG::<float>( // == Params
            self.to_vec().0
                .iter()
                .map(|vad| vad.get_randomized_with_rng(initial_values_random_scale, rng))
                .collect()
        )
    }

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

