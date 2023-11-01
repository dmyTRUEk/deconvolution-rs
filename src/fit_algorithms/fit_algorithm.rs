//! Fit Algorithm enum.

use std::cmp::Ordering;

use toml::Value as TomlValue;

use crate::{
    config::Load,
    deconvolution::deconvolution_data::DeconvolutionData,
    float_type::float,
};

use super::{
    pattern_search::PatternSearch,
    pattern_search_adaptive_step::PatternSearchAdaptiveStep,
    pattern_search_scaled_step::PatternSearchScaledStep,
};


#[derive(Debug)]
pub struct Fit {
    pub params: Vec<float>,
    pub fit_residue: float,
    pub fit_residue_evals: u64,
}


// type FitResult = Option<Fit>;
pub type FitResult = Result<Fit, &'static str>;


#[derive(Debug, Clone, PartialEq)]
pub enum FitAlgorithm {
    PatternSearch(PatternSearch),
    PatternSearchScaledStep(PatternSearchScaledStep),
    PatternSearchAdaptiveStep(PatternSearchAdaptiveStep),
}

impl FitAlgorithm {
    pub fn fit(&self, deconvolution_data: &DeconvolutionData) -> FitResult {
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
        }
    }
}

impl Load for FitAlgorithm {
    const TOML_NAME: &'static str = "fit_algorithm";
    fn load_from_self_toml_value(toml_value: &TomlValue) -> Self {
        const FIT_ALGORITHMS_NAMES: [&'static str; 3] = [
            PatternSearch::TOML_NAME,
            PatternSearchScaledStep::TOML_NAME,
            PatternSearchAdaptiveStep::TOML_NAME,
        ];
        let fit_algorithms = FIT_ALGORITHMS_NAMES
            .map(|fa_name| toml_value.get(fa_name));
        let fit_algorithms_number = fit_algorithms
            .iter()
            .filter(|fa| fa.is_some())
            .count();
        match fit_algorithms_number.cmp(&1) {
            Ordering::Less    => panic!("no known `fit_algorithm.<name>` found"),
            Ordering::Greater => panic!("too many `fit_algorithm.<name>` found"),
            Ordering::Equal => {}
        }
        let fit_algorithm_index = fit_algorithms
            .iter()
            .position(|fa| fa.is_some())
            .unwrap();
        let toml_value = fit_algorithms[fit_algorithm_index].unwrap();
        // TODO(refactor)
        match fit_algorithm_index {
            0 => Self::PatternSearch(
                PatternSearch::load_from_self_toml_value(toml_value)
            ),
            1 => Self::PatternSearchScaledStep(
                PatternSearchScaledStep::load_from_self_toml_value(toml_value)
            ),
            2 => Self::PatternSearchAdaptiveStep(
                PatternSearchAdaptiveStep::load_from_self_toml_value(toml_value)
            ),
            _ => unreachable!()
        }
    }
}

