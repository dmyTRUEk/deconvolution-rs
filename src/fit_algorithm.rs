//! Fit Algorithm.

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{
    deconvolution_data::DeconvolutionData,
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


#[allow(dead_code)]
#[derive(Debug)]
pub enum FitAlgorithm {
    PatternSearch,
    DownhillSimplex,
}

impl FitAlgorithm {
    pub fn fit(&self, deconvolution_data: &DeconvolutionData) -> FitResultOrError {
        match &self {
            FitAlgorithm::PatternSearch   => Self::fit_by_pattern_search_algorithm  (deconvolution_data),
            FitAlgorithm::DownhillSimplex => Self::fit_by_downhill_simplex_algorithm(deconvolution_data),
        }
    }

    fn fit_by_pattern_search_algorithm(deconvolution_data: &DeconvolutionData) -> FitResultOrError {
        use crate::{fit_params::*, pattern_search_params::*};
        const DEBUG: bool = false;

        let f_params_amount: usize = deconvolution_data.get_params_amount();
        if f_params_amount == 0 {
            return Err("too few params");
            // return None;
        }

        type Params = Vec<float>;
        let mut params: Params = deconvolution_data.get_initial_params();
        let mut step: float = INITIAL_STEP;
        let mut fit_residue_evals: u64 = 0;

        let mut res_at_current_params: float = deconvolution_data.calc_residue_function(&params);
        fit_residue_evals += 1;
        if DEBUG { println!("res_at_current_params = {}", res_at_current_params) }
        if !res_at_current_params.is_finite() { return Err("`res_at_current_params` isn't finite") }
        // if !res_at_current_params.is_finite() { return None }
        if res_at_current_params >= FIT_RESIDUE_MAX_VALUE { return Err("`res_at_current_params` is too big") }

        while step > FIT_ALGORITHM_MIN_STEP && fit_residue_evals < FIT_RESIDUE_EVALS_MAX {
        // while residue_function(&params, &points_instrument, &points_spectrum) > FIT_RESIDUE_GOAL && fit_residue_evals < FIT_RESIDUE_EVALS_MAX
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
                    if res_at_current_params >= FIT_RESIDUE_MAX_VALUE { return Err("`res_at_current_params` is too big") }

                    step *= ALPHA;
                }
                None => {
                    if DEBUG { println!("DECREASE STEP") }
                    step *= BETA;
                }
            }

            if DEBUG { println!("\n\n") }
        }
        if fit_residue_evals >= FIT_RESIDUE_EVALS_MAX {
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


    #[allow(unused)]
    fn fit_by_downhill_simplex_algorithm(deconvolution_data: &DeconvolutionData) -> FitResultOrError {
        use crate::{downhill_simplex_params::*, fit_params::*};
        const DEBUG: bool = false;
        const LERP_TS: [float; 15] = [0.5, 0.45, 0.55, 0.4, 0.6, 0.3, 0.7, 0.2, 0.8, 0.1, 0.9, 0.01, 0.99, 0.001, 0.999];

        let f_params_amount: usize = deconvolution_data.get_params_amount();
        if f_params_amount == 0 {
            return Err("too few params");
            // return None;
        }

        type Params = Vec<float>;
        fn is_close_enough(params_a: &Params, params_b: &Params) -> bool {
            let diff = PARAMS_DIFF_TYPE.calc_diff(params_a, params_b);
            diff < FIT_ALGORITHM_MIN_STEP / (params_a.len() as float).powi(1)
        }

        let mut fit_residue_evals = 0;
        // unimplemented!("must rewrite using all new methods and fields");
        // let mut params_prev_prev: Params = vec![INITIAL_VALUES+INITIAL_SIMPLEX_SCALE; f_params_amount];
        // let mut params_prev_this: Params = vec![INITIAL_VALUES-INITIAL_SIMPLEX_SCALE; f_params_amount];
        let mut params_prev_prev: Params = deconvolution_data.get_initial_params().into_iter().map(|p| p + INITIAL_SIMPLEX_SCALE).collect::<Vec<float>>();
        let mut params_prev_this: Params = deconvolution_data.get_initial_params().into_iter().map(|p| p - INITIAL_SIMPLEX_SCALE).collect::<Vec<float>>();
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
        // params_and_ress_vec_push(vec![INITIAL_VALUES-INITIAL_SIMPLEX_SCALE/(f_params_amount as float); f_params_amount]);
        params_and_ress_vec_push(deconvolution_data.get_initial_params().into_iter().map(|p| p - INITIAL_SIMPLEX_SCALE/(f_params_amount as float)).collect::<Vec<float>>());
        for i in 0..f_params_amount {
            // let mut params = vec![INITIAL_VALUES; f_params_amount];
            let mut params = deconvolution_data.get_initial_params();
            params[i] += INITIAL_SIMPLEX_SCALE;
            params_and_ress_vec_push(params);
        }
        assert_eq!(f_params_amount+1, params_and_ress_vec.len());
        assert_eq!(f_params_amount, params_prev_this.len());
        while !is_close_enough(&params_prev_this, &params_prev_prev) && fit_residue_evals < FIT_RESIDUE_EVALS_MAX {
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
        if fit_residue_evals >= FIT_RESIDUE_EVALS_MAX {
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

}


