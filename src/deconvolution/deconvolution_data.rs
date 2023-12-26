//! Deconvolution Data.

use std::{cmp::Ordering, fs::File, io::Write};

use toml::Value as TomlValue;

use crate::{
    fit_algorithms::{Fit, FitAlgorithmVariant, FitResult},
    float_type::float,
    load::Load,
    spectrum::Spectrum,
    stacktrace::Stacktrace,
};

use super::{
    DeconvolutionVariant,
    convolution::convolve_by_points,
    types::{
        InitialValuesGeneric,
        sat_exp__dec_exp::InitialValues_SatExp_DecExp,
        sat_exp__dec_exp_plus_const::InitialValues_SatExp_DecExpPlusConst,
        sat_exp__two_dec_exp::InitialValues_SatExp_TwoDecExp,
        sat_exp__two_dec_exp__constrained_consts::InitialValues_SatExp_TwoDecExp_ConstrainedConsts,
        sat_exp__two_dec_exp__separate_consts::InitialValues_SatExp_TwoDecExp_SeparateConsts,
        sat_exp__two_dec_exp_plus_const::InitialValues_SatExp_TwoDecExpPlusConst,
        two__sat_exp__dec_exp::InitialValues_Two_SatExp_DecExp,
    },
};


pub type DeconvolutionResultOrError = FitResult;


#[derive(Debug, Clone, PartialEq)]
pub struct DeconvolutionData {
    pub instrument: Spectrum,
    pub measured: Spectrum,
    pub deconvolution: DeconvolutionVariant,
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

    pub fn deconvolve(
        &self,
        fit_algorithm: &FitAlgorithmVariant,
        initial_values_random_scale: Option<float>,
    ) -> DeconvolutionResultOrError {
        self.assert_steps_is_aligned();
        let initial_params = if let Some(initial_values_random_scale) = initial_values_random_scale {
            self.deconvolution.get_initial_values_randomized(initial_values_random_scale)
        } else {
            self.deconvolution.get_initial_values()
        };
        fit_algorithm.fit(&self, initial_params)
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
        let initial_params: Vec<float> = self.deconvolution.get_initial_values();
        assert_eq!(self.deconvolution.get_initial_values_len(), initial_params.len());
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

    #[deprecated]
    pub fn calc_chi_squared(&self, deconvolution_results: &Fit) -> float {
        const DEBUG: bool = false;
        let points_convolved = self.convolve_from_params(&deconvolution_results.params);
        let expected = &self.measured.points;
        let observed = points_convolved;
        assert_eq!(expected.len(), observed.len());
        todo!("normalize expected to sum == 1");
        if DEBUG {
            for (e, o) in expected.into_iter().zip(&observed) {
                println!("e={e}, o={o}");
            }
        }
        let n: float = expected.len() as float;
        let chi_squared: float = n * (
            expected
                .into_iter()
                .zip(observed)
                .map(|(&e, o)| if e != 0. { (o - e).powi(2) / e } else { 0. })
                .sum::<float>()
        );
        if DEBUG {
            dbg!(chi_squared);
            // panic!();
        }
        chi_squared
    }

    pub fn calc_reduced_chi_square(&self, deconvolution_results: &Fit) -> float {
        // src: https://www.originlab.com/doc/Quick-Help/measure-fitresult
        deconvolution_results.fit_residue / (self.deconvolution.get_initial_values_len() as float)
    }

    pub fn calc_r_square(&self, deconvolution_results: &Fit) -> float {
        // src: https://en.wikipedia.org/wiki/Coefficient_of_determination#Definitions
        let residual_sum_of_squares = deconvolution_results.fit_residue;
        let y_avg = self.measured.points.iter().sum::<float>() / (self.measured.points.len() as float);
        let total_sum_of_squares = self.measured.points.iter()
            .map(|y_i| (y_i - y_avg).powi(2))
            .sum::<float>();
        let r_square = 1. - residual_sum_of_squares / total_sum_of_squares;
        assert!(r_square >= 0.);
        assert!(r_square <= 1.);
        r_square
    }

    pub fn calc_adjusted_r_square(&self, deconvolution_results: &Fit) -> float {
        // src: https://en.wikipedia.org/wiki/Coefficient_of_determination#Adjusted_R2
        let r_square = self.calc_r_square(deconvolution_results);
        let n = self.measured.points.len() as float;
        let p = self.deconvolution.get_initial_values_len() as float;
        1. - (1. - r_square) * ( (n-1.) / (n-p-1.) )
    }

    pub fn write_result_to_file(
        &self,
        deconvolution_results: &Fit,
        filepathstr_output: &str,
        desmos_function_str: Result<String, &str>,
        origin_function_str: Result<String, &str>,
        fit_goodness_msg: &str,
        params: &Vec<float>,
    ) {
        let mut file_output = File::create(filepathstr_output).unwrap();
        writeln!(file_output, "name: {name}", name=self.deconvolution.get_name()).unwrap();
        // TODO(feat): "function: y(x) = ..."
        writeln!(file_output, "").unwrap();
        writeln!(file_output, "{fit_goodness_msg}").unwrap();
        writeln!(file_output, "").unwrap();
        writeln!(file_output, "params:").unwrap();
        // TODO(refactor): make this a method in corresponding types
        type DV = DeconvolutionVariant;
        match &self.deconvolution {
            DV::PerPoint(..) => {
                let sd_deconvolved = Spectrum {
                    points: deconvolution_results.params.clone(),
                    step: self.get_step(),
                    x_start: self.measured.x_start,
                };
                sd_deconvolved.write_to_file(filepathstr_output);
            }
            DV::Exponents(..) => {
                for (i, [amplitude, shift, tau]) in deconvolution_results.params.array_chunks().enumerate() {
                    writeln!(file_output, "- i={i}:").unwrap();
                    writeln!(file_output, "  - amplitude={amplitude}").unwrap();
                    writeln!(file_output, "  - shift={shift}").unwrap();
                    writeln!(file_output, "  - tau={tau}").unwrap();
                }
            }
            DV::SatExp_DecExp(..) => {
                type SelfF = InitialValues_SatExp_DecExp<float>;
                let SelfF { amplitude, shift, tau_a, tau_b } = SelfF::from_vec(params);
                writeln!(file_output, "- amplitude={amplitude}").unwrap();
                writeln!(file_output, "- shift={shift}").unwrap();
                writeln!(file_output, "- tau_a={tau_a}").unwrap();
                writeln!(file_output, "- tau_b={tau_b}").unwrap();
            }
            DV::Two_SatExp_DecExp(..) => {
                type SelfF = InitialValues_Two_SatExp_DecExp<float>;
                let SelfF { amplitude_1, shift_1, tau_a1, tau_b1, amplitude_2, shift_2, tau_a2, tau_b2 } = SelfF::from_vec(params);
                writeln!(file_output, "- amplitude_1={amplitude_1}").unwrap();
                writeln!(file_output, "- shift_1={shift_1}").unwrap();
                writeln!(file_output, "- tau_a1={tau_a1}").unwrap();
                writeln!(file_output, "- tau_b1={tau_b1}").unwrap();
                writeln!(file_output, "- amplitude_2={amplitude_2}").unwrap();
                writeln!(file_output, "- shift_2={shift_2}").unwrap();
                writeln!(file_output, "- tau_a2={tau_a2}").unwrap();
                writeln!(file_output, "- tau_b2={tau_b2}").unwrap();
            }
            DV::SatExp_DecExpPlusConst(..) => {
                type SelfF = InitialValues_SatExp_DecExpPlusConst<float>;
                let SelfF { amplitude, shift, height, tau_a, tau_b } = SelfF::from_vec(params);
                writeln!(file_output, "- amplitude={amplitude}").unwrap();
                writeln!(file_output, "- shift={shift}").unwrap();
                writeln!(file_output, "- height={height}").unwrap();
                writeln!(file_output, "- tau_a={tau_a}").unwrap();
                writeln!(file_output, "- tau_b={tau_b}").unwrap();
            }
            DV::SatExp_TwoDecExp(..) => {
                type SelfF = InitialValues_SatExp_TwoDecExp<float>;
                let SelfF { amplitude, shift, tau_a, tau_b, tau_c } = SelfF::from_vec(params);
                writeln!(file_output, "- amplitude={amplitude}").unwrap();
                writeln!(file_output, "- shift={shift}").unwrap();
                writeln!(file_output, "- tau_a={tau_a}").unwrap();
                writeln!(file_output, "- tau_b={tau_b}").unwrap();
                writeln!(file_output, "- tau_c={tau_c}").unwrap();
            }
            DV::SatExp_TwoDecExpPlusConst(..) => {
                type SelfF = InitialValues_SatExp_TwoDecExpPlusConst<float>;
                let SelfF { amplitude, shift, height, tau_a, tau_b, tau_c } = SelfF::from_vec(params);
                writeln!(file_output, "- amplitude={amplitude}").unwrap();
                writeln!(file_output, "- shift={shift}").unwrap();
                writeln!(file_output, "- height={height}").unwrap();
                writeln!(file_output, "- tau_a={tau_a}").unwrap();
                writeln!(file_output, "- tau_b={tau_b}").unwrap();
                writeln!(file_output, "- tau_c={tau_c}").unwrap();
            }
            DV::SatExp_TwoDecExp_SeparateConsts(..) => {
                type SelfF = InitialValues_SatExp_TwoDecExp_SeparateConsts<float>;
                let SelfF { amplitude_b, amplitude_c, shift, tau_a, tau_b, tau_c } = SelfF::from_vec(params);
                writeln!(file_output, "- amplitude_b={amplitude_b}").unwrap();
                writeln!(file_output, "- amplitude_c={amplitude_c}").unwrap();
                writeln!(file_output, "- shift={shift}").unwrap();
                writeln!(file_output, "- tau_a={tau_a}").unwrap();
                writeln!(file_output, "- tau_b={tau_b}").unwrap();
                writeln!(file_output, "- tau_c={tau_c}").unwrap();
            }
            DV::SatExp_TwoDecExp_ConstrainedConsts(..) => {
                type SelfF = InitialValues_SatExp_TwoDecExp_ConstrainedConsts<float>;
                let SelfF { amplitude_a, amplitude_b, shift, tau_a, tau_b, tau_c } = SelfF::from_vec(params);
                writeln!(file_output, "- amplitude_a={amplitude_a}").unwrap();
                writeln!(file_output, "- amplitude_b={amplitude_b}").unwrap();
                writeln!(file_output, "- shift={shift}").unwrap();
                writeln!(file_output, "- tau_a={tau_a}").unwrap();
                writeln!(file_output, "- tau_b={tau_b}").unwrap();
                writeln!(file_output, "- tau_c={tau_c}").unwrap();
            }
        }
        if let Ok(desmos_function_str) = desmos_function_str {
            writeln!(file_output, "").unwrap();
            writeln!(file_output, "desmos function:").unwrap();
            writeln!(file_output, "{desmos_function_str}").unwrap();
        }
        if let Ok(origin_function_str) = origin_function_str {
            writeln!(file_output, "").unwrap();
            writeln!(file_output, "origin function:").unwrap();
            writeln!(file_output, "{origin_function_str}").unwrap();
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
            _ => stacktrace.panic_unknown_type(align_steps_to_str, ["bigger", "smaller"])
        }
    }
}





#[cfg(test)]
mod deconvolution_data {
    use crate::{
        deconvolution::DeconvolutionVariant,
        spectrum::Spectrum,
        diff_function::DiffFunction,
    };
    use super::super::{
        deconvolution_data::{AlignStepsTo, DeconvolutionData},
        types::{
            ValueAndDomain,
            per_points::{InitialValues_PerPoint, PerPoint},
        },
    };
    mod align_steps_to_smaller {
        use super::*;
        #[test]
        fn align_i0_4_to_m0_2() {
            assert_eq!(
                DeconvolutionData {
                    instrument: Spectrum {
                        points: vec![0., 0., 0., 0.4999999999999999, 1., 0.5000000000000001, 0., 0., 0.],
                        step: 0.2,
                        x_start: 0.7,
                    },
                    measured: Spectrum {
                        points: vec![0., 0.1, 0.2, 0.3, 0.4, 0.3, 0.2, 0.1, 0.],
                        step: 0.2,
                        x_start: 0.3,
                    },
                    deconvolution: DeconvolutionVariant::PerPoint(PerPoint {
                        diff_function_type: DiffFunction::DySqr,
                        antispikes: None,
                        initial_vad: InitialValues_PerPoint::new(9, ValueAndDomain::free(0.)),
                    }),
                },
                DeconvolutionData {
                    instrument: Spectrum {
                        points: vec![0., 0., 1., 0., 0.,],
                        step: 0.4,
                        x_start: 0.7,
                    },
                    measured: Spectrum {
                        points: vec![0., 0.1, 0.2, 0.3, 0.4, 0.3, 0.2, 0.1, 0.],
                        step: 0.2,
                        x_start: 0.3,
                    },
                    deconvolution: DeconvolutionVariant::PerPoint(PerPoint {
                        diff_function_type: DiffFunction::DySqr,
                        antispikes: None,
                        initial_vad: InitialValues_PerPoint::new(9, ValueAndDomain::free(0.)),
                    }),
                }.aligned_steps_to(AlignStepsTo::Smaller)
            );
        }
        #[test]
        fn align_m0_4_to_i0_2() {
            assert_eq!(
                DeconvolutionData {
                    instrument: Spectrum {
                        points: vec![0., 0.1, 0.2, 0.3, 0.4, 0.3, 0.2, 0.1, 0.],
                        step: 0.2,
                        x_start: 0.5,
                    },
                    measured: Spectrum {
                        points: vec![0., 0., 0., 0.4999999999999999, 1., 0.5000000000000007, 0., 0., 0.],
                        step: 0.2,
                        x_start: 0.9,
                    },
                    deconvolution: DeconvolutionVariant::PerPoint(PerPoint {
                        diff_function_type: DiffFunction::DySqr,
                        antispikes: None,
                        initial_vad: InitialValues_PerPoint::new(9, ValueAndDomain::free(0.)),
                    }),
                },
                DeconvolutionData {
                    instrument: Spectrum {
                        points: vec![0., 0.1, 0.2, 0.3, 0.4, 0.3, 0.2, 0.1, 0.],
                        step: 0.2,
                        x_start: 0.5,
                    },
                    measured: Spectrum {
                        points: vec![0., 0., 1., 0., 0.,],
                        step: 0.4,
                        x_start: 0.9,
                    },
                    deconvolution: DeconvolutionVariant::PerPoint(PerPoint {
                        diff_function_type: DiffFunction::DySqr,
                        antispikes: None,
                        initial_vad: InitialValues_PerPoint::new(9, ValueAndDomain::free(0.)),
                    }),
                }.aligned_steps_to(AlignStepsTo::Smaller)
            );
        }
    }
    mod align_steps_to_bigger {
        use super::*;
        #[test]
        fn align_m0_2_to_i0_4() {
            assert_eq!(
                DeconvolutionData {
                    instrument: Spectrum {
                        points: vec![0., 0., 1., 0., 0.,],
                        step: 0.4,
                        x_start: 0.1,
                    },
                    measured: Spectrum {
                        points: vec![0., 0.2, 0.4, 0.2, 0.],
                        step: 0.4,
                        x_start: 0.5,
                    },
                    deconvolution: DeconvolutionVariant::PerPoint(PerPoint {
                        diff_function_type: DiffFunction::DySqr,
                        antispikes: None,
                        initial_vad: InitialValues_PerPoint::new(9, ValueAndDomain::free(0.)),
                    }),
                },
                DeconvolutionData {
                    instrument: Spectrum {
                        points: vec![0., 0., 1., 0., 0.,],
                        step: 0.4,
                        x_start: 0.1,
                    },
                    measured: Spectrum {
                        points: vec![0., 0.1, 0.2, 0.3, 0.4, 0.3, 0.2, 0.1, 0.],
                        step: 0.2,
                        x_start: 0.5,
                    },
                    deconvolution: DeconvolutionVariant::PerPoint(PerPoint {
                        diff_function_type: DiffFunction::DySqr,
                        antispikes: None,
                        initial_vad: InitialValues_PerPoint::new(9, ValueAndDomain::free(0.)),
                    }),
                }.aligned_steps_to(AlignStepsTo::Bigger)
            );
        }
        #[test]
        fn align_i0_2_to_m0_4() {
            assert_eq!(
                DeconvolutionData {
                    instrument: Spectrum {
                        points: vec![0., 0.2, 0.4, 0.2, 0.],
                        step: 0.4,
                        x_start: 0.5,
                    },
                    measured: Spectrum {
                        points: vec![0., 0., 1., 0., 0.,],
                        step: 0.4,
                        x_start: 0.9,
                    },
                    deconvolution: DeconvolutionVariant::PerPoint(PerPoint {
                        diff_function_type: DiffFunction::DySqr,
                        antispikes: None,
                        initial_vad: InitialValues_PerPoint::new(9, ValueAndDomain::free(0.)),
                    }),
                },
                DeconvolutionData {
                    instrument: Spectrum {
                        points: vec![0., 0.1, 0.2, 0.3, 0.4, 0.3, 0.2, 0.1, 0.],
                        step: 0.2,
                        x_start: 0.5,
                    },
                    measured: Spectrum {
                        points: vec![0., 0., 1., 0., 0.,],
                        step: 0.4,
                        x_start: 0.9,
                    },
                    deconvolution: DeconvolutionVariant::PerPoint(PerPoint {
                        diff_function_type: DiffFunction::DySqr,
                        antispikes: None,
                        initial_vad: InitialValues_PerPoint::new(9, ValueAndDomain::free(0.)),
                    }),
                }.aligned_steps_to(AlignStepsTo::Bigger)
            );
        }
    }
}

