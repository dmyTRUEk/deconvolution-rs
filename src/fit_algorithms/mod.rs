//! Fit Algorithms.

pub mod differential_evolution;
pub mod pattern_search;
pub mod pattern_search_adaptive_step;
pub mod pattern_search_scaled_step;

use std::cmp::Ordering;

use toml::Value as TomlValue;

use crate::{
    deconvolution::deconvolution_data::DeconvolutionData,
    float_type::float,
    load::Load,
    stacktrace::Stacktrace,
};

use self::{
    differential_evolution::DifferentialEvolution,
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
pub enum FitAlgorithmVariant {
    DifferentialEvolution(DifferentialEvolution),
    PatternSearch(PatternSearch),
    PatternSearchScaledStep(PatternSearchScaledStep),
    PatternSearchAdaptiveStep(PatternSearchAdaptiveStep),
}

impl FitAlgorithmVariant {
    pub fn fit(&self, deconvolution_data: &DeconvolutionData, initial_params: Vec<float>) -> FitResult {
        match self {
            Self::DifferentialEvolution(de)       => de.fit(deconvolution_data),
            Self::PatternSearch(ps)               => ps.fit(deconvolution_data, initial_params),
            Self::PatternSearchScaledStep(psss)   => psss.fit(deconvolution_data, initial_params),
            Self::PatternSearchAdaptiveStep(psas) => psas.fit(deconvolution_data, initial_params),
        }
    }
}

impl Load for FitAlgorithmVariant {
    const TOML_NAME: &'static str = "fit_algorithm";
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        const FIT_ALGORITHMS_NAMES: [&'static str; 4] = [
            DifferentialEvolution::TOML_NAME,
            PatternSearch::TOML_NAME,
            PatternSearchScaledStep::TOML_NAME,
            PatternSearchAdaptiveStep::TOML_NAME,
        ];
        let fit_algorithms = FIT_ALGORITHMS_NAMES
            .map(|fa_name| toml_value.get(fa_name));
        let fit_algorithms_number = fit_algorithms
            .iter()
            .flatten() // flatten on Iter<Option/Result> gives only Some/Ok variants and unwraps them
            .count();
        match fit_algorithms_number.cmp(&1) {
            // TODO: maybe somehow get first entry from table and use `panic_unknown_type` with value
            Ordering::Less    => stacktrace.panic_unknown_type_without_value(FIT_ALGORITHMS_NAMES),
            Ordering::Greater => stacktrace.panic_more_than_one_found(
                fit_algorithms
                    .iter()
                    .zip(FIT_ALGORITHMS_NAMES)
                    .map(|(ofa, fan)| ofa.map(|_| fan))
                    .flatten()
                    .collect::<Vec<_>>()
            ),
            Ordering::Equal => {}
        }
        let fit_algorithm_index = fit_algorithms
            .iter()
            .position(|fa| fa.is_some())
            .unwrap();
        let toml_value = fit_algorithms[fit_algorithm_index].unwrap();
        // TODO(refactor): dont use numbers, bc they must be kept in sync with `DECONVOLUTION_FUNCTIONS_NAMES`
        // - maybe create vec of [PerPoint, Exponents, ...] and try load by them?
        match fit_algorithm_index {
            0 => Self::DifferentialEvolution(DifferentialEvolution::load_from_self_handle_stacktrace(toml_value, stacktrace)),
            1 => Self::PatternSearch(PatternSearch::load_from_self_handle_stacktrace(toml_value, stacktrace)),
            2 => Self::PatternSearchScaledStep(PatternSearchScaledStep::load_from_self_handle_stacktrace(toml_value, stacktrace)),
            3 => Self::PatternSearchAdaptiveStep(PatternSearchAdaptiveStep::load_from_self_handle_stacktrace(toml_value, stacktrace)),
            _ => unreachable!()
        }
    }
}

