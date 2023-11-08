//! Deconvolution

pub mod deconvolution_data;
pub mod types;

pub(self) mod convolution;


use std::cmp::Ordering;

use rand::{rngs::ThreadRng, thread_rng};
use toml::Value as TomlValue;

use crate::{
    float_type::float,
    load::Load,
    stacktrace::Stacktrace,
};

use self::types::{
    DeconvolutionType,
    InitialValuesGeneric,
    InitialValuesVAD,
    exponents::{Exponents, InitialValues_Exponents},
    per_points::PerPoint,
    sat_exp__dec_exp::{InitialValues_SatExp_DecExp, SatExp_DecExp},
    sat_exp__dec_exp_plus_const::{InitialValues_SatExp_DecExpPlusConst, SatExp_DecExpPlusConst},
    sat_exp__two_dec_exp::{InitialValues_SatExp_TwoDecExp, SatExp_TwoDecExp},
    sat_exp__two_dec_exp__constrained_consts::{InitialValues_SatExp_TwoDecExp_ConstrainedConsts, SatExp_TwoDecExp_ConstrainedConsts},
    sat_exp__two_dec_exp__separate_consts::{InitialValues_SatExp_TwoDecExp_SeparateConsts, SatExp_TwoDecExp_SeparateConsts},
    sat_exp__two_dec_exp_plus_const::{InitialValues_SatExp_TwoDecExpPlusConst, SatExp_TwoDecExpPlusConst},
    two__sat_exp__dec_exp::{InitialValues_Two_SatExp_DecExp, Two_SatExp_DecExp},
};


/// Deconvolution type and it's corresponding params.
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum DeconvolutionVariant {
    PerPoint(PerPoint),
    Exponents(Exponents),
    SatExp_DecExp(SatExp_DecExp),
    SatExp_TwoDecExp(SatExp_TwoDecExp),
    Two_SatExp_DecExp(Two_SatExp_DecExp),
    SatExp_DecExpPlusConst(SatExp_DecExpPlusConst),
    SatExp_TwoDecExpPlusConst(SatExp_TwoDecExpPlusConst),
    SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts),
    SatExp_TwoDecExp_ConstrainedConsts(SatExp_TwoDecExp_ConstrainedConsts),
    // Fourier { unimplemented },
}

impl DeconvolutionVariant {
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::PerPoint(_) => PerPoint::NAME,
            Self::Exponents(_) => Exponents::NAME,
            Self::SatExp_DecExp(_) => SatExp_DecExp::NAME,
            Self::SatExp_TwoDecExp(_) => SatExp_TwoDecExp::NAME,
            Self::Two_SatExp_DecExp(_) => Two_SatExp_DecExp::NAME,
            Self::SatExp_DecExpPlusConst(_) => SatExp_DecExpPlusConst::NAME,
            Self::SatExp_TwoDecExpPlusConst(_) => SatExp_TwoDecExpPlusConst::NAME,
            Self::SatExp_TwoDecExp_SeparateConsts(_) => SatExp_TwoDecExp_SeparateConsts::NAME,
            Self::SatExp_TwoDecExp_ConstrainedConsts(_) => SatExp_TwoDecExp_ConstrainedConsts::NAME,
        }
    }

    pub fn get_initial_values_len(&self) -> usize {
        match self {
            Self::PerPoint(PerPoint { initial_vad, .. }) => initial_vad.len(),
            Self::Exponents(Exponents { initial_vads, .. }) => initial_vads.len(),
            Self::SatExp_DecExp(SatExp_DecExp { initial_vads, .. }) => initial_vads.len(),
            Self::SatExp_TwoDecExp(SatExp_TwoDecExp { initial_vads, .. }) => initial_vads.len(),
            Self::Two_SatExp_DecExp(Two_SatExp_DecExp { initial_vads, .. }) => initial_vads.len(),
            Self::SatExp_DecExpPlusConst(SatExp_DecExpPlusConst { initial_vads, .. }) => initial_vads.len(),
            Self::SatExp_TwoDecExpPlusConst(SatExp_TwoDecExpPlusConst { initial_vads, .. }) => initial_vads.len(),
            Self::SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts { initial_vads, .. }) => initial_vads.len(),
            Self::SatExp_TwoDecExp_ConstrainedConsts(SatExp_TwoDecExp_ConstrainedConsts { initial_vads, .. }) => initial_vads.len(),
        }
    }

    pub fn get_initial_values(&self) -> Vec<float> {
        match self {
            Self::PerPoint(PerPoint { initial_vad, .. }) => vec![initial_vad.vad.value; initial_vad.len],
            Self::Exponents(Exponents { initial_vads, .. }) => InitialValues_Exponents::<float>::from(initial_vads.clone()).to_vec(),
            Self::SatExp_DecExp(SatExp_DecExp { initial_vads, .. }) => InitialValues_SatExp_DecExp::<float>::from(*initial_vads).to_vec(),
            Self::SatExp_TwoDecExp(SatExp_TwoDecExp { initial_vads, .. }) => InitialValues_SatExp_TwoDecExp::<float>::from(*initial_vads).to_vec(),
            Self::Two_SatExp_DecExp(Two_SatExp_DecExp { initial_vads, .. }) => InitialValues_Two_SatExp_DecExp::<float>::from(*initial_vads).to_vec(),
            Self::SatExp_DecExpPlusConst(SatExp_DecExpPlusConst { initial_vads, .. }) => InitialValues_SatExp_DecExpPlusConst::<float>::from(*initial_vads).to_vec(),
            Self::SatExp_TwoDecExpPlusConst(SatExp_TwoDecExpPlusConst { initial_vads, .. }) => InitialValues_SatExp_TwoDecExpPlusConst::<float>::from(*initial_vads).to_vec(),
            Self::SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts { initial_vads, .. }) => InitialValues_SatExp_TwoDecExp_SeparateConsts::<float>::from(*initial_vads).to_vec(),
            Self::SatExp_TwoDecExp_ConstrainedConsts(SatExp_TwoDecExp_ConstrainedConsts { initial_vads, .. }) => InitialValues_SatExp_TwoDecExp_ConstrainedConsts::<float>::from(*initial_vads).to_vec(),
        }
    }

    pub fn get_initial_values_randomized(&self, initial_values_random_scale: float) -> Vec<float> {
        let mut rng = thread_rng();
        self.get_initial_values_randomized_with_rng(initial_values_random_scale, &mut rng)
    }

    pub fn get_initial_values_randomized_with_rng(&self, initial_values_random_scale: float, rng: &mut ThreadRng) -> Vec<float> {
        match self {
            Self::PerPoint(PerPoint { initial_vad, .. }) => initial_vad.get_randomized_with_rng(initial_values_random_scale, rng),
            Self::Exponents(Exponents { initial_vads, .. }) => initial_vads.get_randomized_with_rng(initial_values_random_scale, rng),
            Self::SatExp_DecExp(SatExp_DecExp { initial_vads, .. }) => initial_vads.get_randomized_with_rng(initial_values_random_scale, rng),
            Self::SatExp_TwoDecExp(SatExp_TwoDecExp { initial_vads, .. }) => initial_vads.get_randomized_with_rng(initial_values_random_scale, rng),
            Self::Two_SatExp_DecExp(Two_SatExp_DecExp { initial_vads, .. }) => initial_vads.get_randomized_with_rng(initial_values_random_scale, rng),
            Self::SatExp_DecExpPlusConst(SatExp_DecExpPlusConst { initial_vads, .. }) => initial_vads.get_randomized_with_rng(initial_values_random_scale, rng),
            Self::SatExp_TwoDecExpPlusConst(SatExp_TwoDecExpPlusConst { initial_vads, .. }) => initial_vads.get_randomized_with_rng(initial_values_random_scale, rng),
            Self::SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts { initial_vads, .. }) => initial_vads.get_randomized_with_rng(initial_values_random_scale, rng),
            Self::SatExp_TwoDecExp_ConstrainedConsts(SatExp_TwoDecExp_ConstrainedConsts { initial_vads, .. }) => initial_vads.get_randomized_with_rng(initial_values_random_scale, rng),
        }
    }

    pub fn is_params_ok(&self, params: &Vec<float>) -> bool {
        match self {
            Self::PerPoint(PerPoint { initial_vad, .. }) => initial_vad.is_params_ok(params),
            Self::Exponents(Exponents { initial_vads, .. }) => initial_vads.is_params_ok(params),
            Self::SatExp_DecExp(SatExp_DecExp { initial_vads, .. }) => initial_vads.is_params_ok(params),
            Self::SatExp_TwoDecExp(SatExp_TwoDecExp { initial_vads, .. }) => initial_vads.is_params_ok(params),
            Self::Two_SatExp_DecExp(Two_SatExp_DecExp { initial_vads, .. }) => initial_vads.is_params_ok(params),
            Self::SatExp_DecExpPlusConst(SatExp_DecExpPlusConst { initial_vads, .. }) => initial_vads.is_params_ok(params),
            Self::SatExp_TwoDecExpPlusConst(SatExp_TwoDecExpPlusConst { initial_vads, .. }) => initial_vads.is_params_ok(params),
            Self::SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts { initial_vads, .. }) => initial_vads.is_params_ok(params),
            Self::SatExp_TwoDecExp_ConstrainedConsts(SatExp_TwoDecExp_ConstrainedConsts { initial_vads, .. }) => initial_vads.is_params_ok(params),
        }
    }

    pub fn params_to_points(
        &self,
        params: &Vec<float>,
        points_len: usize,
        x_start_end: (float, float),
    ) -> Vec<float> {
        assert!(points_len > 1);
        assert!(x_start_end.0 < x_start_end.1);

        match self {
            Self::PerPoint(PerPoint { initial_vad, .. }) => initial_vad.params_to_points(params, points_len, x_start_end),
            Self::Exponents(Exponents { initial_vads, .. }) => initial_vads.params_to_points(params, points_len, x_start_end),
            Self::SatExp_DecExp(SatExp_DecExp { initial_vads, .. }) => initial_vads.params_to_points(params, points_len, x_start_end),
            Self::SatExp_TwoDecExp(SatExp_TwoDecExp { initial_vads, .. }) => initial_vads.params_to_points(params, points_len, x_start_end),
            Self::Two_SatExp_DecExp(Two_SatExp_DecExp { initial_vads, .. }) => initial_vads.params_to_points(params, points_len, x_start_end),
            Self::SatExp_DecExpPlusConst(SatExp_DecExpPlusConst { initial_vads, .. }) => initial_vads.params_to_points(params, points_len, x_start_end),
            Self::SatExp_TwoDecExpPlusConst(SatExp_TwoDecExpPlusConst { initial_vads, .. }) => initial_vads.params_to_points(params, points_len, x_start_end),
            Self::SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts { initial_vads, .. }) => initial_vads.params_to_points(params, points_len, x_start_end),
            Self::SatExp_TwoDecExp_ConstrainedConsts(SatExp_TwoDecExp_ConstrainedConsts{ initial_vads, .. }) => initial_vads.params_to_points(params, points_len, x_start_end),
        }
    }

    pub fn calc_residue_function(&self, points_measured: &Vec<float>, points_convolved: &Vec<float>) -> float {
        match self {
            Self::PerPoint(PerPoint { diff_function_type, antispikes, .. }) => {
                diff_function_type.calc_diff_with_antispikes(points_measured, points_convolved, antispikes)
            }
            Self::Exponents(Exponents { diff_function_type, .. })
            | Self::SatExp_DecExp(SatExp_DecExp { diff_function_type, .. })
            | Self::SatExp_TwoDecExp(SatExp_TwoDecExp { diff_function_type, .. })
            | Self::Two_SatExp_DecExp(Two_SatExp_DecExp { diff_function_type, .. })
            | Self::SatExp_DecExpPlusConst(SatExp_DecExpPlusConst { diff_function_type, .. })
            | Self::SatExp_TwoDecExpPlusConst(SatExp_TwoDecExpPlusConst { diff_function_type, .. })
            | Self::SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts { diff_function_type, .. })
            | Self::SatExp_TwoDecExp_ConstrainedConsts(SatExp_TwoDecExp_ConstrainedConsts { diff_function_type, .. })
            => {
                diff_function_type.calc_diff(points_measured, points_convolved)
            }
        }
    }

    // TODO: tests, check if they work in desmos
    pub fn to_desmos_function(&self, params: &Vec<float>, significant_digits: u8) -> Result<String, &'static str> {
        let sd = significant_digits;
        Ok(format!("y=") + &match self {
            Self::PerPoint(_) => { return Err("not plottable") },
            Self::Exponents(self_) => self_.to_desmos_function(params, sd),
            Self::SatExp_DecExp(self_) => self_.to_desmos_function(params, sd),
            Self::SatExp_TwoDecExp(self_) => self_.to_desmos_function(params, sd),
            Self::Two_SatExp_DecExp(self_) => self_.to_desmos_function(params, sd),
            Self::SatExp_DecExpPlusConst(self_) => self_.to_desmos_function(params, sd),
            Self::SatExp_TwoDecExpPlusConst(self_) => self_.to_desmos_function(params, sd),
            Self::SatExp_TwoDecExp_SeparateConsts(self_) => self_.to_desmos_function(params, sd),
            Self::SatExp_TwoDecExp_ConstrainedConsts(self_) => self_.to_desmos_function(params, sd),
        })
    }

    // TODO: tests, check if they work in origin
    pub fn to_origin_function(&self, params: &Vec<float>, significant_digits: u8) -> Result<String, &'static str> {
        let sd = significant_digits;
        Ok(match self {
            Self::PerPoint(_) => { return Err("not plottable") },
            Self::Exponents(self_) => self_.to_origin_function(params, sd),
            Self::SatExp_DecExp(self_) => self_.to_origin_function(params, sd),
            Self::SatExp_TwoDecExp(self_) => self_.to_origin_function(params, sd),
            Self::Two_SatExp_DecExp(self_) => self_.to_origin_function(params, sd),
            Self::SatExp_DecExpPlusConst(self_) => self_.to_origin_function(params, sd),
            Self::SatExp_TwoDecExpPlusConst(self_) => self_.to_origin_function(params, sd),
            Self::SatExp_TwoDecExp_SeparateConsts(self_) => self_.to_origin_function(params, sd),
            Self::SatExp_TwoDecExp_ConstrainedConsts(self_) => self_.to_origin_function(params, sd),
        })
    }
}


impl Load for DeconvolutionVariant {
    const TOML_NAME: &'static str = "deconvolution_function";
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        const DECONVOLUTION_FUNCTIONS_NAMES: [&'static str; 9] = [
            PerPoint::TOML_NAME,
            Exponents::TOML_NAME,
            SatExp_DecExp::TOML_NAME,
            SatExp_TwoDecExp::TOML_NAME,
            Two_SatExp_DecExp::TOML_NAME,
            SatExp_DecExpPlusConst::TOML_NAME,
            SatExp_TwoDecExpPlusConst::TOML_NAME,
            SatExp_TwoDecExp_SeparateConsts::TOML_NAME,
            SatExp_TwoDecExp_ConstrainedConsts::TOML_NAME,
        ];
        let deconvolution_functions = DECONVOLUTION_FUNCTIONS_NAMES
            .map(|df_name| toml_value.get(df_name));
        let deconvolution_functions_number = deconvolution_functions
            .iter()
            .flatten() // flatten on Iter<Option/Result> gives only Some/Ok variants and unwraps them
            .count();
        match deconvolution_functions_number.cmp(&1) {
            // TODO: maybe somehow get first entry from table and use `panic_unknown_type` with value
            Ordering::Less    => stacktrace.panic_unknown_type_without_value(DECONVOLUTION_FUNCTIONS_NAMES),
            Ordering::Greater => stacktrace.panic_more_than_one_found(
                deconvolution_functions
                    .iter()
                    .zip(DECONVOLUTION_FUNCTIONS_NAMES)
                    .map(|(odf, dfn)| odf.map(|_| dfn))
                    .flatten()
                    .collect::<Vec<_>>()
            ),
            Ordering::Equal => {}
        }
        let deconvolution_function_index = deconvolution_functions
            .iter()
            .position(|df| df.is_some())
            .unwrap();
        let toml_value = deconvolution_functions[deconvolution_function_index].unwrap();
        // TODO(refactor): dont use numbers, bc they must be kept in sync with `DECONVOLUTION_FUNCTIONS_NAMES`
        // - maybe create vec of [PerPoint, Exponents, ...] and try load by them?
        match deconvolution_function_index {
            0 => Self::PerPoint(PerPoint::load_from_self_handle_stacktrace(toml_value, stacktrace)),
            1 => Self::Exponents(Exponents::load_from_self_handle_stacktrace(toml_value, stacktrace)),
            2 => Self::SatExp_DecExp(SatExp_DecExp::load_from_self_handle_stacktrace(toml_value, stacktrace)),
            3 => Self::SatExp_TwoDecExp(SatExp_TwoDecExp::load_from_self_handle_stacktrace(toml_value, stacktrace)),
            4 => Self::Two_SatExp_DecExp(Two_SatExp_DecExp::load_from_self_handle_stacktrace(toml_value, stacktrace)),
            5 => Self::SatExp_DecExpPlusConst(SatExp_DecExpPlusConst::load_from_self_handle_stacktrace(toml_value, stacktrace)),
            6 => Self::SatExp_TwoDecExpPlusConst(SatExp_TwoDecExpPlusConst::load_from_self_handle_stacktrace(toml_value, stacktrace)),
            7 => Self::SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts::load_from_self_handle_stacktrace(toml_value, stacktrace)),
            8 => Self::SatExp_TwoDecExp_ConstrainedConsts(SatExp_TwoDecExp_ConstrainedConsts::load_from_self_handle_stacktrace(toml_value, stacktrace)),
            _ => unreachable!()
        }
    }
}

