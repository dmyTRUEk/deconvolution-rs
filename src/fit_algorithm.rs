//! Fit Algorithm.

use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use toml::Value as TomlValue;

use crate::{
    config::Load,
    deconvolution_data::DeconvolutionData,
    diff_function::DiffFunction,
    extensions::{ArrayMath, Avg, IndexOfMax, IndexOfMinWithCeil},
    float_type::float,
    utils_io::press_enter_to_continue,
};


#[derive(Debug)]
pub struct FitResult {
    pub params: Vec<float>,
    pub fit_residue: float,
    pub fit_residue_evals: u64,
}


// type FitResultsOrNone = Option<FitResults>;
pub type FitResultOrError = Result<FitResult, &'static str>;


#[derive(Debug, Clone, PartialEq)]
pub enum FitAlgorithm {
    PatternSearch(PatternSearchParams),
    DownhillSimplex(DownhillSimplexParams),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PatternSearchParams {
    pub fit_algorithm_min_step: float,
    // TODO(feat):
    // fit_residue_goal: float,
    pub fit_residue_evals_max: u64,
    pub fit_residue_max_value: float,
    pub initial_step: float,
    pub alpha: float,
    pub beta: Option<float>,
    // TODO: PatternSearchScaleStep
}

#[derive(Debug, Clone, PartialEq)]
pub struct DownhillSimplexParams {
    pub fit_algorithm_min_step: float,
    // TODO(feat):
    // fit_residue_goal: float,
    pub fit_residue_evals_max: u64,
    pub fit_residue_max_value: float,
    pub initial_simplex_scale: float,
    pub params_diff_type: DiffFunction,
}

impl FitAlgorithm {
    pub fn fit(&self, deconvolution_data: &DeconvolutionData) -> FitResultOrError {
        match &self {
            FitAlgorithm::PatternSearch  (psp) => fit_by_pattern_search_algorithm  (deconvolution_data, psp.clone()),
            FitAlgorithm::DownhillSimplex(dsp) => fit_by_downhill_simplex_algorithm(deconvolution_data, dsp.clone()),
        }
    }
}

fn fit_by_pattern_search_algorithm(
    deconvolution_data: &DeconvolutionData,
    pattern_search_params: PatternSearchParams,
) -> FitResultOrError {
    const DEBUG: bool = false;

    let PatternSearchParams { fit_algorithm_min_step, fit_residue_evals_max, fit_residue_max_value, initial_step, alpha, beta } = pattern_search_params;
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


fn fit_by_downhill_simplex_algorithm(
    deconvolution_data: &DeconvolutionData,
    downhill_simplex_params: DownhillSimplexParams,
) -> FitResultOrError {
    const DEBUG: bool = false;
    const LERP_TS: [float; 15] = [0.5, 0.45, 0.55, 0.4, 0.6, 0.3, 0.7, 0.2, 0.8, 0.1, 0.9, 0.01, 0.99, 0.001, 0.999];

    let DownhillSimplexParams { fit_algorithm_min_step, fit_residue_evals_max, fit_residue_max_value, initial_simplex_scale, params_diff_type } = downhill_simplex_params;

    let f_params_amount: usize = deconvolution_data.get_params_amount();
    if f_params_amount == 0 {
        return Err("too few params");
        // return None;
    }

    type Params = Vec<float>;
    let is_close_enough = |params_a: &Params, params_b: &Params| -> bool {
        let diff = params_diff_type.calc_diff(params_a, params_b);
        diff < fit_algorithm_min_step / (params_a.len() as float).powi(1)
    };

    let mut fit_residue_evals = 0;
    // unimplemented!("must rewrite using all new methods and fields");
    // let mut params_prev_prev: Params = vec![INITIAL_VALUES+initial_simplex_scale; f_params_amount];
    // let mut params_prev_this: Params = vec![INITIAL_VALUES-initial_simplex_scale; f_params_amount];
    let mut params_prev_prev: Params = deconvolution_data.get_initial_params().into_iter().map(|p| p + initial_simplex_scale).collect::<Vec<float>>();
    let mut params_prev_this: Params = deconvolution_data.get_initial_params().into_iter().map(|p| p - initial_simplex_scale).collect::<Vec<float>>();
    //                                 data  residue
    let mut params_and_ress_vec: Vec<(Params, float)> = Vec::with_capacity(f_params_amount+1);
    trait ExtParamsAndRessVec {
        fn get_params(&self) -> Vec<Params>;
        fn get_residues(&self) -> Vec<float>;
    }
    impl ExtParamsAndRessVec for Vec<(Params, float)> {
        fn get_params(&self) -> Vec<Params> {
            self.iter().map(|p| p.0.clone()).collect()
        }
        fn get_residues(&self) -> Vec<float> {
            self.iter().map(|p| p.1).collect()
        }
    }
    trait ExtParamsVec {
        fn get_all_except(&self, index: usize) -> Vec<Params>;
    }
    impl ExtParamsVec for Vec<Params> {
        fn get_all_except(&self, index: usize) -> Vec<Params> {
            let mut self_ = self.clone();
            self_.remove(index);
            self_
        }
    }
    trait ExtParams {
        fn mirror_relative_to(self, others: Vec<Params>) -> Params;
        fn lerp(self, other: Params, t: float) -> Params;
    }
    impl ExtParams for Params {
        fn mirror_relative_to(self, others: Vec<Params>) -> Params {
            let others_avg: Params = others.avg();
            self.add(others_avg.sub(self.clone()).scale(2.))
        }
        fn lerp(self, other: Params, t: float) -> Params {
            self.scale(t).add(other.scale(1.-t))
        }
    }
    let mut params_and_ress_vec_push = |params: Params| {
        let fit_residue = deconvolution_data.calc_residue_function(&params);
        fit_residue_evals += 1;
        params_and_ress_vec.push((params, fit_residue));
    };
    // params_and_ress_vec_push(vec![INITIAL_VALUES-initial_simplex_scale/(f_params_amount as float); f_params_amount]);
    params_and_ress_vec_push(deconvolution_data.get_initial_params().into_iter().map(|p| p - initial_simplex_scale/(f_params_amount as float)).collect::<Vec<float>>());
    for i in 0..f_params_amount {
        // let mut params = vec![INITIAL_VALUES; f_params_amount];
        let mut params = deconvolution_data.get_initial_params();
        params[i] += initial_simplex_scale;
        params_and_ress_vec_push(params);
    }
    assert_eq!(f_params_amount+1, params_and_ress_vec.len());
    assert_eq!(f_params_amount, params_prev_this.len());
    while !is_close_enough(&params_prev_this, &params_prev_prev) && fit_residue_evals < fit_residue_evals_max {
        let index_of_max = params_and_ress_vec.get_residues().index_of_max();
        if index_of_max.is_none() { return Err("`fit_residue` at all `params_vec` is NaN or Inf") }
        // if index_of_max.is_none() { return None }
        let index_of_max = index_of_max.unwrap();

        let (params_max, value_at_params_max) = params_and_ress_vec[index_of_max].clone();
        let params_other: Vec<Params> = params_and_ress_vec.get_params().get_all_except(index_of_max);
        // assert_eq!(f_params_amount, params_other.len());

        let params_symmetric = params_max.clone().mirror_relative_to(params_other.clone());
        let value_at_params_symmetric = if deconvolution_data.is_params_ok(&params_symmetric) {
            fit_residue_evals += 1;
            deconvolution_data.calc_residue_function(&params_symmetric)
        } else {
            float::NAN
        };

        params_prev_prev = params_prev_this;
        params_and_ress_vec[index_of_max] = if value_at_params_symmetric < value_at_params_max && value_at_params_symmetric.is_finite() {
            (params_symmetric, value_at_params_symmetric)
        } else {
            let mut option_params_value: Option<(Params, float)> = None;
            for lerp_t in LERP_TS {
                let params_lerp = params_max.clone().lerp(params_other.clone().avg(), lerp_t);
                let value_at_params_lerp = if deconvolution_data.is_params_ok(&params_lerp) {
                    fit_residue_evals += 1;
                    deconvolution_data.calc_residue_function(&params_lerp)
                } else {
                    float::NAN
                };
                if value_at_params_lerp.is_finite() {
                    option_params_value = Some((params_lerp, value_at_params_lerp));
                    break
                }
            }
            match option_params_value {
                None => return Err("`fit_residue` at all `LERP_TS` is NaN or Inf"),
                // None => return None,
                Some(params_value) => params_value
            }
        };
        params_prev_this = params_and_ress_vec[index_of_max].0.clone();
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
    let points = params_and_ress_vec.get_params().avg();
    let fit_residue = deconvolution_data.calc_residue_function(&points);
    fit_residue_evals += 1;
    Ok(FitResult {
        params: points,
        fit_residue,
        fit_residue_evals,
    })
}



impl Load for FitAlgorithm {
    fn load_from_toml_value(toml_value: TomlValue) -> Self {
        match (toml_value.get("pattern_search"), toml_value.get("downhill_simplex")) {
            (Some(toml_value), None) => {
                Self::PatternSearch(
                    PatternSearchParams::load_from_toml_value(
                        toml_value.clone()
                    )
                )
            }
            (None, Some(toml_value)) => {
                Self::DownhillSimplex(
                    DownhillSimplexParams::load_from_toml_value(
                        toml_value.clone()
                    )
                )
            }
            (None, None) => {
                panic!("nor `pattern_search` nor `downhill_simplex` not found")
            }
            (Some(_), Some(_)) => {
                panic!("both `pattern_search` and `downhill_simplex` found")
            }
        }
    }
}

impl Load for PatternSearchParams {
    fn load_from_toml_value(toml_value: TomlValue) -> Self {
        let fit_algorithm_min_step = toml_value
            .get("fit_algorithm_min_step")
            .expect("fit_params -> pattern_search: `fit_algorithm_min_step` not found")
            .as_float()
            .expect("fit_params -> pattern_search -> fit_algorithm_min_step: can't parse as float");
        let fit_residue_evals_max = toml_value
            .get("fit_residue_evals_max")
            .expect("fit_params -> pattern_search: `fit_residue_evals_max` not found")
            .as_integer()
            .expect("fit_params -> pattern_search -> fit_residue_evals_max: can't parse as integer");
        let fit_residue_max_value = toml_value
            .get("fit_residue_max_value")
            .expect("fit_params -> pattern_search: `fit_residue_max_value` not found")
            .as_float()
            .expect("fit_params -> pattern_search -> fit_residue_max_value: can't parse as float");
        assert!(fit_residue_evals_max > 0);
        let fit_residue_evals_max = fit_residue_evals_max as u64;
        let initial_step = toml_value
            .get("initial_step")
            .expect("fit_method_params -> pattern_search: `initial_step` not found")
            .as_float()
            .expect("fit_method_params -> pattern_search -> initial_step: can't parse as float");
        let alpha = toml_value
            .get("alpha")
            .expect("fit_method_params -> pattern_search: `alpha` not found")
            .as_float()
            .expect("fit_method_params -> pattern_search -> alpha: can't parse as float");
        let beta = if let Some(beta_toml_value) = toml_value.get("beta") {
            Some(
                beta_toml_value
                    .as_float()
                    .expect("fit_method_params -> pattern_search -> beta: can't parse as float")
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

impl Load for DownhillSimplexParams {
    fn load_from_toml_value(toml_value: TomlValue) -> Self {
        let fit_algorithm_min_step = toml_value
            .get("fit_algorithm_min_step")
            .expect("fit_params -> downhill_simplex: `fit_algorithm_min_step` not found")
            .as_float()
            .expect("fit_params -> downhill_simplex -> fit_algorithm_min_step: can't parse as float");
        let fit_residue_evals_max = toml_value
            .get("fit_residue_evals_max")
            .expect("fit_params -> downhill_simplex: `fit_residue_evals_max` not found")
            .as_integer()
            .expect("fit_params -> downhill_simplex -> fit_residue_evals_max: can't parse as integer");
        let fit_residue_max_value = toml_value
            .get("fit_residue_max_value")
            .expect("fit_params -> downhill_simplex: `fit_residue_max_value` not found")
            .as_float()
            .expect("fit_params -> downhill_simplex -> fit_residue_max_value: can't parse as float");
        assert!(fit_residue_evals_max > 0);
        let fit_residue_evals_max = fit_residue_evals_max as u64;
        let initial_simplex_scale = toml_value
            .get("initial_simplex_scale")
            .expect("fit_params -> downhill_simplex: `initial_simplex_scale` not found")
            .as_float()
            .expect("fit_params -> downhill_simplex -> initial_simplex_scale: can't parse as float");
        let params_diff_type = DiffFunction::load_from_toml_value(
            toml_value
                .get("params_diff_type")
                .expect("fit_params -> downhill_simplex: `params_diff_type` not found")
                .clone()
        );
        Self {
            fit_algorithm_min_step,
            fit_residue_evals_max,
            fit_residue_max_value,
            initial_simplex_scale,
            params_diff_type,
        }
    }
}

