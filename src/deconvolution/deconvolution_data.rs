//! Deconvolution Data.

use std::{cmp::Ordering, fs::File, io::Write};

use toml::Value as TomlValue;

use crate::{
    fit_algorithms::{Fit, FitAlgorithmVariant, FitResult},
    load::Load,
    spectrum::Spectrum,
    stacktrace::Stacktrace,
    types::{float::float, named_wrappers::{ConvolvedV, DeconvolvedV, InstrumentRevV, MeasuredV, Params, ParamsG, ParamsV}},
};

use super::{
    DeconvolutionVariant,
    convolution::convolve_by_points_v,
    types::{
        InitialValuesGeneric,
        sat_exp__dec_exp::InitialValues_SatExp_DecExp,
        sat_exp__dec_exp_plus_const::InitialValues_SatExp_DecExpPlusConst,
        sat_exp__two_dec_exp::InitialValues_SatExp_TwoDecExp,
        sat_exp__two_dec_exp__constrained_consts::InitialValues_SatExp_TwoDecExp_ConstrainedConsts,
        sat_exp__two_dec_exp__separate_consts::InitialValues_SatExp_TwoDecExp_SeparateConsts,
        sat_exp__two_dec_exp_plus_const::InitialValues_SatExp_TwoDecExpPlusConst,
        sigmoid__two_dec_exp__constrained_consts::InitialValues_Sigmoid_TwoDecExp_ConstrainedConsts,
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

    pub fn assert_x_starts_is_aligned(&self) {
        assert_eq!(self.instrument.x_start, self.measured.x_start);
    }

    pub fn get_step(&self) -> float {
        self.assert_steps_is_aligned();
        self.instrument.step
    }

    #[expect(dead_code)]
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
            ParamsG::<float>(self.deconvolution.get_initial_values_randomized_v(initial_values_random_scale).0.data.as_vec().to_vec())
        } else {
            self.deconvolution.get_initial_values()
        };
        fit_algorithm.fit(&self, initial_params)
    }

    // pub fn calc_residue_function(&self, params: &Params) -> float {
    //     let points_convolved: Convolved = self.convolve_from_params(params);
    //     assert_eq!(self.get_params_amount(), params.0.len());
    //     self.deconvolution.calc_residue_function(&self.measured.points, points_convolved)
    // }

    /// Depending on the `self.deconvolution` `params` is:
    /// - PerPoint: list of values at that point,
    /// - Exponents: list of (amplitude, shift, tau),
    /// - SatExp_DecExp: amplitude, shift, tau_a, tau_b,
    /// for other look in [`Deconvolution`].
    pub fn calc_residue_function_v(&self, params: &ParamsV, instrument_rev: &InstrumentRevV, measured: &MeasuredV) -> float {
        let points_convolved: ConvolvedV = self.convolve_from_params_v(params, instrument_rev);
        assert_eq!(self.get_params_amount(), params.0.len());
        self.deconvolution.calc_residue_function_v(measured, points_convolved)
    }

    pub fn get_params_amount(&self) -> usize {
        self.deconvolution.get_initial_values_len(/*self.measured.points.len()*/)
    }

    pub fn get_initial_params(&self) -> Params {
        let initial_params: Params = self.deconvolution.get_initial_values();
        assert_eq!(self.deconvolution.get_initial_values_len(), initial_params.0.len());
        initial_params
    }

    // pub fn is_params_ok(&self, params: &Params) -> bool {
    //     self.deconvolution.is_params_ok(params)
    // }

    pub fn is_params_ok_v(&self, params: &ParamsV) -> bool {
        self.deconvolution.is_params_ok_v(params)
    }

    // pub fn convolve_from_params(&self, params: &Params) -> Convolved {
    //     // convert `params` into `points` ("deconvolved"):
    //     let points_deconvolved: Deconvolved = self.deconvolution.params_to_points(
    //         params,
    //         self.measured.points.len(),
    //         (self.measured.x_start, self.measured.get_x_end())
    //     );
    //     self.convolve_from_points(points_deconvolved)
    // }

    pub fn convolve_from_params_v(&self, params: &ParamsV, instrument_rev: &InstrumentRevV) -> ConvolvedV {
        // convert `params` into `points` ("deconvolved"):
        let points_deconvolved: DeconvolvedV = self.deconvolution.params_to_points_v(
            params,
            self.measured.points.len(),
            (self.measured.x_start, self.measured.get_x_end())
        );
        self.convolve_from_points_v(points_deconvolved, instrument_rev)
    }

    // pub fn convolve_from_points(&self, points_deconvolved: Deconvolved) -> Convolved {
    //     let points_convolved: Convolved = convolve_by_points(&self.instrument.points, points_deconvolved);
    //     assert_eq!(self.measured.points.len(), points_convolved.0.len());
    //     points_convolved
    // }

    pub fn convolve_from_points_v(&self, points_deconvolved: DeconvolvedV, instrument_rev: &InstrumentRevV) -> ConvolvedV {
        let points_convolved: ConvolvedV = convolve_by_points_v(instrument_rev, points_deconvolved);
        assert_eq!(self.measured.points.len(), points_convolved.0.len());
        points_convolved
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
        filepathstr_output: &str,
        fit_goodness_msg: &str,
        params: &Params,
        desmos_function_str: Result<String, &str>,
        origin_function_str: Result<String, &str>,
    ) {
        let mut file_output = File::create(filepathstr_output).unwrap();
        writeln!(file_output, "name: {name}", name=self.deconvolution.get_name()).unwrap();
        writeln!(file_output, "\n{fit_goodness_msg}").unwrap();
        writeln!(file_output, "\nparams:").unwrap();
        // TODO(refactor): make this a method in corresponding types
        type DV = DeconvolutionVariant;
        match &self.deconvolution {
            DV::PerPoint(..) => {
                let sd_deconvolved = Spectrum {
                    points: params.0.clone(),
                    step: self.get_step(),
                    x_start: self.measured.x_start,
                };
                sd_deconvolved.write_to_file(filepathstr_output);
            }
            DV::Exponents(..) => {
                for (i, [amplitude, shift, tau]) in params.0.array_chunks().enumerate() {
                    writeln!(file_output, "- i={i}:").unwrap();
                    writeln!(file_output, "  - amplitude={amplitude}").unwrap();
                    writeln!(file_output, "  - shift={shift}").unwrap();
                    writeln!(file_output, "  - tau={tau}").unwrap();
                }
            }
            DV::SatExp_DecExp(..) => {
                type SelfF = InitialValues_SatExp_DecExp<float>;
                let SelfF { amplitude, shift, tau_a, tau_b } = SelfF::from_vec(&params);
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
            DV::Sigmoid_TwoDecExp_ConstrainedConsts(..) => {
                type SelfF = InitialValues_Sigmoid_TwoDecExp_ConstrainedConsts<float>;
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
            writeln!(file_output, "\ndesmos function:\n{desmos_function_str}").unwrap();
        }
        if let Ok(origin_function_str) = origin_function_str {
            writeln!(file_output, "\norigin function:\n{origin_function_str}").unwrap();
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

