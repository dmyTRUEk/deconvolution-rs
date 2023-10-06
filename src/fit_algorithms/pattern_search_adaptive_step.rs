//! Pattern Search using step scaled by current params.

use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use toml::Value as TomlValue;

use crate::{
    config::Load,
    deconvolution_data::DeconvolutionData,
    extensions::IndexOfMinWithCeil,
    float_type::float,
    utils_io::press_enter_to_continue,
};

use super::fit_algorithm::{FitResult, FitResultOrError};


#[derive(Debug, Clone, PartialEq)]
pub struct PatternSearchAdaptiveStep {
    pub fit_algorithm_min_step: float,
    // TODO(feat):
    // fit_residue_goal: float,
    pub fit_residue_evals_max: u64,
    pub fit_residue_max_value: float,
    pub initial_step: float,
    pub alpha: float,
    pub beta: Option<float>,
}

impl PatternSearchAdaptiveStep {
    pub fn fit(&self, deconvolution_data: &DeconvolutionData) -> FitResultOrError {
        const DEBUG: bool = false;

        let PatternSearchAdaptiveStep { fit_algorithm_min_step, fit_residue_evals_max, fit_residue_max_value, initial_step, alpha, beta } = self.clone();
        let beta = beta.unwrap_or(1. / alpha);

        let f_params_amount: usize = deconvolution_data.get_params_amount();
        if f_params_amount == 0 {
            return Err("too few params");
            // return None;
        }

        type Params = Vec<float>;
        let mut params: Params = deconvolution_data.get_initial_params();
        let mut step: float = initial_step;
        let mut fit_residue_evals: u64 = 0;

        let mut res_at_current_params: float = deconvolution_data.calc_residue_function(&params);
        fit_residue_evals += 1;
        if DEBUG { println!("res_at_current_params = {}", res_at_current_params) }
        if !res_at_current_params.is_finite() { return Err("`res_at_current_params` isn't finite") }
        // if !res_at_current_params.is_finite() { return None }
        if res_at_current_params >= fit_residue_max_value { return Err("`res_at_current_params` is too big") }

        while step > fit_algorithm_min_step && fit_residue_evals < fit_residue_evals_max {
        // while residue_function(&params, &points_instrument, &points_spectrum) > FIT_RESIDUE_GOAL && fit_residue_evals < fit_residue_evals_max
            if DEBUG {
                println!("params = {:#?}", params);
                println!("step = {}", step);
            }

            let (fit_residue_evals_extra, ress_at_shifted_params): (Vec<u64>, Vec<float>) =
                (0..2*params.len())
                    // .into_iter()
                    .into_par_iter()
                    .map(|i| -> (u64, float) {
                        let delta = if i % 2 == 0 { -step } else { step };
                        let delta = params[i/2] * delta;
                        let param_new = params[i/2] + delta;
                        // if !param_new.is_finite() { return Err("`param.value + delta` isn't finite") }
                        // TODO(optimization)?: remove `.is_finite()` check, bc it already will be "done" when calculating residue function.
                        let mut params_new = params.clone();
                        params_new[i/2] = param_new;
                        if !param_new.is_finite() || !deconvolution_data.is_params_ok(&params_new) {
                            (0, float::NAN)
                        } else {
                            let res = deconvolution_data.calc_residue_function(&params_new);
                            (1, if res.is_finite() { res } else { float::NAN })
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
                    let delta = params[index_of_min/2] * delta;
                    params[param_index] += delta;

                    res_at_current_params = ress_at_shifted_params[index_of_min];
                    if DEBUG { println!("res_at_current_params = {}", res_at_current_params) }
                    if !res_at_current_params.is_finite() { return Err("`res_at_current_params` isn't finite") }
                    // if !res_at_current_params.is_finite() { return None }
                    if res_at_current_params >= fit_residue_max_value { return Err("`res_at_current_params` is too big") }

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
        let fit_residue = res_at_current_params;
        Ok(FitResult {
            params,
            fit_residue,
            fit_residue_evals,
        })
    }
}


impl Load for PatternSearchAdaptiveStep {
    fn load_from_toml_value(toml_value: TomlValue) -> Self {
        let fit_algorithm_min_step = toml_value
            .get("fit_algorithm_min_step")
            .expect("fit_params -> pattern_search_adaptive_step: `fit_algorithm_min_step` not found")
            .as_float()
            .expect("fit_params -> pattern_search_adaptive_step -> fit_algorithm_min_step: can't parse as float");
        let fit_residue_evals_max = toml_value
            .get("fit_residue_evals_max")
            .expect("fit_params -> pattern_search_adaptive_step: `fit_residue_evals_max` not found")
            .as_integer()
            .expect("fit_params -> pattern_search_adaptive_step -> fit_residue_evals_max: can't parse as integer");
        let fit_residue_max_value = toml_value
            .get("fit_residue_max_value")
            .expect("fit_params -> pattern_search_adaptive_step: `fit_residue_max_value` not found")
            .as_float()
            .expect("fit_params -> pattern_search_adaptive_step -> fit_residue_max_value: can't parse as float");
        assert!(fit_residue_evals_max > 0);
        let fit_residue_evals_max = fit_residue_evals_max as u64;
        let initial_step = toml_value
            .get("initial_step")
            .expect("fit_method_params -> pattern_search_adaptive_step: `initial_step` not found")
            .as_float()
            .expect("fit_method_params -> pattern_search_adaptive_step -> initial_step: can't parse as float");
        let alpha = toml_value
            .get("alpha")
            .expect("fit_method_params -> pattern_search_adaptive_step: `alpha` not found")
            .as_float()
            .expect("fit_method_params -> pattern_search_adaptive_step -> alpha: can't parse as float");
        let beta = if let Some(beta_toml_value) = toml_value.get("beta") {
            Some(
                beta_toml_value
                    .as_float()
                    .expect("fit_method_params -> pattern_search_adaptive_step -> beta: can't parse as float")
            )
        } else {
            None
        };
        Self {
            fit_algorithm_min_step,
            fit_residue_evals_max,
            fit_residue_max_value,
            initial_step,
            alpha,
            beta,
        }
    }
}

