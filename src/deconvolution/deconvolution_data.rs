//! Deconvolution Data.

use std::{cmp::Ordering, fs::File, io::Write};

use toml::Value as TomlValue;

use crate::{
    fit_algorithms::fit_algorithm::{Fit, FitAlgorithm, FitResult},
    float_type::float,
    load::Load,
    spectrum::Spectrum, stacktrace::Stacktrace,
};

use super::{
    Deconvolution,
    convolution::convolve_by_points,
};


pub type DeconvolutionResultOrError = FitResult;


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
        self.deconvolution.get_initial_values_len(/*self.measured.points.len()*/)
    }

    pub fn get_initial_params(&self) -> Vec<float> {
        let spectrum_measured_len: usize = self.measured.points.len();
        let initial_params: Vec<float> = self.deconvolution.get_initial_values(/*spectrum_measured_len*/);
        assert_eq!(self.deconvolution.get_initial_values_len(/*spectrum_measured_len*/), initial_params.len());
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

    pub fn write_result_to_file(
        &self,
        deconvolution_results: &Fit,
        filepathstr_output: &str,
        desmos_function_str: Result<String, &str>,
        fit_residue_and_evals_msg: &str,
        params: &Vec<float>,
    ) {
        // TODO(refactor): extract common logic
        let mut file_output = File::create(filepathstr_output).unwrap();
        match &self.deconvolution {
            self_ @ Deconvolution::PerPoint { .. } => {
                let sd_deconvolved = Spectrum {
                    points: deconvolution_results.params.clone(),
                    step: self.get_step(),
                    x_start: self.measured.x_start,
                };
                writeln!(file_output, "{name} params ({fit_residue_and_evals_msg}):", name=self_.get_name()).unwrap();
                drop(file_output);
                sd_deconvolved.write_to_file(filepathstr_output);
            }
            self_ @ Deconvolution::Exponents { .. } => {
                writeln!(file_output, "{name} params ({fit_residue_and_evals_msg}):", name=self_.get_name()).unwrap();
                for parts in deconvolution_results.params.chunks(3) {
                    let (amplitude, shift, tau) = (parts[0], parts[1], parts[2]);
                    writeln!(file_output, "amplitude={amplitude}").unwrap();
                    writeln!(file_output, "shift={shift}").unwrap();
                    writeln!(file_output, "tau={tau}").unwrap();
                }
                if let Ok(desmos_function_str) = desmos_function_str {
                    writeln!(file_output, "{desmos_function_str}").unwrap();
                }
            }
            self_ @ Deconvolution::SatExp_DecExp { .. } => {
                writeln!(file_output, "{name} params ({fit_residue_and_evals_msg}):", name=self_.get_name()).unwrap();
                // let (amplitude, shift, tau_a, tau_b) = (params[0], params[1], params[2], params[3]);
                let (shift, tau_a, tau_b) = (params[0], params[1], params[2]);
                // writeln!(file_output, "amplitude={amplitude}").unwrap();
                writeln!(file_output, "shift={shift}").unwrap();
                writeln!(file_output, "tau_a={tau_a}").unwrap();
                writeln!(file_output, "tau_b={tau_b}").unwrap();
                if let Ok(desmos_function_str) = desmos_function_str {
                    writeln!(file_output, "{desmos_function_str}").unwrap();
                }
            }
            self_ @ Deconvolution::Two_SatExp_DecExp { .. } => {
                writeln!(file_output, "{name} params ({fit_residue_and_evals_msg}):", name=self_.get_name()).unwrap();
                let (amplitude_1, shift_1, tau_a1, tau_b1) = (params[0], params[1], params[2], params[3]);
                let (amplitude_2, shift_2, tau_a2, tau_b2) = (params[4], params[5], params[6], params[7]);
                writeln!(file_output, "amplitude_1={amplitude_1}").unwrap();
                writeln!(file_output, "shift_1={shift_1}").unwrap();
                writeln!(file_output, "tau_a1={tau_a1}").unwrap();
                writeln!(file_output, "tau_b1={tau_b1}").unwrap();
                writeln!(file_output, "amplitude_2={amplitude_2}").unwrap();
                writeln!(file_output, "shift_2={shift_2}").unwrap();
                writeln!(file_output, "tau_a2={tau_a2}").unwrap();
                writeln!(file_output, "tau_b2={tau_b2}").unwrap();
                if let Ok(desmos_function_str) = desmos_function_str {
                    writeln!(file_output, "{desmos_function_str}").unwrap();
                }
            }
            self_ @ Deconvolution::SatExp_DecExpPlusConst { .. } => {
                writeln!(file_output, "{name} params ({fit_residue_and_evals_msg}):", name=self_.get_name()).unwrap();
                let (amplitude, shift, height, tau_a, tau_b) = (params[0], params[1], params[2], params[3], params[4]);
                writeln!(file_output, "amplitude={amplitude}").unwrap();
                writeln!(file_output, "shift={shift}").unwrap();
                writeln!(file_output, "height={height}").unwrap();
                writeln!(file_output, "tau_a={tau_a}").unwrap();
                writeln!(file_output, "tau_b={tau_b}").unwrap();
                if let Ok(desmos_function_str) = desmos_function_str {
                    writeln!(file_output, "{desmos_function_str}").unwrap();
                }
            }
            self_ @ Deconvolution::SatExp_TwoDecExp { .. } => {
                writeln!(file_output, "{name} params ({fit_residue_and_evals_msg}):", name=self_.get_name()).unwrap();
                let (amplitude, shift, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4]);
                writeln!(file_output, "amplitude={amplitude}").unwrap();
                writeln!(file_output, "shift={shift}").unwrap();
                writeln!(file_output, "tau_a={tau_a}").unwrap();
                writeln!(file_output, "tau_b={tau_b}").unwrap();
                writeln!(file_output, "tau_c={tau_c}").unwrap();
                if let Ok(desmos_function_str) = desmos_function_str {
                    writeln!(file_output, "{desmos_function_str}").unwrap();
                }
            }
            self_ @ Deconvolution::SatExp_TwoDecExpPlusConst { .. } => {
                writeln!(file_output, "{name} params ({fit_residue_and_evals_msg}):", name=self_.get_name()).unwrap();
                let (amplitude, shift, height, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4], params[5]);
                writeln!(file_output, "amplitude={amplitude}").unwrap();
                writeln!(file_output, "shift={shift}").unwrap();
                writeln!(file_output, "height={height}").unwrap();
                writeln!(file_output, "tau_a={tau_a}").unwrap();
                writeln!(file_output, "tau_b={tau_b}").unwrap();
                writeln!(file_output, "tau_c={tau_c}").unwrap();
                if let Ok(desmos_function_str) = desmos_function_str {
                    writeln!(file_output, "{desmos_function_str}").unwrap();
                }
            }
            self_ @ Deconvolution::SatExp_TwoDecExp_SeparateConsts { .. } => {
                writeln!(file_output, "{name} params ({fit_residue_and_evals_msg}):", name=self_.get_name()).unwrap();
                let (amplitude_b, amplitude_c, shift, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4], params[5]);
                writeln!(file_output, "amplitude_b={amplitude_b}").unwrap();
                writeln!(file_output, "amplitude_c={amplitude_c}").unwrap();
                writeln!(file_output, "shift={shift}").unwrap();
                writeln!(file_output, "tau_a={tau_a}").unwrap();
                writeln!(file_output, "tau_b={tau_b}").unwrap();
                writeln!(file_output, "tau_c={tau_c}").unwrap();
                if let Ok(desmos_function_str) = desmos_function_str {
                    writeln!(file_output, "{desmos_function_str}").unwrap();
                }
            }
        }
    }
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignStepsTo {
    Bigger,
    Smaller,
}

impl Load for AlignStepsTo {
    const TOML_NAME: &'static str = "align_steps_to";
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        let align_steps_to_str = toml_value
            .as_str()
            .unwrap_or_else(|| stacktrace.panic_cant_parse_as("string"));
        match align_steps_to_str {
            "bigger"  => AlignStepsTo::Bigger,
            "smaller" => AlignStepsTo::Smaller,
            _ => stacktrace.panic("unknown type, known types: [`bigger`, `smaller`]")
        }
    }
}

