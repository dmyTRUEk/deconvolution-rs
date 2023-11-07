//! Simple Pattern Search algorithm.

use std::cmp::Ordering;

use rand::{thread_rng, Rng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use toml::Value as TomlValue;

use crate::{
    deconvolution::deconvolution_data::DeconvolutionData,
    extensions::IndexOfMin,
    float_type::float,
    load::Load,
    stacktrace::Stacktrace,
    unmut,
};

use super::{Fit, FitResult};


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DifferentialEvolution {
    initial_values_random_scale: float,
    generations: usize,
    population: usize,
    mutation_speed: float,
    crossover_probability: float,
    // TODO(feat):
    // fit_residue_goal: float,
}

impl DifferentialEvolution {
    pub fn fit(&self, deconvolution_data: &DeconvolutionData) -> FitResult {
        const DEBUG: bool = false;

        let Self { initial_values_random_scale, generations, population, mutation_speed, crossover_probability } = *self;

        let f_params_amount: usize = deconvolution_data.deconvolution.get_initial_values_len();
        if f_params_amount == 0 {
            return Err("too few params");
            // return None;
        }

        let mut rng = thread_rng();

        type Params = Vec<float>;
        let mut generation = Vec::<Params>::from_iter(
            (0..population)
                .map(|_|
                    deconvolution_data.deconvolution.get_initial_values_randomized_with_rng(initial_values_random_scale, &mut rng)
                )
        );
        let mut fit_residue_evals: u64 = 0;

        let mut ress_of_current_gen: Vec<float> = generation
            .iter()
            .map(|p| deconvolution_data.calc_residue_function(p))
            .collect();
        fit_residue_evals += population as u64;
        if DEBUG { println!("res_at_current_gen = {:?}", ress_of_current_gen) }
        if ress_of_current_gen.iter().all(|r| !r.is_finite()) { return Err("`res_at_current_params` isn't finite") }
        // if !res_at_current_params.is_finite() { return None }
        // if ress_of_current_gen.iter().all(|&r| r >= fit_residue_max_value) { return Err("`res_at_current_params` is too big") }

        let mut successful_mutations: u64 = 0;
        for gen_i in 0..generations {
            // if fit_residue_evals >= fit_residue_evals_max {
            //     break;
            // }
            if DEBUG {
                println!("generation = {:#?}", generation);
            }

            // indices to use for create childer
            let mut new_generation = vec![];
            for child_i in 0..population {
                let parent_a_i = rng.gen_range(0..population);
                let parent_b_i = rng.gen_range(0..population);
                let parent_c_i = rng.gen_range(0..population);

                let parent_a = generation[parent_a_i].clone();
                let parent_b = generation[parent_b_i].clone();
                let parent_c = generation[parent_c_i].clone();

                let child_pure: Params = (0..f_params_amount)
                    .map(|i|
                        parent_a[i] + mutation_speed * (parent_b[i] - parent_c[i])
                    )
                    .collect();

                let child = (0..f_params_amount)
                    .map(|i| {
                        let parent = generation[child_i].clone();
                        if rng.gen_range(0.0..=1.0) < crossover_probability {
                            child_pure[i]
                        } else {
                            parent[i]
                        }
                    })
                    .collect();

                new_generation.push(child);
            }
            unmut!(new_generation);

            let (fit_residue_evals_extra, ress_of_new_gen): (Vec<u64>, Vec<float>) = (&new_generation)
                // .into_iter()
                .into_par_iter()
                .map(|params_new| -> (u64, float) {
                    if !deconvolution_data.is_params_ok(params_new) {
                        (0, float::NAN)
                    } else {
                        let residue = deconvolution_data.calc_residue_function(params_new);
                        (1, if residue.is_finite() { residue } else { float::NAN })
                    }
                    // returns tuple of `residue_function_evals` and `residue_result`.
                })
                .unzip();
            fit_residue_evals += fit_residue_evals_extra.iter().sum::<u64>();

            if DEBUG { println!("res_of_new_gen = {:?}", ress_of_new_gen) }
            assert_eq!(population, ress_of_new_gen.len());
            // if res_at_shifted_params.iter().any(|r| !r.is_finite()) { return Err(format!("one of `res_at_shifted_params` isn't finite")) }

            for i in 0..population {
                match ress_of_new_gen[i].partial_cmp(&ress_of_current_gen[i]) {
                    Some(Ordering::Less) => {
                        generation[i] = new_generation[i].clone();
                        ress_of_current_gen[i] = ress_of_new_gen[i];
                        successful_mutations += 1;
                    }
                    _ => {}
                }
            }
            if DEBUG {
                println!(
                    "gen {gen_i}: successful_mutations = {successful_mutations}, best residue = {}",
                    ress_of_current_gen.iter()
                        .filter(|x| x.is_finite())
                        .min_by(|x, y| x.partial_cmp(y).unwrap())
                        .unwrap()
                );
            }
        }
        // if fit_residue_evals >= fit_residue_evals_max {
        //     if DEBUG {
        //         println!("{}", "!".repeat(21));
        //         println!("HIT MAX_ITERS!!!");
        //         press_enter_to_continue();
        //     }
        //     return Err("hit max evals");
        //     // return None;
        // }
        if DEBUG { println!("finished in {} iters", fit_residue_evals) }
        let best_i = ress_of_current_gen.index_of_min().unwrap();
        let params = generation[best_i].clone();
        let fit_residue = ress_of_current_gen[best_i];
        Ok(Fit {
            params,
            fit_residue,
            fit_residue_evals,
        })
    }
}


impl Load for DifferentialEvolution {
    const TOML_NAME: &'static str = "differential_evolution";
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        let load_float = |name: &'static str| -> float {
            let stacktrace = stacktrace.pushed(name);
            toml_value
                .get(name)
                .unwrap_or_else(|| stacktrace.panic_not_found())
                .as_float()
                .unwrap_or_else(|| stacktrace.panic_cant_parse_as("float"))
        };
        let load_usize = |name: &'static str| -> usize {
            let stacktrace = stacktrace.pushed(name);
            let value = toml_value
                .get(name)
                .unwrap_or_else(|| stacktrace.panic_not_found())
                .as_integer()
                .unwrap_or_else(|| stacktrace.panic_cant_parse_as("int"));
            if !(usize::MIN as i128..=usize::MAX as i128).contains(&(value as i128)) {
                stacktrace.panic_cant_parse_as("usize")
            }
            value as usize
        };
        Self {
            initial_values_random_scale: load_float("initial_values_random_scale"),
            generations: load_usize("generations"),
            population: load_usize("population"),
            mutation_speed: load_float("mutation_speed"),
            crossover_probability: load_float("crossover_probability"),
        }
    }
}

