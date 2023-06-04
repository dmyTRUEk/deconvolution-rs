//! Deconvolution.

use crate::{
    aliases_method_to_function::exp,
    antispikes::Antispikes,
    diff_function::DiffFunction,
    exponent_function::ExponentFunction,
    extensions::ToStringWithSignificantDigits,
    float_type::float,
    output_params,
};


/// Deconvolution type and it's corresponding params.
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Deconvolution {
    /// aka Simple
    /// [y0, y1, y2, ...]
    PerPoint {
        diff_function_type: DiffFunction,
        antispikes: Option<Antispikes>,
        initial_value: float, // [y0, y1, y2, ...]
    },
    /// a1*exp(-(x-s1)/t1) + ...
    Exponents {
        diff_function_type: DiffFunction,
        exponents_amount: usize,
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
        initial_values: [float; 4], // a, s, ta, tb
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
    pub const SAT_DEC_EXP_AMPLITUDE_SCALE: float = 1. / 1.;

    pub fn get_name(&self) -> &'static str {
        match self {
            Deconvolution::PerPoint { .. } => todo!(),
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
                let (amplitude, _, tau_a, tau_b) = (params[0], params[1], params[2], params[3]);
                amplitude >= 0. && tau_a >= 0. && tau_b >= 0.
            }
            Deconvolution::Two_SatExp_DecExp { .. } => {
                let (amplitude_1, _, tau_a1, tau_b1) = (params[0], params[1], params[2], params[3]);
                let (amplitude_2, _, tau_a2, tau_b2) = (params[4], params[5], params[6], params[7]);
                amplitude_1 >= 0. && tau_a1 >= 0. && tau_b1 >= 0. &&
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
            Self::Exponents { exponents_amount, .. } => {
                assert_eq!(exponents_amount * 3, params.len());
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
                let (amplitude, shift, tau_a, tau_b) = (params[0], params[1], params[2], params[3]);
                for i in 0..points_len {
                    let x: float = i_to_x(i, points_len, (x_start, x_end));
                    let x_m_shift: float = x - shift;
                    // let y = if x >= shift
                    let y = if x_m_shift >= 0. {
                        // "optimization" (don't work): precalc `1/tau_a` & `1/tau_b`.
                        Self::SAT_DEC_EXP_AMPLITUDE_SCALE * amplitude * (1. - exp(-x_m_shift/tau_a)) * exp(-x_m_shift/tau_b)
                    } else {
                        0.
                    };
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
                    let y1 = if x_m_shift_1 >= 0. {
                        Self::SAT_DEC_EXP_AMPLITUDE_SCALE * amplitude_1 * (1. - exp(-(x_m_shift_1)/tau_a1)) * exp(-(x_m_shift_1)/tau_b1)
                    } else {
                        0.
                    };
                    let y2 = if x_m_shift_2 >= 0. {
                        Self::SAT_DEC_EXP_AMPLITUDE_SCALE * amplitude_2 * (1. - exp(-(x_m_shift_2)/tau_a2)) * exp(-(x_m_shift_2)/tau_b2)
                    } else {
                        0.
                    };
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
                    let y = if x_m_shift >= 0. {
                        amplitude * (1. - exp(-x_m_shift/tau_a)) * (exp(-x_m_shift/tau_b) + height)
                    } else {
                        0.
                    };
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
                    let y = if x_m_shift >= 0. {
                        amplitude * (1. - exp(-x_m_shift/tau_a)) * (exp(-x_m_shift/tau_b) + exp(-x_m_shift/tau_c))
                    } else {
                        0.
                    };
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
                    let y = if x_m_shift >= 0. {
                        amplitude * (1. - exp(-x_m_shift/tau_a)) * (exp(-x_m_shift/tau_b) + exp(-x_m_shift/tau_c) + height)
                    } else {
                        0.
                    };
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
                    let y = if x_m_shift >= 0. {
                        (1. - exp(-x_m_shift/tau_a)) * (b*exp(-x_m_shift/tau_b) + c*exp(-x_m_shift/tau_c))
                    } else {
                        0.
                    };
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

    pub fn to_desmos_function(&self, params: &Vec<float>) -> Result<String, &'static str> {
        use output_params::SIGNIFICANT_DIGITS as SD;
        match self {
            Deconvolution::PerPoint { .. } => Err("impossible to build this function"),
            Deconvolution::Exponents { exponents_amount, .. } => {
                Ok([
                    r"f_{",
                    &exponents_amount.to_string(),
                    r"}\left(x\right)=",
                    &params
                        .chunks(3).into_iter()
                        .map(|parts| {
                            let (amplitude, shift, tau) = (parts[0], parts[1], parts[2]);
                            let neg_shift = -shift;
                            assert_ne!(0., tau);
                            [
                                r"\left\{x",
                                if tau > 0. { ">" } else { "<" },
                                &format!("{}", neg_shift.to_string_with_significant_digits(SD)),
                                r":\frac{",
                                &format!("{}", amplitude.to_string_with_significant_digits(SD)),
                                r"}{",
                                &format!("{}", (1./ExponentFunction::AMPLITUDE_SCALE).to_string_with_significant_digits(SD)),
                                r"}e^{-\frac{x",
                                if neg_shift.is_sign_positive() { "+" } else { "" },
                                &format!("{}", neg_shift.to_string_with_significant_digits(SD)),
                                r"}{",
                                &format!("{}", tau.to_string_with_significant_digits(SD)),
                                r"}},0\right\}",
                            ].concat()
                        })
                        .reduce(|acc, el| format!("{acc}+{el}")).unwrap(),
                ].concat())
            }
            Deconvolution::SatExp_DecExp { .. } => {
                let (amplitude, shift, tau_a, tau_b) = (params[0], params[1], params[2], params[3]);
                let neg_shift = -shift;
                Ok([
                    // y=a\left(1-e^{-\frac{x-s}{t_{1}}}\right)\left(e^{-\frac{x-s-d}{t_{2}}}\right)\left\{x>s\right\}
                    r"y=\frac{",
                    &format!("{}", amplitude.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", (1./Deconvolution::SAT_DEC_EXP_AMPLITUDE_SCALE).to_string_with_significant_digits(SD)),
                    r"}\left(1-e^{-\frac{x",
                    if neg_shift.is_sign_positive() { "+" } else { "" },
                    &format!("{}", neg_shift.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", tau_a.to_string_with_significant_digits(SD)),
                    r"}}\right)\left(e^{-\frac{x",
                    if neg_shift.is_sign_positive() { "+" } else { "" },
                    &format!("{}", neg_shift.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", tau_b.to_string_with_significant_digits(SD)),
                    r"}}\right)\left\{x\ge",
                    &format!("{}", shift.to_string_with_significant_digits(SD)),
                    r"\right\}",
                ].concat())
            }
            Deconvolution::Two_SatExp_DecExp { .. } => {
                let (amplitude_1, shift_1, tau_a1, tau_b1) = (params[0], params[1], params[2], params[3]);
                let (amplitude_2, shift_2, tau_a2, tau_b2) = (params[4], params[5], params[6], params[7]);
                let neg_shift_1 = -shift_1;
                let neg_shift_2 = -shift_2;
                Ok([
                    r"y=\frac{",
                    &format!("{}", amplitude_1.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", (1./Deconvolution::SAT_DEC_EXP_AMPLITUDE_SCALE).to_string_with_significant_digits(SD)),
                    r"}\left(1-e^{-\frac{x",
                    if neg_shift_1.is_sign_positive() { "+" } else { "" },
                    &format!("{}", neg_shift_1.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", tau_a1.to_string_with_significant_digits(SD)),
                    r"}}\right)\left(e^{-\frac{x",
                    if neg_shift_1.is_sign_positive() { "+" } else { "" },
                    &format!("{}", neg_shift_1.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", tau_b1.to_string_with_significant_digits(SD)),
                    r"}}\right)\left\{x\ge",
                    &format!("{}", shift_1.to_string_with_significant_digits(SD)),
                    r"\right\}+\frac{",
                    &format!("{}", amplitude_2.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", (1./Deconvolution::SAT_DEC_EXP_AMPLITUDE_SCALE).to_string_with_significant_digits(SD)),
                    r"}\left(1-e^{-\frac{x",
                    if neg_shift_2.is_sign_positive() { "+" } else { "" },
                    &format!("{}", neg_shift_2.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", tau_a2.to_string_with_significant_digits(SD)),
                    r"}}\right)\left(e^{-\frac{x",
                    if neg_shift_2.is_sign_positive() { "+" } else { "" },
                    &format!("{}", neg_shift_2.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", tau_b2.to_string_with_significant_digits(SD)),
                    r"}}\right)\left\{x\ge",
                    &format!("{}", shift_2.to_string_with_significant_digits(SD)),
                    r"\right\}",
                ].concat())
            }
            Deconvolution::SatExp_DecExpPlusConst { .. } => {
                let (amplitude, shift, height, tau_a, tau_b) = (params[0], params[1], params[2], params[3], params[4]);
                let neg_shift = -shift;
                Ok([
                    r"y=",
                    &format!("{}", amplitude.to_string_with_significant_digits(SD)),
                    r"\left(1-e^{-\frac{x",
                    if neg_shift.is_sign_positive() { "+" } else { "" },
                    &format!("{}", neg_shift.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", tau_a.to_string_with_significant_digits(SD)),
                    r"}}\right)\left(e^{-\frac{x",
                    if neg_shift.is_sign_positive() { "+" } else { "" },
                    &format!("{}", neg_shift.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", tau_b.to_string_with_significant_digits(SD)),
                    r"}}+",
                    &format!("{}", height.to_string_with_significant_digits(SD)),
                    r"\right)\left\{x\ge",
                    &format!("{}", shift.to_string_with_significant_digits(SD)),
                    r"\right\}",
                ].concat())
            }
            Deconvolution::SatExp_TwoDecExp { .. } => {
                let (amplitude, shift, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4]);
                let neg_shift = -shift;
                Ok([
                    r"y=",
                    &format!("{}", amplitude.to_string_with_significant_digits(SD)),
                    r"\left(1-e^{-\frac{x",
                    if neg_shift.is_sign_positive() { "+" } else { "" },
                    &format!("{}", neg_shift.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", tau_a.to_string_with_significant_digits(SD)),
                    r"}}\right)\left(e^{-\frac{x",
                    if neg_shift.is_sign_positive() { "+" } else { "" },
                    &format!("{}", neg_shift.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", tau_b.to_string_with_significant_digits(SD)),
                    r"}}+e^{-\frac{x",
                    if neg_shift.is_sign_positive() { "+" } else { "" },
                    &format!("{}", neg_shift.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", tau_c.to_string_with_significant_digits(SD)),
                    r"}}\right)\left\{x\ge",
                    &format!("{}", shift.to_string_with_significant_digits(SD)),
                    r"\right\}",
                ].concat())
            }
            Deconvolution::SatExp_TwoDecExpPlusConst { .. } => {
                let (amplitude, shift, height, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4], params[5]);
                let neg_shift = -shift;
                Ok([
                    r"y=",
                    &format!("{}", amplitude.to_string_with_significant_digits(SD)),
                    r"\left(1-e^{-\frac{x",
                    if neg_shift.is_sign_positive() { "+" } else { "" },
                    &format!("{}", neg_shift.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", tau_a.to_string_with_significant_digits(SD)),
                    r"}}\right)\left(e^{-\frac{x",
                    if neg_shift.is_sign_positive() { "+" } else { "" },
                    &format!("{}", neg_shift.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", tau_b.to_string_with_significant_digits(SD)),
                    r"}}+e^{-\frac{x",
                    if neg_shift.is_sign_positive() { "+" } else { "" },
                    &format!("{}", neg_shift.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", tau_c.to_string_with_significant_digits(SD)),
                    r"}}",
                    if height.is_sign_positive() { "+" } else { "" },
                    &format!("{}", height.to_string_with_significant_digits(SD)),
                    r"\right)\left\{x\ge",
                    &format!("{}", shift.to_string_with_significant_digits(SD)),
                    r"\right\}",
                ].concat())
            }
            Deconvolution::SatExp_TwoDecExp_SeparateConsts { .. } => {
                let (b, c, shift, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4], params[5]);
                let neg_shift = -shift;
                Ok([
                    r"y=\left(1-",
                    // if a.is_sign_positive() { "-" } else { "+" },
                    // &format!("{}", a.abs().to_string_with_significant_digits(SD)),
                    r"e^{-\frac{x",
                    if neg_shift.is_sign_positive() { "+" } else { "" },
                    &format!("{}", neg_shift.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", tau_a.to_string_with_significant_digits(SD)),
                    r"}}\right)\left(",
                    &format!("{}", b.to_string_with_significant_digits(SD)),
                    r"e^{-\frac{x",
                    if neg_shift.is_sign_positive() { "+" } else { "" },
                    &format!("{}", neg_shift.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", tau_b.to_string_with_significant_digits(SD)),
                    r"}}",
                    if c.is_sign_positive() { "+" } else { "" },
                    &format!("{}", c.to_string_with_significant_digits(SD)),
                    r"e^{-\frac{x",
                    if neg_shift.is_sign_positive() { "+" } else { "" },
                    &format!("{}", neg_shift.to_string_with_significant_digits(SD)),
                    r"}{",
                    &format!("{}", tau_c.to_string_with_significant_digits(SD)),
                    r"}}\right)\left\{x\ge",
                    &format!("{}", shift.to_string_with_significant_digits(SD)),
                    r"\right\}",
                ].concat())
            }
            Deconvolution::Fourier {} => unimplemented!(),
        }
    }
}


