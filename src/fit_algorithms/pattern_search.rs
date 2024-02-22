//! Simple Pattern Search algorithm.

use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use toml::Value as TomlValue;

use crate::{
    deconvolution::deconvolution_data::DeconvolutionData,
    extensions::IndexOfMinWithCeil,
    load::Load,
    stacktrace::Stacktrace,
    types::{float::float, named_wrappers::{Instrument, InstrumentRevV, Measured, MeasuredV, ParamsG, ParamsV}},
    utils_io::press_enter_to_continue,
};

use super::{Fit, FitResult};


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PatternSearch {
    // TODO(refactor): remove pub (used only for tests)
    pub fit_algorithm_min_step: float,
    // TODO(feat):
    // fit_residue_goal: float,
    pub fit_residue_evals_max: u64,
    pub initial_step: float,
    pub alpha: float,
    pub beta: Option<float>,
}

impl PatternSearch {
    pub fn fit(&self, deconvolution_data: &DeconvolutionData, initial_params: ParamsV) -> FitResult {
        const DEBUG: bool = false;

        let Self { fit_algorithm_min_step, fit_residue_evals_max, initial_step, alpha, beta } = *self;
        let beta = beta.unwrap_or(1. / alpha);

        let f_params_amount: usize = initial_params.0.len();
        if f_params_amount == 0 {
            return Err("too few params");
            // return None;
        }

        let instrument_v_rev: InstrumentRevV = Instrument(deconvolution_data.instrument.points.clone()).into();
        let measured_v: MeasuredV = Measured(deconvolution_data.measured.points.clone()).into();

        let mut params: ParamsV = initial_params;
        let mut step: float = initial_step;
        let mut fit_residue_evals: u64 = 0;

        let mut res_at_current_params: float = deconvolution_data.calc_residue_function_v(&params, &instrument_v_rev, &measured_v);
        fit_residue_evals += 1;
        if DEBUG { println!("res_at_current_params = {}", res_at_current_params) }
        if !res_at_current_params.is_finite() { return Err("`res_at_current_params` isn't finite") }
        // if !res_at_current_params.is_finite() { return None }
        // if res_at_current_params >= fit_residue_max_value { return Err("`res_at_current_params` is too big") }

        while step > fit_algorithm_min_step && fit_residue_evals < fit_residue_evals_max {
        // while residue_function(&params, &points_instrument, &points_spectrum) > FIT_RESIDUE_GOAL && fit_residue_evals < fit_residue_evals_max
            if DEBUG {
                println!("params = {:#?}", params);
                println!("step = {}", step);
            }

            let (fit_residue_evals_extra, ress_at_shifted_params): (Vec<u64>, Vec<float>) = (0..2*f_params_amount)
                // .into_iter()
                .into_par_iter()
                .map(|i| -> (u64, float) {
                    let delta = if i % 2 == 0 { -step } else { step };
                    let param_new = params.0[i/2] + delta;
                    // if !param_new.is_finite() { return Err("`param.value + delta` isn't finite") }
                    let mut params_new = params.clone();
                    params_new.0[i/2] = param_new;
                    if !deconvolution_data.is_params_ok_v(&params_new) || !param_new.is_finite() {
                        (0, float::NAN)
                    } else {
                        let residue = deconvolution_data.calc_residue_function_v(&params_new, &instrument_v_rev, &measured_v);
                        (1, if residue.is_finite() { residue } else { float::NAN })
                    }
                    // returns tuple of `residue_function_evals` and `residue_result`.
                })
                .unzip();
            fit_residue_evals += fit_residue_evals_extra.iter().sum::<u64>();

            if DEBUG { println!("res_at_shifted_params = {:?}", ress_at_shifted_params) }
            assert_eq!(2 * f_params_amount, ress_at_shifted_params.len());
            // if res_at_shifted_params.iter().any(|r| !r.is_finite()) { return Err(format!("one of `res_at_shifted_params` isn't finite")) }

            match ress_at_shifted_params.index_of_min_with_ceil(res_at_current_params) {
                Some(index_of_min) => {
                    if DEBUG { println!("INCREASE STEP") }
                    let param_index = index_of_min as usize / 2;
                    let delta = if index_of_min % 2 == 0 { -step } else { step };
                    params.0[param_index] += delta;

                    res_at_current_params = ress_at_shifted_params[index_of_min];
                    if DEBUG { println!("res_at_current_params = {}", res_at_current_params) }
                    if !res_at_current_params.is_finite() { return Err("`res_at_current_params` isn't finite") }
                    // if !res_at_current_params.is_finite() { return None }
                    // if res_at_current_params >= fit_residue_max_value { return Err("`res_at_current_params` is too big") }

                    step *= alpha;
                }
                None => {
                    if DEBUG { println!("DECREASE STEP") }
                    step *= beta;
                }
            }

            if DEBUG { println!("\n\n") }
        }
        if fit_residue_evals >= fit_residue_evals_max {
            if DEBUG {
                println!("{}", "!".repeat(21));
                println!("HIT MAX_ITERS!!!");
                press_enter_to_continue();
            }
            return Err("hit max evals");
            // return None;
        }
        if DEBUG { println!("finished in {} iters", fit_residue_evals) }
        let params = ParamsG::<float>(params.0.data.into());
        let fit_residue = res_at_current_params;
        Ok(Fit {
            params,
            fit_residue,
            fit_residue_evals,
        })
    }
}


impl Load for PatternSearch {
    const TOML_NAME: &'static str = "pattern_search";
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        let load_float = |name: &'static str| -> float {
            let stacktrace = stacktrace.pushed(name);
            toml_value
                .get(name)
                .unwrap_or_else(|| stacktrace.panic_not_found())
                .as_float()
                .unwrap_or_else(|| stacktrace.panic_cant_parse_as("float"))
        };
        let load_u64 = |name: &'static str| -> u64 {
            let stacktrace = stacktrace.pushed(name);
            let value = toml_value
                .get(name)
                .unwrap_or_else(|| stacktrace.panic_not_found())
                .as_integer()
                .unwrap_or_else(|| stacktrace.panic_cant_parse_as("int"));
            if !(u64::MIN as i128..=u64::MAX as i128).contains(&(value as i128)) {
                stacktrace.panic_cant_parse_as("u64")
            }
            value as u64
        };
        let beta = {
            let name = "beta";
            let stacktrace = stacktrace.pushed(name);
            toml_value
                .get(name)
                .map(|beta_toml_value| {
                    beta_toml_value
                        .as_float()
                        .unwrap_or_else(|| stacktrace.panic_cant_parse_as("float"))
                })
        };
        Self {
            fit_algorithm_min_step: load_float("fit_algorithm_min_step"),
            fit_residue_evals_max: load_u64("fit_residue_evals_max"),
            initial_step: load_float("initial_step"),
            alpha: load_float("alpha"),
            beta,
        }
    }
}

