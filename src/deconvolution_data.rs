//! Deconvolution Data.

use std::cmp::Ordering;

use toml::Value as TomlValue;

use crate::{
    config::Load,
    convolution::convolve_by_points,
    deconvolution::Deconvolution,
    fit_algorithms::fit_algorithm::{FitAlgorithm, FitResultOrError},
    float_type::float,
    spectrum::Spectrum,
};


pub type DeconvolutionResultOrError = FitResultOrError;


#[derive(Debug, Clone, PartialEq)]
pub struct DeconvolutionData {
    pub instrument: Spectrum,
    pub measured: Spectrum,
    pub deconvolution: Deconvolution,
}

impl DeconvolutionData {
    pub fn assert_steps_is_aligned(&self) {
        assert_eq!(self.instrument.step, self.measured.step);
    }

    #[allow(dead_code)]
    pub fn assert_x_starts_is_aligned(&self) {
        assert_eq!(self.instrument.x_start, self.measured.x_start);
    }

    pub fn get_step(&self) -> float {
        self.assert_steps_is_aligned();
        self.instrument.step
    }

    #[allow(dead_code)]
    pub fn get_x_start(&self) -> float {
        self.assert_x_starts_is_aligned();
        self.instrument.x_start
    }

    /// Make [`step`] in [`instrument`] and [`measured`] same,
    /// towards smaller/bigger step (more points in total).
    ///
    /// [`step`]: SpectrumData::step
    /// [`instrument`]: DeconvolutionData::instrument
    /// [`measured`]: DeconvolutionData::measured
    pub fn aligned_steps_to(mut self, align_steps_to: AlignStepsTo) -> Self {
        match self.instrument.step.partial_cmp(&self.measured.step) {
            Some(Ordering::Equal) => return self,
            Some(Ordering::Less) => {
                match align_steps_to {
                    AlignStepsTo::Smaller => {
                        self.measured = self.measured.recalculated_with_step(self.instrument.step);
                    }
                    AlignStepsTo::Bigger => {
                        self.instrument = self.instrument.recalculated_with_step(self.measured.step);
                    }
                }
            }
            Some(Ordering::Greater) => {
                match align_steps_to {
                    AlignStepsTo::Smaller => {
                        self.instrument = self.instrument.recalculated_with_step(self.measured.step);
                    }
                    AlignStepsTo::Bigger => {
                        self.measured = self.measured.recalculated_with_step(self.instrument.step);
                    }
                }
            }
            None => panic!("One of the steps is `NaN`")
        };
        self.assert_steps_is_aligned();
        self
    }

    pub fn deconvolve(&self, fit_algorithm: &FitAlgorithm) -> DeconvolutionResultOrError {
        self.assert_steps_is_aligned();
        fit_algorithm.fit(&self)
    }

    /// Depending on the `self.deconvolution` `params` is:
    /// - PerPoint: list of values at that point,
    /// - Exponents: list of (amplitude, shift, tau),
    /// - SatExp_DecExp: amplitude, shift, tau_a, tau_b,
    /// for other look in [`Deconvolution`].
    pub fn calc_residue_function(&self, params: &Vec<float>) -> float {
        let points_convolved: Vec<float> = self.convolve_from_params(params);
        assert_eq!(self.get_params_amount(), params.len());
        self.deconvolution.calc_residue_function(&self.measured.points, &points_convolved)
    }

    pub fn get_params_amount(&self) -> usize {
        self.deconvolution.get_initial_values_len(self.measured.points.len())
    }

    pub fn get_initial_params(&self) -> Vec<float> {
        let spectrum_measured_len: usize = self.measured.points.len();
        let initial_params: Vec<float> = self.deconvolution.get_initial_values(spectrum_measured_len);
        assert_eq!(self.deconvolution.get_initial_values_len(spectrum_measured_len), initial_params.len());
        initial_params
    }

    pub fn is_params_ok(&self, params: &Vec<float>) -> bool {
        self.deconvolution.is_params_ok(params)
    }

    pub fn convolve_from_params(&self, params: &Vec<float>) -> Vec<float> {
        // convert `params` into `points` ("deconvolved"):
        let points_deconvolved: Vec<float> = self.deconvolution.params_to_points(
            &params,
            self.measured.points.len(),
            (self.measured.x_start, self.measured.get_x_end())
        );
        self.convolve_from_points(&points_deconvolved)
    }

    pub fn convolve_from_points(&self, points_deconvolved: &Vec<float>) -> Vec<float> {
        let points_convolved: Vec<float> = convolve_by_points(&self.instrument.points, &points_deconvolved);
        assert_eq!(self.measured.points.len(), points_convolved.len());
        points_convolved
    }
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignStepsTo {
    Bigger,
    Smaller,
}

impl Load for AlignStepsTo {
    fn load_from_toml_value(toml_value: &TomlValue) -> Self {
        let align_steps_to_str = toml_value
            .as_str()
            .expect("align_steps_to: can't parse as string");
        match align_steps_to_str {
            "bigger"  => AlignStepsTo::Bigger,
            "smaller" => AlignStepsTo::Smaller,
            _ => panic!("unknown `align_steps_to`")
        }
    }
}

