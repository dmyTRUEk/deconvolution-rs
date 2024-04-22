//! Deconvolution

pub mod deconvolution_data;
pub mod types;

pub(self) mod convolution;


use std::cmp::Ordering;

use rand::{rngs::ThreadRng, thread_rng};
use toml::Value as TomlValue;

use crate::{
    load::{LoadAutoImplFns, Load},
    stacktrace::Stacktrace,
    types::{
        float::float,
        named_wrappers::{ConvolvedV, DeconvolvedV, MeasuredV, Params, ParamsG, ParamsV},
    },
};

use self::types::{
    Function,
    FunctionAutoImplFns,
    exponents::{Exponents, InitialValues_Exponents},
    initial_values::{InitialValuesGeneric, InitialValuesVAD},
    per_points::PerPoint,
    sat_exp__dec_exp::{InitialValues_SatExp_DecExp, SatExp_DecExp},
    sat_exp__dec_exp_plus_const::{InitialValues_SatExp_DecExpPlusConst, SatExp_DecExpPlusConst},
    sat_exp__two_dec_exp::{InitialValues_SatExp_TwoDecExp, SatExp_TwoDecExp},
    sat_exp__two_dec_exp__constrained_consts::{InitialValues_SatExp_TwoDecExp_ConstrainedConsts, SatExp_TwoDecExp_ConstrainedConsts},
    sat_exp__two_dec_exp__separate_consts::{InitialValues_SatExp_TwoDecExp_SeparateConsts, SatExp_TwoDecExp_SeparateConsts},
    sat_exp__two_dec_exp_plus_const::{InitialValues_SatExp_TwoDecExpPlusConst, SatExp_TwoDecExpPlusConst},
    sigmoid__two_dec_exp__constrained_consts::{InitialValues_Sigmoid_TwoDecExp_ConstrainedConsts, Sigmoid_TwoDecExp_ConstrainedConsts},
    two__sat_exp__dec_exp::{InitialValues_Two_SatExp_DecExp, Two_SatExp_DecExp},
};


/// Deconvolution type and it's corresponding params.
#[allow(non_camel_case_types)]
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
    Sigmoid_TwoDecExp_ConstrainedConsts(Sigmoid_TwoDecExp_ConstrainedConsts),
    // Fourier { unimplemented },
}

impl DeconvolutionVariant {
    // TODO(refactor): rewrite all this boilerplate code using trait and impl for each

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
            Self::Sigmoid_TwoDecExp_ConstrainedConsts(_) => Sigmoid_TwoDecExp_ConstrainedConsts::NAME,
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
            Self::Sigmoid_TwoDecExp_ConstrainedConsts(Sigmoid_TwoDecExp_ConstrainedConsts { initial_vads, .. }) => initial_vads.len(),
        }
    }

    pub fn get_initial_values(&self) -> Params {
        match self {
            Self::PerPoint(PerPoint { initial_vad, .. }) => ParamsG::<float>(vec![initial_vad.vad.value; initial_vad.len]),
            Self::Exponents(Exponents { initial_vads, .. }) => InitialValues_Exponents::<float>::from(initial_vads.clone()).to_vec(),
            Self::SatExp_DecExp(SatExp_DecExp { initial_vads, .. }) => InitialValues_SatExp_DecExp::<float>::from(*initial_vads).to_vec(),
            Self::SatExp_TwoDecExp(SatExp_TwoDecExp { initial_vads, .. }) => InitialValues_SatExp_TwoDecExp::<float>::from(*initial_vads).to_vec(),
            Self::Two_SatExp_DecExp(Two_SatExp_DecExp { initial_vads, .. }) => InitialValues_Two_SatExp_DecExp::<float>::from(*initial_vads).to_vec(),
            Self::SatExp_DecExpPlusConst(SatExp_DecExpPlusConst { initial_vads, .. }) => InitialValues_SatExp_DecExpPlusConst::<float>::from(*initial_vads).to_vec(),
            Self::SatExp_TwoDecExpPlusConst(SatExp_TwoDecExpPlusConst { initial_vads, .. }) => InitialValues_SatExp_TwoDecExpPlusConst::<float>::from(*initial_vads).to_vec(),
            Self::SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts { initial_vads, .. }) => InitialValues_SatExp_TwoDecExp_SeparateConsts::<float>::from(*initial_vads).to_vec(),
            Self::SatExp_TwoDecExp_ConstrainedConsts(SatExp_TwoDecExp_ConstrainedConsts { initial_vads, .. }) => InitialValues_SatExp_TwoDecExp_ConstrainedConsts::<float>::from(*initial_vads).to_vec(),
            Self::Sigmoid_TwoDecExp_ConstrainedConsts(Sigmoid_TwoDecExp_ConstrainedConsts { initial_vads, .. }) => InitialValues_Sigmoid_TwoDecExp_ConstrainedConsts::<float>::from(*initial_vads).to_vec(),
        }
    }

    pub fn get_initial_values_randomized_v(&self, initial_values_random_scale: float) -> ParamsV {
        let mut rng = thread_rng();
        self.get_initial_values_randomized_with_rng_v(initial_values_random_scale, &mut rng)
    }

    pub fn get_initial_values_randomized_with_rng_v(&self, initial_values_random_scale: float, rng: &mut ThreadRng) -> ParamsV {
        match self {
            Self::PerPoint(PerPoint { initial_vad, .. }) => initial_vad.get_randomized_with_rng_v(initial_values_random_scale, rng),
            Self::Exponents(Exponents { initial_vads, .. }) => initial_vads.get_randomized_with_rng_v(initial_values_random_scale, rng),
            Self::SatExp_DecExp(SatExp_DecExp { initial_vads, .. }) => initial_vads.get_randomized_with_rng_v(initial_values_random_scale, rng),
            Self::SatExp_TwoDecExp(SatExp_TwoDecExp { initial_vads, .. }) => initial_vads.get_randomized_with_rng_v(initial_values_random_scale, rng),
            Self::Two_SatExp_DecExp(Two_SatExp_DecExp { initial_vads, .. }) => initial_vads.get_randomized_with_rng_v(initial_values_random_scale, rng),
            Self::SatExp_DecExpPlusConst(SatExp_DecExpPlusConst { initial_vads, .. }) => initial_vads.get_randomized_with_rng_v(initial_values_random_scale, rng),
            Self::SatExp_TwoDecExpPlusConst(SatExp_TwoDecExpPlusConst { initial_vads, .. }) => initial_vads.get_randomized_with_rng_v(initial_values_random_scale, rng),
            Self::SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts { initial_vads, .. }) => initial_vads.get_randomized_with_rng_v(initial_values_random_scale, rng),
            Self::SatExp_TwoDecExp_ConstrainedConsts(SatExp_TwoDecExp_ConstrainedConsts { initial_vads, .. }) => initial_vads.get_randomized_with_rng_v(initial_values_random_scale, rng),
            Self::Sigmoid_TwoDecExp_ConstrainedConsts(Sigmoid_TwoDecExp_ConstrainedConsts { initial_vads, .. }) => initial_vads.get_randomized_with_rng_v(initial_values_random_scale, rng),
        }
    }

    pub fn is_params_ok_v(&self, params: &ParamsV) -> bool {
        match self {
            Self::PerPoint(PerPoint { initial_vad, .. }) => initial_vad.is_params_ok_v(params),
            Self::Exponents(Exponents { initial_vads, .. }) => initial_vads.is_params_ok_v(params),
            Self::SatExp_DecExp(SatExp_DecExp { initial_vads, .. }) => initial_vads.is_params_ok_v(params),
            Self::SatExp_TwoDecExp(SatExp_TwoDecExp { initial_vads, .. }) => initial_vads.is_params_ok_v(params),
            Self::Two_SatExp_DecExp(Two_SatExp_DecExp { initial_vads, .. }) => initial_vads.is_params_ok_v(params),
            Self::SatExp_DecExpPlusConst(SatExp_DecExpPlusConst { initial_vads, .. }) => initial_vads.is_params_ok_v(params),
            Self::SatExp_TwoDecExpPlusConst(SatExp_TwoDecExpPlusConst { initial_vads, .. }) => initial_vads.is_params_ok_v(params),
            Self::SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts { initial_vads, .. }) => initial_vads.is_params_ok_v(params),
            Self::SatExp_TwoDecExp_ConstrainedConsts(SatExp_TwoDecExp_ConstrainedConsts { initial_vads, .. }) => initial_vads.is_params_ok_v(params),
            Self::Sigmoid_TwoDecExp_ConstrainedConsts(Sigmoid_TwoDecExp_ConstrainedConsts { initial_vads, .. }) => initial_vads.is_params_ok_v(params),
        }
    }

    pub fn params_to_points_v(
        &self,
        params: &ParamsV,
        points_len: usize,
        x_start_end: (float, float),
    ) -> DeconvolvedV {
        assert!(points_len > 1);
        assert!(x_start_end.0 < x_start_end.1);
        match self {
            Self::PerPoint(PerPoint { initial_vad, .. }) => initial_vad.params_to_points_v(params, points_len, x_start_end),
            Self::Exponents(Exponents { initial_vads, .. }) => initial_vads.params_to_points_v(params, points_len, x_start_end),
            Self::SatExp_DecExp(SatExp_DecExp { initial_vads, .. }) => initial_vads.params_to_points_v(params, points_len, x_start_end),
            Self::SatExp_TwoDecExp(SatExp_TwoDecExp { initial_vads, .. }) => initial_vads.params_to_points_v(params, points_len, x_start_end),
            Self::Two_SatExp_DecExp(Two_SatExp_DecExp { initial_vads, .. }) => initial_vads.params_to_points_v(params, points_len, x_start_end),
            Self::SatExp_DecExpPlusConst(SatExp_DecExpPlusConst { initial_vads, .. }) => initial_vads.params_to_points_v(params, points_len, x_start_end),
            Self::SatExp_TwoDecExpPlusConst(SatExp_TwoDecExpPlusConst { initial_vads, .. }) => initial_vads.params_to_points_v(params, points_len, x_start_end),
            Self::SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts { initial_vads, .. }) => initial_vads.params_to_points_v(params, points_len, x_start_end),
            Self::SatExp_TwoDecExp_ConstrainedConsts(SatExp_TwoDecExp_ConstrainedConsts { initial_vads, .. }) => initial_vads.params_to_points_v(params, points_len, x_start_end),
            Self::Sigmoid_TwoDecExp_ConstrainedConsts(Sigmoid_TwoDecExp_ConstrainedConsts { initial_vads, .. }) => initial_vads.params_to_points_v(params, points_len, x_start_end),
        }
    }

    pub fn calc_residue_function_v(&self, points_measured: &MeasuredV, points_convolved: ConvolvedV) -> float {
        match self {
            Self::PerPoint(PerPoint { diff_function_type, antispikes, .. }) => {
                diff_function_type.calc_diff_with_antispikes_v(&points_measured.0, &points_convolved.0, antispikes)
            }
            Self::Exponents(Exponents { diff_function_type, .. })
            | Self::SatExp_DecExp(SatExp_DecExp { diff_function_type, .. })
            | Self::SatExp_TwoDecExp(SatExp_TwoDecExp { diff_function_type, .. })
            | Self::Two_SatExp_DecExp(Two_SatExp_DecExp { diff_function_type, .. })
            | Self::SatExp_DecExpPlusConst(SatExp_DecExpPlusConst { diff_function_type, .. })
            | Self::SatExp_TwoDecExpPlusConst(SatExp_TwoDecExpPlusConst { diff_function_type, .. })
            | Self::SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts { diff_function_type, .. })
            | Self::SatExp_TwoDecExp_ConstrainedConsts(SatExp_TwoDecExp_ConstrainedConsts { diff_function_type, .. })
            | Self::Sigmoid_TwoDecExp_ConstrainedConsts(Sigmoid_TwoDecExp_ConstrainedConsts { diff_function_type, .. })
            => {
                diff_function_type.calc_diff_v(&points_measured.0, &points_convolved.0)
            }
        }
    }

    // TODO: tests, check if they work in desmos
    pub fn to_desmos_function(&self, params: &Params, significant_digits: u8) -> Result<String, &'static str> {
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
            Self::Sigmoid_TwoDecExp_ConstrainedConsts(self_) => self_.to_desmos_function(params, sd),
        })
    }

    // TODO: tests, check if they work in origin
    pub fn to_origin_function(&self, params: &Params, significant_digits: u8) -> Result<String, &'static str> {
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
            Self::Sigmoid_TwoDecExp_ConstrainedConsts(self_) => self_.to_origin_function(params, sd),
        })
    }
}


impl Load for DeconvolutionVariant {
    const TOML_NAME: &'static str = "deconvolution_function";
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        const DECONVOLUTION_FUNCTIONS_NAMES: [&'static str; 10] = [
            PerPoint::TOML_NAME,
            Exponents::TOML_NAME,
            SatExp_DecExp::TOML_NAME,
            SatExp_TwoDecExp::TOML_NAME,
            Two_SatExp_DecExp::TOML_NAME,
            SatExp_DecExpPlusConst::TOML_NAME,
            SatExp_TwoDecExpPlusConst::TOML_NAME,
            SatExp_TwoDecExp_SeparateConsts::TOML_NAME,
            SatExp_TwoDecExp_ConstrainedConsts::TOML_NAME,
            Sigmoid_TwoDecExp_ConstrainedConsts::TOML_NAME,
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
            9 => Self::Sigmoid_TwoDecExp_ConstrainedConsts(Sigmoid_TwoDecExp_ConstrainedConsts::load_from_self_handle_stacktrace(toml_value, stacktrace)),
            _ => unreachable!()
        }
    }
}





#[cfg(test)]
mod deconvolve {
    mod per_point {
        use crate::{
            DeconvolutionData,
            Spectrum,
            deconvolution::{
                DeconvolutionVariant,
                types::{
                    per_points::{InitialValues_PerPoint, PerPoint},
                    value_and_domain::ValueAndDomain,
                },
            },
            diff_function::DiffFunction,
            fit_algorithms::{FitAlgorithmVariant, pattern_search::PatternSearch},
            float,
        };
        use super::super::deconvolution_data::DeconvolutionResultOrError;
        const FIT_ALGORITHM: FitAlgorithmVariant = FitAlgorithmVariant::PatternSearch(PatternSearch {
            fit_algorithm_min_step: 1e-4,
            fit_residue_evals_max: 1_000_000,
            initial_step: 1.,
            alpha: 1.1,
            beta: None,
        });
        fn deconvolve(points_instrument: Vec<float>, points_spectrum: Vec<float>) -> DeconvolutionResultOrError {
            let instrument: Spectrum = Spectrum {
                points: points_instrument,
                step: 1.,
                x_start: 0.,
            };
            let measured: Spectrum = Spectrum {
                points: points_spectrum.clone(),
                step: 1.,
                x_start: 0.,
            };
            let deconvolution_data: DeconvolutionData = DeconvolutionData {
                instrument,
                measured,
                deconvolution: DeconvolutionVariant::PerPoint(PerPoint {
                    diff_function_type: DiffFunction::DySqr,
                    antispikes: None,
                    initial_vad: InitialValues_PerPoint::new(points_spectrum.len(), ValueAndDomain::free(0.)),
                }),
            };
            deconvolution_data.deconvolve(&FIT_ALGORITHM, None)
        }
        mod instrument_is_identity {
            use super::*;
            const POINTS_INSTRUMENT: [float; 1] = [1.];
            mod original_spectrum_is_delta_21 {
                use super::*;
                const EPSILON: float = 1e-4;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 10], vec![1.], vec![0.; 10]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![1.], vec![0.; 20]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 20], vec![1.]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
            mod original_spectrum_is_two_deltas_20 {
                use super::*;
                const EPSILON: float = 1e-4;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 6], vec![1.], vec![0.; 6], vec![1.], vec![0.; 6]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![1.], vec![0.; 6], vec![1.], vec![0.; 12]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 12], vec![1.], vec![0.; 6], vec![1.]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
        }
        mod instrument_is_delta3 {
            use super::*;
            const POINTS_INSTRUMENT: [float; 3] = [0., 1., 0.];
            mod original_spectrum_is_delta_21 {
                use super::*;
                const EPSILON: float = 1e-4;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 10], vec![1.], vec![0.; 10]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![1.], vec![0.; 20]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 20], vec![1.]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
            mod original_spectrum_is_two_deltas_20 {
                use super::*;
                const EPSILON: float = 1e-4;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 6], vec![1.], vec![0.; 6], vec![1.], vec![0.; 6]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![1.], vec![0.; 6], vec![1.], vec![0.; 12]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 12], vec![1.], vec![0.; 6], vec![1.]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
        }
        mod instrument_is_delta7 {
            use super::*;
            const POINTS_INSTRUMENT: [float; 7] = [0., 0., 0., 1., 0., 0., 0.];
            mod original_spectrum_is_delta_21 {
                use super::*;
                const EPSILON: float = 1e-4;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 10], vec![1.], vec![0.; 10]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![1.], vec![0.; 20]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 20], vec![1.]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
            mod original_spectrum_is_two_deltas_20 {
                use super::*;
                const EPSILON: float = 1e-4;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 6], vec![1.], vec![0.; 6], vec![1.], vec![0.; 6]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![1.], vec![0.; 6], vec![1.], vec![0.; 12]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 12], vec![1.], vec![0.; 6], vec![1.]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
        }
        mod instrument_is_symmetric_triangle5 {
            use super::*;
            const POINTS_INSTRUMENT: [float; 5] = [0., 0.5, 1., 0.5, 0.];
            mod original_spectrum_is_delta_21 {
                use super::*;
                const EPSILON: float = 1e-4;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 9], vec![0.5, 1., 0.5], vec![0.; 9]].concat();
                    let points_deconvolved_expected = [vec![0.; 10], vec![1.], vec![0.; 10]].concat();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![1., 0.5], vec![0.; 19]].concat();
                    let points_deconvolved_expected = [vec![1.], vec![0.; 20]].concat();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 19], vec![0.5, 1.]].concat();
                    let points_deconvolved_expected = [vec![0.; 20], vec![1.]].concat();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
            mod original_spectrum_is_two_deltas_20 {
                use super::*;
                const EPSILON: float = 1e-4;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 5], vec![0.5, 1., 0.5], vec![0.; 4], vec![0.5, 1., 0.5], vec![0.; 5]].concat();
                    let points_deconvolved_expected = [vec![0.; 6], vec![1.], vec![0.; 6], vec![1.], vec![0.; 6]].concat();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![1., 0.5], vec![0.; 4], vec![0.5, 1., 0.5], vec![0.; 11]].concat();
                    let points_deconvolved_expected = [vec![1.], vec![0.; 6], vec![1.], vec![0.; 12]].concat();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 11], vec![0.5, 1., 0.5], vec![0.; 4], vec![0.5, 1.]].concat();
                    let points_deconvolved_expected = [vec![0.; 12], vec![1.], vec![0.; 6], vec![1.]].concat();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params.0;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
        }
    }
}

