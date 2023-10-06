//! Fit Algorithm enum.

use toml::Value as TomlValue;

use crate::{
    config::Load,
    deconvolution_data::DeconvolutionData,
    float_type::float,
};

use super::{
    downhill_simplex::DownhillSimplex,
    pattern_search::PatternSearch,
    pattern_search_adaptive_step::PatternSearchAdaptiveStep,
    pattern_search_scaled_step::PatternSearchScaledStep,
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
    PatternSearch(PatternSearch),
    PatternSearchScaledStep(PatternSearchScaledStep),
    PatternSearchAdaptiveStep(PatternSearchAdaptiveStep),
    DownhillSimplex(DownhillSimplex),
}

impl FitAlgorithm {
    pub fn fit(&self, deconvolution_data: &DeconvolutionData) -> FitResultOrError {
        match &self {
            FitAlgorithm::PatternSearch(ps) => {
                ps.fit(deconvolution_data)
            }
            FitAlgorithm::PatternSearchScaledStep(psss) => {
                psss.fit(deconvolution_data)
            }
            FitAlgorithm::PatternSearchAdaptiveStep(psas) => {
                psas.fit(deconvolution_data)
            }
            FitAlgorithm::DownhillSimplex(ds) => {
                ds.fit(deconvolution_data)
            }
        }
    }
}




impl Load for FitAlgorithm {
    fn load_from_toml_value(toml_value: TomlValue) -> Self {
        const FIT_ALGORITHMS_NAMES: [&str; 4] = [
            "pattern_search",
            "pattern_search_scaled_step",
            "pattern_search_adaptive_step",
            "downhill_simplex",
        ];
        let fit_algorithms: Vec<Option<&TomlValue>> = FIT_ALGORITHMS_NAMES
            .iter()
            .map(|fa_name| toml_value.get(fa_name))
            .collect();
        match fit_algorithms.iter().filter(|fa| fa.is_some()).count() {
            0 => panic!("no known `fit_algorithm.<name>` found"),
            1 => {}
            _ => panic!("too many `fit_algorithm.<name>` found")
        }
        let fit_algorithm_i = fit_algorithms.iter().position(|fa| fa.is_some()).unwrap();
        // TODO(refactor)
        match fit_algorithm_i {
            0 => Self::PatternSearch(
                PatternSearch::load_from_toml_value(
                    fit_algorithms[0].unwrap().clone()
                )
            ),
            1 => Self::PatternSearchScaledStep(
                PatternSearchScaledStep::load_from_toml_value(
                    fit_algorithms[1].unwrap().clone()
                )
            ),
            2 => Self::PatternSearchAdaptiveStep(
                PatternSearchAdaptiveStep::load_from_toml_value(
                    fit_algorithms[2].unwrap().clone()
                )
            ),
            3 => Self::DownhillSimplex(
                DownhillSimplex::load_from_toml_value(
                    fit_algorithms[3].unwrap().clone()
                )
            ),
            _ => unreachable!()
        }
    }
}

