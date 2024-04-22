//! Deconvolution types

pub mod initial_values;
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


use crate::types::named_wrappers::Params;

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

