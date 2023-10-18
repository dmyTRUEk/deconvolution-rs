//! Deconvolution.

use std::cmp::Ordering;

use toml::Value as TomlValue;

use crate::{
    aliases_method_to_function::exp,
    antispikes::Antispikes,
    config::Load,
    diff_function::DiffFunction,
    exponent_function::ExponentFunction,
    extensions::ToStringWithSignificantDigits,
    float_type::float,
    utils_io::format_by_dollar_str,
};


/// Deconvolution type and it's corresponding params.
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Deconvolution {
    /// [y0, y1, y2, …]
    PerPoint {
        diff_function_type: DiffFunction,
        antispikes: Option<Antispikes>,
        initial_value: float, // [y0, y1, y2, ...]
    },

    /// a1*exp(-(x-s1)/t1) + …
    Exponents {
        diff_function_type: DiffFunction,
        // initial_values: &'a [float],
        // initial_values: Vec<float>,
        // TODO(refactor): change to `&'static [float]`?
        // initial_values: [float; 3*deconvolution_params::EXPONENTS_AMOUNT],
        initial_values: Vec<float>, // ai, si, ti, ...
    },

    /// a * (1-exp(-(x-s)/ta)) * exp(-(x-s)/tb)
    SatExp_DecExp {
        diff_function_type: DiffFunction,
        // TODO(refactor): save initial params as struct with named fields, not stupid array.
        // initial_values: [float; 4], // a, s, ta, tb
        initial_values: [float; 3], // s, ta, tb
        // initial_values: { a: float, s: float, tau_a: float, tau_b: float },
    },

    /// a1 * (1-exp(-(x-s1)/ta1)) * exp(-(x-s1)/tb1) + a2 * (1-exp(-(x-s2)/ta2)) * exp(-(x-s2)/tb2)
    Two_SatExp_DecExp {
        diff_function_type: DiffFunction,
        initial_values: [float; 8], // a1, s1, ta1, tb1, a2, s2, ta2, tb2
    },

    /// a * (1-exp(-(x-s)/ta)) * (exp(-(x-s)/tb) + h)
    SatExp_DecExpPlusConst {
        diff_function_type: DiffFunction,
        // initial_values: [float; 4], // s, h, ta, tb
        initial_values: [float; 5], // a, s, h, ta, tb
        allow_tb_less_than_ta: bool,
    },

    /// a * (1-exp(-(x-s)/ta)) * (exp(-(x-s)/tb) + exp(-(x-s)/tc))
    SatExp_TwoDecExp {
        diff_function_type: DiffFunction,
        initial_values: [float; 5], // a, s, ta, tb, tc
    },

    /// a * (1-exp(-(x-s)/ta)) * (exp(-(x-s)/tb) + exp(-(x-s)/tc) + h)
    SatExp_TwoDecExpPlusConst {
        diff_function_type: DiffFunction,
        initial_values: [float; 6], // a, s, h, ta, tb, tc
    },

    /// (1-exp(-(x-s)/ta)) * (b*exp(-(x-s)/tb) + c*exp(-(x-s)/tc))
    SatExp_TwoDecExp_SeparateConsts {
        diff_function_type: DiffFunction,
        initial_values: [float; 6], // b, c, s, ta, tb, tc
    },

    Fourier {
        // unimplemented
    },
}

impl<'a> Deconvolution {
    pub fn get_name(&self) -> &'static str {
        match self {
            Deconvolution::PerPoint { .. } => "per point",
            Deconvolution::Exponents { .. } => "exponents",
            Deconvolution::SatExp_DecExp { .. } => "saturated decaying exponential",
            Deconvolution::Two_SatExp_DecExp { .. } => "two saturated decaying exponentials",
            Deconvolution::SatExp_DecExpPlusConst { .. } => "saturated decaying exponential plus const",
            Deconvolution::SatExp_TwoDecExp { .. } => "saturated exponential and two decaying exponentials",
            Deconvolution::SatExp_TwoDecExpPlusConst { .. } => "saturated exponential and two decaying exponentials plus const",
            Deconvolution::SatExp_TwoDecExp_SeparateConsts { .. } => "saturated exponential and two decaying exponentials with individual amplitudes",
            Deconvolution::Fourier {} => unimplemented!(),
        }
    }

    pub fn get_initial_values_len(&self, spectrum_measured_len: usize) -> usize {
        match self {
            Deconvolution::PerPoint { .. } => spectrum_measured_len,
            Deconvolution::Exponents { initial_values, .. } => initial_values.len(),
            Deconvolution::SatExp_DecExp { initial_values, .. } => initial_values.len(),
            Deconvolution::Two_SatExp_DecExp { initial_values, .. } => initial_values.len(),
            Deconvolution::SatExp_DecExpPlusConst { initial_values, .. } => initial_values.len(),
            Deconvolution::SatExp_TwoDecExp { initial_values, .. } => initial_values.len(),
            Deconvolution::SatExp_TwoDecExpPlusConst { initial_values, .. } => initial_values.len(),
            Deconvolution::SatExp_TwoDecExp_SeparateConsts { initial_values, .. } => initial_values.len(),
            Deconvolution::Fourier {} => unimplemented!(),
        }
    }

    pub fn get_initial_values(&self, spectrum_measured_len: usize) -> Vec<float> {
        match self {
            Deconvolution::PerPoint { initial_value, .. } => vec![*initial_value; self.get_initial_values_len(spectrum_measured_len)],
            Deconvolution::Exponents { initial_values, .. } => initial_values.to_vec(),
            Deconvolution::SatExp_DecExp { initial_values, .. } => initial_values.to_vec(),
            Deconvolution::Two_SatExp_DecExp { initial_values, .. } => initial_values.to_vec(),
            Deconvolution::SatExp_DecExpPlusConst { initial_values, .. } => initial_values.to_vec(),
            Deconvolution::SatExp_TwoDecExp { initial_values, .. } => initial_values.to_vec(),
            Deconvolution::SatExp_TwoDecExpPlusConst { initial_values, .. } => initial_values.to_vec(),
            Deconvolution::SatExp_TwoDecExp_SeparateConsts { initial_values, .. } => initial_values.to_vec(),
            Deconvolution::Fourier {} => unimplemented!(),
        }
    }

    pub fn is_params_ok(&self, params: &Vec<float>) -> bool {
        match self {
            Deconvolution::PerPoint { .. } => params.into_iter().all(|&x| x >= 0.),
            Deconvolution::Exponents { .. } => params.chunks(3).into_iter().all(|parts| {
                let (amplitude, _tau, _shift) = (parts[0], parts[1], parts[2]);
                amplitude >= 0.
            }),
            Deconvolution::SatExp_DecExp { .. } => {
                // let (amplitude, _, tau_a, tau_b) = (params[0], params[1], params[2], params[3]);
                let (_, tau_a, tau_b) = (params[0], params[1], params[2]);
                // amplitude >= 0. && tau_a >= 0. && tau_b >= 0.
                tau_a >= 0. && tau_b >= 0.
            }
            Deconvolution::Two_SatExp_DecExp { .. } => {
                let (amplitude_1, shift_1, tau_a1, tau_b1) = (params[0], params[1], params[2], params[3]);
                let (amplitude_2, shift_2, tau_a2, tau_b2) = (params[4], params[5], params[6], params[7]);
                amplitude_1 >= 0. && tau_a1 >= 0. && tau_b1 >= 0. &&
                shift_1 < shift_2 &&
                amplitude_2 >= 0. && tau_a2 >= 0. && tau_b2 >= 0.
            }
            Deconvolution::SatExp_DecExpPlusConst { allow_tb_less_than_ta, .. } => {
                let (amplitude, _, height, tau_a, tau_b) = (params[0], params[1], params[2], params[3], params[4]);
                amplitude >= 0. && height >= 0. && tau_a >= 0. && tau_b >= 0. && if *allow_tb_less_than_ta { true } else { tau_a < tau_b }
            }
            Deconvolution::SatExp_TwoDecExp { .. } => {
                let (amplitude, _, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4]);
                amplitude >= 0. && tau_a >= 0. && tau_b >= 0. && tau_c >= 0.
            }
            Deconvolution::SatExp_TwoDecExpPlusConst { .. } => {
                let (amplitude, _, height, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4], params[5]);
                amplitude >= 0. && height >= 0. && tau_a >= 0. && tau_b >= 0. && tau_c >= 0.
            }
            Deconvolution::SatExp_TwoDecExp_SeparateConsts { .. } => {
                let (b, c, _, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4], params[5]);
                b >= 0. && c >= 0. && tau_a >= 0. && tau_b >= 0. && tau_c >= 0.
            }
            Deconvolution::Fourier {} => unimplemented!(),
        }
    }

    pub fn params_to_points(
        &self,
        params: &Vec<float>,
        points_len: usize,
        (x_start, x_end): (float, float),
    ) -> Vec<float> {
        assert!(points_len > 1);
        assert!(x_start < x_end);

        // TODO(optimization): measure perf, if this is slowing the program
        fn i_to_x(
            i: usize,
            points_len: usize,
            (x_start, x_end): (float, float),
        ) -> float {
            let x_range: float = x_end - x_start;
            let t: float = (i as float) / ((points_len - 1) as float);
            let x: float = t * x_range + x_start;
            x
        }

        match self {
            Self::PerPoint { .. } => params.to_vec(),
            Self::Exponents { .. } => {
                assert_eq!(0, params.len() % 3);
                let exponents: Vec<ExponentFunction> = params
                    .chunks(3).into_iter()
                    .map(|parts| ExponentFunction { amplitude: parts[0], shift: parts[1], tau: parts[2] } )
                    .collect();
                let mut points = vec![0.; points_len];
                for i in 0..points_len {
                    let x: float = i_to_x(i, points_len, (x_start, x_end));
                    let sum: float = exponents.iter()
                        .map(|exponent| exponent.eval_at(x))
                        .sum();
                    points[i] = sum;
                }
                points
            }
            Self::SatExp_DecExp { .. } => {
                let mut points = vec![0.; points_len];
                // let (_amplitude, shift, tau_a, tau_b) = (params[0], params[1], params[2], params[3]);
                let (shift, tau_a, tau_b) = (params[0], params[1], params[2]);
                for i in 0..points_len {
                    let x: float = i_to_x(i, points_len, (x_start, x_end));
                    let x_m_shift: float = x - shift;
                    // let y = if x >= shift
                    // "optimization" (don't work): precalc `1/tau_a` & `1/tau_b`.
                    let y = ( (1. - exp(-x_m_shift/tau_a)) * exp(-x_m_shift/tau_b) ).max(0.);
                    points[i] = y;
                }
                points
            }
            Self::Two_SatExp_DecExp { .. } => {
                let mut points = vec![0.; points_len];
                let (amplitude_1, shift_1, tau_a1, tau_b1) = (params[0], params[1], params[2], params[3]);
                let (amplitude_2, shift_2, tau_a2, tau_b2) = (params[4], params[5], params[6], params[7]);
                for i in 0..points_len {
                    let x: float = i_to_x(i, points_len, (x_start, x_end));
                    let x_m_shift_1: float = x - shift_1;
                    let x_m_shift_2: float = x - shift_2;
                    let y1 = ( amplitude_1 * (1. - exp(-(x_m_shift_1)/tau_a1)) * exp(-(x_m_shift_1)/tau_b1) ).max(0.);
                    let y2 = ( amplitude_2 * (1. - exp(-(x_m_shift_2)/tau_a2)) * exp(-(x_m_shift_2)/tau_b2) ).max(0.);
                    points[i] = y1 + y2;
                }
                points
            }
            Self::SatExp_DecExpPlusConst { .. } => {
                let mut points = vec![0.; points_len];
                let (amplitude, shift, height, tau_a, tau_b) = (params[0], params[1], params[2], params[3], params[4]);
                for i in 0..points_len {
                    let x: float = i_to_x(i, points_len, (x_start, x_end));
                    let x_m_shift: float = x - shift;
                    let y = ( amplitude * (1. - exp(-x_m_shift/tau_a)) * (exp(-x_m_shift/tau_b) + height) ).max(0.);
                    points[i] = y;
                }
                points
            }
            Self::SatExp_TwoDecExp { .. } => {
                let mut points = vec![0.; points_len];
                let (amplitude, shift, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4]);
                for i in 0..points_len {
                    let x: float = i_to_x(i, points_len, (x_start, x_end));
                    let x_m_shift: float = x - shift;
                    let y = ( amplitude * (1. - exp(-x_m_shift/tau_a)) * (exp(-x_m_shift/tau_b) + exp(-x_m_shift/tau_c)) ).max(0.);
                    points[i] = y;
                }
                points
            }
            Self::SatExp_TwoDecExpPlusConst { .. } => {
                let mut points = vec![0.; points_len];
                let (amplitude, shift, height, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4], params[5]);
                for i in 0..points_len {
                    let x: float = i_to_x(i, points_len, (x_start, x_end));
                    let x_m_shift: float = x - shift;
                    let y = ( amplitude * (1. - exp(-x_m_shift/tau_a)) * (exp(-x_m_shift/tau_b) + exp(-x_m_shift/tau_c) + height) ).max(0.);
                    points[i] = y;
                }
                points
            }
            Self::SatExp_TwoDecExp_SeparateConsts { .. } => {
                let mut points = vec![0.; points_len];
                let (b, c, shift, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4], params[5]);
                for i in 0..points_len {
                    let x: float = i_to_x(i, points_len, (x_start, x_end));
                    let x_m_shift: float = x - shift;
                    let y = ( (1. - exp(-x_m_shift/tau_a)) * (b*exp(-x_m_shift/tau_b) + c*exp(-x_m_shift/tau_c)) ).max(0.);
                    points[i] = y;
                }
                points
            }
            Self::Fourier {} => unimplemented!(),
        }
    }

    pub fn calc_residue_function(&self, points_measured: &Vec<float>, points_convolved: &Vec<float>) -> float {
        match self {
            Deconvolution::PerPoint { diff_function_type, antispikes, .. } => {
                diff_function_type.calc_diff_with_antispikes(points_measured, points_convolved, antispikes)
            }
            Deconvolution::Exponents { diff_function_type, .. }
            | Deconvolution::SatExp_DecExp { diff_function_type, .. }
            | Deconvolution::Two_SatExp_DecExp { diff_function_type, .. }
            | Deconvolution::SatExp_DecExpPlusConst { diff_function_type, .. }
            | Deconvolution::SatExp_TwoDecExp { diff_function_type, .. }
            | Deconvolution::SatExp_TwoDecExpPlusConst { diff_function_type, .. }
            | Deconvolution::SatExp_TwoDecExp_SeparateConsts { diff_function_type, .. }
            => {
                diff_function_type.calc_diff(points_measured, points_convolved)
            }
            Deconvolution::Fourier {} => unimplemented!(),
        }
    }

    // TODO: tests, check if they are work in desmos
    pub fn to_desmos_function(&self, params: &Vec<float>, significant_digits: u8) -> Result<String, &'static str> {
        let sd = significant_digits as usize;
        if let Deconvolution::PerPoint { .. } = self { return Err("not plottable") };
        Ok(format!("y=") + &match self {
            Deconvolution::PerPoint { .. } => unreachable!(),
            Deconvolution::Exponents { .. } => {
                params
                    .chunks(3).into_iter()
                    .map(|parts| {
                        let (amplitude, shift, tau) = (parts[0], parts[1], parts[2]);
                        let neg_shift = -shift;
                        assert_ne!(0., tau);
                        format_by_dollar_str(
                            r"\left\{x$comp$ns:0,\frac{$a}{$sc}e^{-\frac{x$p$ns}{$t}}\right\}",
                            vec![
                                ("a", &amplitude.to_string_with_significant_digits(sd)),
                                ("comp", if tau > 0. { "<" } else { ">" }),
                                ("ns", &neg_shift.to_string_with_significant_digits(sd)),
                                ("p", if neg_shift.is_sign_positive() { "+" } else { "" }),
                                ("sc", &(1./ExponentFunction::AMPLITUDE_SCALE).to_string_with_significant_digits(sd)),
                                ("t", &tau.to_string_with_significant_digits(sd)),
                            ]
                        );
                        todo!("rewrite using `max(0,…)`");
                    })
                    .reduce(|acc, el| format!("{acc}+{el}")).unwrap()
            }
            Deconvolution::SatExp_DecExp { .. } => {
                // let (_amplitude, shift, tau_a, tau_b) = (params[0], params[1], params[2], params[3]);
                let (shift, tau_a, tau_b) = (params[0], params[1], params[2]);
                format_by_dollar_str(
                    r"max(0,\left(1-e^{-\frac{x$pm$s}{$ta}}\right)\left(e^{-\frac{x$pm$s}{$tb}}\right))",
                    vec![
                        ("pm", if !shift.is_sign_positive() { "+" } else { "-" }),
                        ("s", &format!("{}", shift.abs().to_string_with_significant_digits(sd))),
                        ("ta", &format!("{}", tau_a.to_string_with_significant_digits(sd))),
                        ("tb", &format!("{}", tau_b.to_string_with_significant_digits(sd))),
                    ]
                )
            }
            Deconvolution::Two_SatExp_DecExp { .. } => {
                let (amplitude_1, shift_1, tau_a1, tau_b1) = (params[0], params[1], params[2], params[3]);
                let (amplitude_2, shift_2, tau_a2, tau_b2) = (params[4], params[5], params[6], params[7]);
                format_by_dollar_str(
                    &[
                        r"max(0,$a1\left(1-e^{-\frac{x$pm1$s1}{$ta1}}\right)\left(e^{-\frac{x$pm1$s1}{$tb1}}\right))",
                        r"+",
                        r"max(0,$a2\left(1-e^{-\frac{x$pm2$s2}{$ta2}}\right)\left(e^{-\frac{x$pm2$s2}{$tb2}}\right))"
                    ].concat(),
                    vec![
                        ("a1", &amplitude_1.to_string_with_significant_digits(sd)),
                        ("pm1", if !shift_1.is_sign_positive() { "+" } else { "-" }),
                        ("s1", &shift_1.abs().to_string_with_significant_digits(sd)),
                        ("ta1", &tau_a1.to_string_with_significant_digits(sd)),
                        ("tb1", &tau_b1.to_string_with_significant_digits(sd)),

                        ("a2", &amplitude_2.to_string_with_significant_digits(sd)),
                        ("pm2", if !shift_2.is_sign_positive() { "+" } else { "-" }),
                        ("s2", &shift_2.abs().to_string_with_significant_digits(sd)),
                        ("ta2", &tau_a2.to_string_with_significant_digits(sd)),
                        ("tb2", &tau_b2.to_string_with_significant_digits(sd)),
                    ]
                )
            }
            Deconvolution::SatExp_DecExpPlusConst { .. } => {
                let (amplitude, shift, height, tau_a, tau_b) = (params[0], params[1], params[2], params[3], params[4]);
                format_by_dollar_str(
                    r"max(0,$a\left(1-e^{-\frac{x$pm$s}{$ta}}\right)\left(e^{-\frac{x$pm$s}{$tb}}+$h\right))",
                    vec![
                        ("a", &amplitude.to_string_with_significant_digits(sd)),
                        ("h", &height.to_string_with_significant_digits(sd)),
                        ("pm", if !shift.is_sign_positive() { "+" } else { "-" }),
                        ("s", &shift.abs().to_string_with_significant_digits(sd)),
                        ("ta", &tau_a.to_string_with_significant_digits(sd)),
                        ("tb", &tau_b.to_string_with_significant_digits(sd)),
                    ]
                )
            }
            Deconvolution::SatExp_TwoDecExp { .. } => {
                let (amplitude, shift, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4]);
                format_by_dollar_str(
                    r"max(0,$a\left(1-e^{-\frac{x$pm$s}{$ta}}\right)\left(e^{-\frac{x$pm$s}{$tb}}+e^{-\frac{x$pm$s}{$tc}}\right))",
                    vec![
                        ("a", &amplitude.to_string_with_significant_digits(sd)),
                        ("pm", if !shift.is_sign_positive() { "+" } else { "-" }),
                        ("s", &shift.abs().to_string_with_significant_digits(sd)),
                        ("ta", &tau_a.to_string_with_significant_digits(sd)),
                        ("tb", &tau_b.to_string_with_significant_digits(sd)),
                        ("tc", &tau_c.to_string_with_significant_digits(sd)),
                    ]
                )
            }
            Deconvolution::SatExp_TwoDecExpPlusConst { .. } => {
                let (amplitude, shift, height, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4], params[5]);
                format_by_dollar_str(
                    r"max(0,$a\left(1-e^{-\frac{x$pm$s}{$ta}}\right)\left(e^{-\frac{x$pm$s}{$tb}}+e^{-\frac{x$pm$s}{$tc}}$pmh$h\right))",
                    vec![
                        ("a", &amplitude.to_string_with_significant_digits(sd)),
                        ("h", &height.abs().to_string_with_significant_digits(sd)),
                        ("pm", if !shift.is_sign_positive() { "+" } else { "-" }),
                        ("pmh", if height.is_sign_positive() { "+" } else { "-" }),
                        ("s", &shift.abs().to_string_with_significant_digits(sd)),
                        ("ta", &tau_a.to_string_with_significant_digits(sd)),
                        ("tb", &tau_b.to_string_with_significant_digits(sd)),
                        ("tc", &tau_c.to_string_with_significant_digits(sd)),
                    ]
                )
            }
            Deconvolution::SatExp_TwoDecExp_SeparateConsts { .. } => {
                // let (a, b, c, shift, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4], params[5], params[6]);
                let (b, c, shift, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4], params[5]);
                format_by_dollar_str(
                    r"max(0,\left(1-e^{-\frac{x$pm$s}{$ta}}\right)\left($be^{-\frac{x$pm$s}{$tb}}$pmc$ce^{-\frac{x$pm$s}{$tc}}\right))",
                    vec![
                        ("b", &b.to_string_with_significant_digits(sd)),
                        ("c", &c.abs().to_string_with_significant_digits(sd)),
                        ("pm", if !shift.is_sign_positive() { "+" } else { "-" }),
                        ("pmc", if c.is_sign_positive() { "+" } else { "-" }),
                        ("s", &shift.abs().to_string_with_significant_digits(sd)),
                        ("ta", &tau_a.to_string_with_significant_digits(sd)),
                        ("tb", &tau_b.to_string_with_significant_digits(sd)),
                        ("tc", &tau_c.to_string_with_significant_digits(sd)),
                    ]
                )
            }
            Deconvolution::Fourier {} => unimplemented!(),
        })
    }
}


impl Load for Deconvolution {
    fn load_from_toml_value(toml_value: &TomlValue) -> Self {
        const DECONVOLUTION_FUNCTIONS_NAMES: [&'static str; 8] = [
            "PerPoint",
            "Exponents",
            "SatExp_DecExp",
            "Two_SatExp_DecExp",
            "SatExp_DecExpPlusConst",
            "SatExp_TwoDecExp",
            "SatExp_TwoDecExpPlusConst",
            "SatExp_TwoDecExp_SeparateConsts",
        ];
        let deconvolution_functions = DECONVOLUTION_FUNCTIONS_NAMES
            .map(|df_name| toml_value.get(df_name));
        let deconvolution_functions_number = deconvolution_functions
            .iter()
            .filter(|df| df.is_some())
            .count();
        match deconvolution_functions_number.cmp(&1) {
            Ordering::Less    => panic!("no known `deconvolution_function.<name>` found"),
            Ordering::Greater => panic!("too many `deconvolution_function.<name>` found"),
            Ordering::Equal   => {}
        }
        let deconvolution_function_index = deconvolution_functions
            .iter()
            .position(|df| df.is_some())
            .unwrap();
        let toml_value = deconvolution_functions[deconvolution_function_index].unwrap();
        // TODO(refactor)
        match deconvolution_function_index {
            0 => { // PerPoint
                let diff_function_type = DiffFunction::load_from_toml_value(
                    toml_value
                        .get("diff_function_type")
                        .expect("deconvolution_function -> PerPoint: `diff_function_type` not found")
                );
                let antispikes = toml_value
                    .get("antispikes")
                    .map(Antispikes::load_from_toml_value);
                let initial_value = toml_value
                    .get("initial_value")
                    .expect("deconvolution_function -> PerPoint: `initial_value` not found")
                    .as_float()
                    .expect("deconvolution_function -> PerPoint -> initial_value: can't parse as float");
                Self::PerPoint {
                    diff_function_type,
                    antispikes,
                    initial_value,
                }
            }
            1 => { // Exponents
                let diff_function_type = DiffFunction::load_from_toml_value(
                    toml_value
                        .get("diff_function_type")
                        .expect("deconvolution_function -> Exponents: `diff_function_type` not found")
                );
                let initial_values = toml_value
                    .get("initial_values")
                    .expect("deconvolution_function -> Exponents: `initial_values` not found")
                    .as_array()
                    .expect("deconvolution_function -> Exponents -> initial_values: can't parse as list")
                    .iter()
                    .enumerate()
                    .map(|(i, initial_value)| {
                        initial_value
                            .as_float()
                            .expect(&format!("deconvolution_function -> Exponents -> initial_values[{i}]: can't parse as float"))
                    })
                    .collect::<Vec<_>>();
                assert_eq!(0, initial_values.len() % 3);
                Self::Exponents {
                    diff_function_type,
                    initial_values,
                }
            }
            2 => { // SatExp_DecExp
                let diff_function_type = DiffFunction::load_from_toml_value(
                    toml_value
                        .get("diff_function_type")
                        .expect("deconvolution_function -> SatExp_DecExp: `diff_function_type` not found")
                );
                let initial_values = toml_value
                    .get("initial_values")
                    .expect("deconvolution_function -> SatExp_DecExp: `initial_values` not found")
                    .as_array()
                    .expect("deconvolution_function -> SatExp_DecExp -> initial_values: can't parse as list")
                    .iter()
                    .enumerate()
                    .map(|(i, initial_value)| {
                        initial_value
                            .as_float()
                            .expect(&format!("deconvolution_function -> SatExp_DecExp -> initial_values[{i}]: can't parse as float"))
                    })
                    .collect::<Vec<_>>()//[..3]
                    .try_into()
                    .expect("deconvolution_function -> SatExp_DecExp -> initial_values: len != 3");
                Self::SatExp_DecExp {
                    diff_function_type,
                    initial_values,
                }
            }
            3 => { // Two_SatExp_DecExp
                dbg!(toml_value);
                let diff_function_type = DiffFunction::load_from_toml_value(
                    toml_value
                        .get("diff_function_type")
                        .expect("deconvolution_function -> Two_SatExp_DecExp: `diff_function_type` not found")
                );
                let initial_values = toml_value
                    .get("initial_values")
                    .expect("deconvolution_function -> Two_SatExp_DecExp: `initial_values` not found")
                    .as_array()
                    .expect("deconvolution_function -> Two_SatExp_DecExp -> initial_values: can't parse as list")
                    .iter()
                    .enumerate()
                    .map(|(i, initial_value)| {
                        initial_value
                            .as_float()
                            .expect(&format!("deconvolution_function -> Two_SatExp_DecExp -> initial_values[{i}]: can't parse as float"))
                    })
                    .collect::<Vec<_>>()//[..8]
                    .try_into()
                    .expect("deconvolution_function -> Two_SatExp_DecExp -> initial_values: len != 8");
                Self::Two_SatExp_DecExp {
                    diff_function_type,
                    initial_values,
                }
            }
            4 => { // SatExp_DecExpPlusConst
                let diff_function_type = DiffFunction::load_from_toml_value(
                    toml_value
                        .get("diff_function_type")
                        .expect("deconvolution_function -> SatExp_DecExpPlusConst: `diff_function_type` not found")
                );
                let initial_values = toml_value
                    .get("initial_values")
                    .expect("deconvolution_function -> SatExp_DecExpPlusConst: `initial_values` not found")
                    .as_array()
                    .expect("deconvolution_function -> SatExp_DecExpPlusConst -> initial_values: can't parse as list")
                    .iter()
                    .enumerate()
                    .map(|(i, initial_value)| {
                        initial_value
                            .as_float()
                            .expect(&format!("deconvolution_function -> SatExp_DecExpPlusConst -> initial_values[{i}]: can't parse as float"))
                    })
                    .collect::<Vec<_>>()//[..5]
                    .try_into()
                    .expect("deconvolution_function -> SatExp_DecExpPlusConst -> initial_values: len != 5");
                let allow_tb_less_than_ta = toml_value
                    .get("allow_tb_less_than_ta")
                    .expect("deconvolution_function -> SatExp_DecExpPlusConst: `allow_tb_less_than_ta` not found")
                    .as_bool()
                    .expect("deconvolution_function -> SatExp_DecExpPlusConst -> allow_tb_less_than_ta: can't parse as boolean");
                Self::SatExp_DecExpPlusConst {
                    diff_function_type,
                    initial_values,
                    allow_tb_less_than_ta,
                }
            }
            5 => { // SatExp_TwoDecExp
                let diff_function_type = DiffFunction::load_from_toml_value(
                    toml_value
                        .get("diff_function_type")
                        .expect("deconvolution_function -> SatExp_TwoDecExp: `diff_function_type` not found")
                );
                let initial_values = toml_value
                    .get("initial_values")
                    .expect("deconvolution_function -> SatExp_TwoDecExp: `initial_values` not found")
                    .as_array()
                    .expect("deconvolution_function -> SatExp_TwoDecExp -> initial_values: can't parse as list")
                    .iter()
                    .enumerate()
                    .map(|(i, initial_value)| {
                        initial_value
                            .as_float()
                            .expect(&format!("deconvolution_function -> SatExp_TwoDecExp -> initial_values[{i}]: can't parse as float"))
                    })
                    .collect::<Vec<_>>()//[..5]
                    .try_into()
                    .expect("deconvolution_function -> SatExp_TwoDecExp -> initial_values: len != 5");
                Self::SatExp_TwoDecExp {
                    diff_function_type,
                    initial_values,
                }
            }
            6 => { // SatExp_TwoDecExpPlusConst
                let diff_function_type = DiffFunction::load_from_toml_value(
                    toml_value
                        .get("diff_function_type")
                        .expect("deconvolution_function -> SatExp_TwoDecExpPlusConst: `diff_function_type` not found")
                );
                let initial_values = toml_value
                    .get("initial_values")
                    .expect("deconvolution_function -> SatExp_TwoDecExpPlusConst: `initial_values` not found")
                    .as_array()
                    .expect("deconvolution_function -> SatExp_TwoDecExpPlusConst -> initial_values: can't parse as list")
                    .iter()
                    .enumerate()
                    .map(|(i, initial_value)| {
                        initial_value
                            .as_float()
                            .expect(&format!("deconvolution_function -> SatExp_TwoDecExpPlusConst -> initial_values[{i}]: can't parse as float"))
                    })
                    .collect::<Vec<_>>()//[..6]
                    .try_into()
                    .expect("deconvolution_function -> SatExp_TwoDecExpPlusConst -> initial_values: len != 6");
                Self::SatExp_TwoDecExpPlusConst {
                    diff_function_type,
                    initial_values,
                }
            }
            7 => { // SatExp_TwoDecExp_SeparateConsts
                let diff_function_type = DiffFunction::load_from_toml_value(
                    toml_value
                        .get("diff_function_type")
                        .expect("deconvolution_function -> SatExp_TwoDecExp_SeparateConsts: `diff_function_type` not found")
                );
                let initial_values = toml_value
                    .get("initial_values")
                    .expect("deconvolution_function -> SatExp_TwoDecExp_SeparateConsts: `initial_values` not found")
                    .as_array()
                    .expect("deconvolution_function -> SatExp_TwoDecExp_SeparateConsts -> initial_values: can't parse as list")
                    .iter()
                    .enumerate()
                    .map(|(i, initial_value)| {
                        initial_value
                            .as_float()
                            .expect(&format!("deconvolution_function -> SatExp_TwoDecExp_SeparateConsts -> initial_values[{i}]: can't parse as float"))
                    })
                    .collect::<Vec<_>>()//[..6]
                    .try_into()
                    .expect("deconvolution_function -> SatExp_TwoDecExp_SeparateConsts -> initial_values: len != 6");
                Self::SatExp_TwoDecExp_SeparateConsts {
                    diff_function_type,
                    initial_values,
                }
            }
            _ => unreachable!()
        }
    }
}


// Domain, Limits, Bounds
enum ValueDomain {
    /// -∞ < x < ∞ 
    Free,
    /// x == x0
    Fixed(float),
    /// x_min < x < x_max 
    Range(float, float),
}

