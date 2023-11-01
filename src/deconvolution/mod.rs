//! Deconvolution

pub mod deconvolution_data;
pub mod types;

pub(self) mod convolution;


use std::cmp::Ordering;

use toml::Value as TomlValue;

use crate::{
    config::Load,
    float_type::float,
};

use self::types::{
    DeconvolutionType,
    InitialValuesGeneric,
    InitialValuesVAD,
    exponents::{Exponents, InitialValues_Exponents},
    per_points::{InitialValues_PerPoint, PerPoint},
    sat_exp__dec_exp::{InitialValues_SatExp_DecExp, SatExp_DecExp},
    sat_exp__dec_exp_plus_const::{InitialValues_SatExp_DecExpPlusConst, SatExp_DecExpPlusConst},
    sat_exp__two_dec_exp::{InitialValues_SatExp_TwoDecExp, SatExp_TwoDecExp},
    sat_exp__two_dec_exp__separate_consts::{InitialValues_SatExp_TwoDecExp_SeparateConsts, SatExp_TwoDecExp_SeparateConsts},
    sat_exp__two_dec_exp_plus_const::{InitialValues_SatExp_TwoDecExpPlusConst, SatExp_TwoDecExpPlusConst},
    two__sat_exp__dec_exp::{InitialValues_Two_SatExp_DecExp, Two_SatExp_DecExp},
};


/// Deconvolution type and it's corresponding params.
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Deconvolution {
    PerPoint(PerPoint),
    Exponents(Exponents),
    SatExp_DecExp(SatExp_DecExp),
    SatExp_TwoDecExp(SatExp_TwoDecExp),
    Two_SatExp_DecExp(Two_SatExp_DecExp),
    SatExp_DecExpPlusConst(SatExp_DecExpPlusConst),
    SatExp_TwoDecExpPlusConst(SatExp_TwoDecExpPlusConst),
    SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts),
    // Fourier { unimplemented },
}

impl<'a> Deconvolution {
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
        }
    }

    pub fn get_initial_values_len(&self) -> usize {
        match self {
            Self::PerPoint(PerPoint { initial_vad, .. }) => initial_vad.len_dyn(),
            Self::Exponents(Exponents { initial_vads, .. }) => initial_vads.len_dyn(),
            Self::SatExp_DecExp(SatExp_DecExp { initial_vads, .. }) => initial_vads.len_dyn(),
            Self::SatExp_TwoDecExp(SatExp_TwoDecExp { initial_vads, .. }) => initial_vads.len_dyn(),
            Self::Two_SatExp_DecExp(Two_SatExp_DecExp { initial_vads, .. }) => initial_vads.len_dyn(),
            Self::SatExp_DecExpPlusConst(SatExp_DecExpPlusConst { initial_vads, .. }) => initial_vads.len_dyn(),
            Self::SatExp_TwoDecExpPlusConst(SatExp_TwoDecExpPlusConst { initial_vads, .. }) => initial_vads.len_dyn(),
            Self::SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts { initial_vads, .. }) => initial_vads.len_dyn(),
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
        }
    }

    pub fn calc_residue_function(&self, points_measured: &Vec<float>, points_convolved: &Vec<float>) -> float {
        match self {
            Deconvolution::PerPoint(PerPoint { diff_function_type, antispikes, .. }) => {
                diff_function_type.calc_diff_with_antispikes(points_measured, points_convolved, antispikes)
            }
            Deconvolution::Exponents(Exponents { diff_function_type, .. })
            | Deconvolution::SatExp_DecExp(SatExp_DecExp { diff_function_type, .. })
            | Deconvolution::SatExp_TwoDecExp(SatExp_TwoDecExp { diff_function_type, .. })
            | Deconvolution::Two_SatExp_DecExp(Two_SatExp_DecExp { diff_function_type, .. })
            | Deconvolution::SatExp_DecExpPlusConst(SatExp_DecExpPlusConst { diff_function_type, .. })
            | Deconvolution::SatExp_TwoDecExpPlusConst(SatExp_TwoDecExpPlusConst { diff_function_type, .. })
            | Deconvolution::SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts { diff_function_type, .. })
            => {
                diff_function_type.calc_diff(points_measured, points_convolved)
            }
        }
    }

    pub fn randomize(&mut self, initial_values_random_scale: float) {
        match self {
            Deconvolution::PerPoint(_) => panic!("there is no need to try different initial params"),
            Deconvolution::Exponents(Exponents { ref mut initial_vads, .. }) => initial_vads.randomize(initial_values_random_scale),
            Deconvolution::SatExp_DecExp(SatExp_DecExp { ref mut initial_vads, .. }) => initial_vads.randomize(initial_values_random_scale),
            Deconvolution::SatExp_TwoDecExp(SatExp_TwoDecExp { ref mut initial_vads, .. }) => initial_vads.randomize(initial_values_random_scale),
            Deconvolution::Two_SatExp_DecExp(Two_SatExp_DecExp { ref mut initial_vads, .. }) => initial_vads.randomize(initial_values_random_scale),
            Deconvolution::SatExp_DecExpPlusConst(SatExp_DecExpPlusConst { ref mut initial_vads, .. }) => initial_vads.randomize(initial_values_random_scale),
            Deconvolution::SatExp_TwoDecExpPlusConst(SatExp_TwoDecExpPlusConst { ref mut initial_vads, .. }) => initial_vads.randomize(initial_values_random_scale),
            Deconvolution::SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts { ref mut initial_vads, .. }) => initial_vads.randomize(initial_values_random_scale),
        }
    }

    // TODO: tests, check if they are work in desmos
    pub fn to_desmos_function(&self, params: &Vec<float>, significant_digits: u8) -> Result<String, &'static str> {
        let sd = significant_digits;
        Ok(format!("y=") + &match self {
            Deconvolution::PerPoint(_) => { return Err("not plottable") },
            Deconvolution::Exponents(self_) => self_.to_desmos_function(params, significant_digits),
            Deconvolution::SatExp_DecExp(self_) => self_.to_desmos_function(params, significant_digits),
            Deconvolution::SatExp_TwoDecExp(self_) => self_.to_desmos_function(params, significant_digits),
            Deconvolution::Two_SatExp_DecExp(self_) => self_.to_desmos_function(params, significant_digits),
            Deconvolution::SatExp_DecExpPlusConst(self_) => self_.to_desmos_function(params, significant_digits),
            Deconvolution::SatExp_TwoDecExpPlusConst(self_) => self_.to_desmos_function(params, significant_digits),
            Deconvolution::SatExp_TwoDecExp_SeparateConsts(self_) => self_.to_desmos_function(params, significant_digits),
        })
    }
}


impl Load for Deconvolution {
    const TOML_NAME: &'static str = "deconvolution_function";
    fn load_from_self_toml_value(toml_value: &TomlValue) -> Self {
        const DECONVOLUTION_FUNCTIONS_NAMES: [&'static str; 8] = [
            PerPoint::TOML_NAME,
            Exponents::TOML_NAME,
            SatExp_DecExp::TOML_NAME,
            SatExp_TwoDecExp::TOML_NAME,
            Two_SatExp_DecExp::TOML_NAME,
            SatExp_DecExpPlusConst::TOML_NAME,
            SatExp_TwoDecExpPlusConst::TOML_NAME,
            SatExp_TwoDecExp_SeparateConsts::TOML_NAME,
        ];
        let deconvolution_functions = DECONVOLUTION_FUNCTIONS_NAMES
            .map(|df_name| toml_value.get(df_name));
        let deconvolution_functions_number = deconvolution_functions
            .iter()
            .filter(|df| df.is_some())
            .count();
        match deconvolution_functions_number.cmp(&1) {
            Ordering::Less    => panic!("no known `deconvolution_function.<name>` found"),
            Ordering::Greater => panic!("too many `deconvolution_function.<name>` found"),
            Ordering::Equal   => {}
        }
        let deconvolution_function_index = deconvolution_functions
            .iter()
            .position(|df| df.is_some())
            .unwrap();
        let toml_value = deconvolution_functions[deconvolution_function_index].unwrap();
        // TODO(refactor): dont use numbers, bc they must be in sync with `DECONVOLUTION_FUNCTIONS_NAMES`
        match deconvolution_function_index {
            0 => Self::PerPoint(PerPoint::load_from_self_toml_value(toml_value)),
            1 => Self::Exponents(Exponents::load_from_self_toml_value(toml_value)),
            2 => Self::SatExp_DecExp(SatExp_DecExp::load_from_self_toml_value(toml_value)),
            3 => Self::SatExp_TwoDecExp(SatExp_TwoDecExp::load_from_self_toml_value(toml_value)),
            4 => Self::Two_SatExp_DecExp(Two_SatExp_DecExp::load_from_self_toml_value(toml_value)),
            5 => Self::SatExp_DecExpPlusConst(SatExp_DecExpPlusConst::load_from_self_toml_value(toml_value)),
            6 => Self::SatExp_TwoDecExpPlusConst(SatExp_TwoDecExpPlusConst::load_from_self_toml_value(toml_value)),
            7 => Self::SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts::load_from_self_toml_value(toml_value)),
            _ => unreachable!()
        }
    }
}

