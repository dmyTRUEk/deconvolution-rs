//! Downhill Simplex algorithm.

use toml::Value as TomlValue;

use crate::{
    config::Load,
    deconvolution_data::DeconvolutionData,
    diff_function::DiffFunction,
    extensions::{ArrayMath, Avg, IndexOfMax},
    float_type::float,
    utils_io::press_enter_to_continue,
};

use super::fit_algorithm::{FitResult, FitResultOrError};


#[derive(Debug, Clone, PartialEq)]
pub struct DownhillSimplex {
    pub fit_algorithm_min_step: float,
    // TODO(feat):
    // fit_residue_goal: float,
    pub fit_residue_evals_max: u64,
    pub fit_residue_max_value: float,
    pub initial_simplex_scale: float,
    pub params_diff_type: DiffFunction,
}

impl DownhillSimplex {
    pub fn fit(&self, deconvolution_data: &DeconvolutionData) -> FitResultOrError {
        const DEBUG: bool = false;
        const LERP_TS: [float; 15] = [0.5, 0.45, 0.55, 0.4, 0.6, 0.3, 0.7, 0.2, 0.8, 0.1, 0.9, 0.01, 0.99, 0.001, 0.999];

        let DownhillSimplex { fit_algorithm_min_step, fit_residue_evals_max, fit_residue_max_value, initial_simplex_scale, params_diff_type } = self.clone();

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
}


impl Load for DownhillSimplex {
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

