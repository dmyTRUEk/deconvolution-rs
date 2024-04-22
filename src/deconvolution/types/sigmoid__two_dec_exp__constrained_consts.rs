//! SatExp_TwoDecExp_ConstrainedConsts

use std::collections::HashMap;

use toml::Value as TomlValue;

use crate::{
    aliases_method_to_function::exp,
    diff_function::DiffFunction,
    extensions::ToStringWithSignificantDigits,
    load::{LoadAutoImplFns, Load},
    stacktrace::Stacktrace,
    types::{float::float, linalg::DVect, named_wrappers::{Deconvolved, DeconvolvedV, Params, ParamsG, ParamsV}},
    utils_io::format_by_dollar_str,
};

use super::{DeconvolutionType, InitialValuesGeneric, InitialValuesVAD, ValueAndDomain, i_to_x};


/// a / (1+exp(-(x-s)/ta)) * (b*exp(-(x-s)/tb) + (1-b)*exp(-(x-s)/tc))
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sigmoid_TwoDecExp_ConstrainedConsts {
    pub diff_function_type: DiffFunction,
    pub initial_vads: InitialValues_Sigmoid_TwoDecExp_ConstrainedConsts<ValueAndDomain>,
}

impl DeconvolutionType for Sigmoid_TwoDecExp_ConstrainedConsts {
    const NAME: &'static str = "sigmoid and two decaying exponentials with constrained amplitudes";

    const FORMAT_FOR_DESMOS: &'static str = r"\frac{$a}{1+\exp\left(-\frac{x$ssn$s}{$ta}\right)}\left($b\exp\left(-\frac{x$ssn$s}{$tb}\right)+(1$bsn$ba)\exp\left(-\frac{x$ssn$s}{$tc}\right)\right)";
    const FORMAT_FOR_ORIGIN: &'static str = r"$a/(1+exp(-(x$ssn$s)/($ta)))*($b*exp(-(x$ssn$s)/($tb))+(1$bsn$ba)*exp(-(x$ssn$s)/($tc)))";

    fn to_plottable_function(&self, params: &Params, significant_digits: u8, format: &'static str) -> String {
        let v = InitialValues_Sigmoid_TwoDecExp_ConstrainedConsts::from_vec(params);
        let sd = significant_digits;
        format_by_dollar_str(
            format,
            vec![
                ("a", &v.amplitude_a.to_string_with_significant_digits(sd)),
                ("b", &v.amplitude_b.to_string_with_significant_digits(sd)),
                ("ba", &v.amplitude_b.abs().to_string_with_significant_digits(sd)),
                ("bsn", if !v.amplitude_b.is_sign_positive() { "+" } else { "-" }),
                ("s", &v.shift.abs().to_string_with_significant_digits(sd)),
                ("ssn", if !v.shift.is_sign_positive() { "+" } else { "-" }),
                ("ta", &v.tau_a.to_string_with_significant_digits(sd)),
                ("tb", &v.tau_b.to_string_with_significant_digits(sd)),
                ("tc", &v.tau_c.to_string_with_significant_digits(sd)),
            ]
        )
    }
}

// TODO: write at least one test
impl Load for Sigmoid_TwoDecExp_ConstrainedConsts {
    const TOML_NAME: &'static str = stringify!(Sigmoid_TwoDecExp_ConstrainedConsts);
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        Self {
            diff_function_type: DiffFunction::load_from_parent_handle_stacktrace(toml_value, stacktrace),
            initial_vads: InitialValues_Sigmoid_TwoDecExp_ConstrainedConsts::load_from_parent_handle_stacktrace(toml_value, stacktrace),
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InitialValues_Sigmoid_TwoDecExp_ConstrainedConsts<T> {
    // TODO: remove `pub`?
    pub amplitude_a: T,
    pub amplitude_b: T,
    pub shift: T,
    pub tau_a: T,
    pub tau_b: T,
    pub tau_c: T,
}

impl InitialValues_Sigmoid_TwoDecExp_ConstrainedConsts<float> {
    fn from_vec_vf(params: &ParamsV) -> Self {
        // TODO(optimize)?
        match params.0.as_slice()[..] {
            [      amplitude_a, amplitude_b, shift, tau_a, tau_b, tau_c ] =>
            Self { amplitude_a, amplitude_b, shift, tau_a, tau_b, tau_c },
            _ => unreachable!()
        }
        // Self {
        //     amplitude_a: params[0],
        //     amplitude_b: params[1],
        //     shift: params[2],
        //     tau_a: params[3],
        //     tau_b: params[4],
        //     tau_c: params[5],
        // }
    }
}

impl<T: Copy> InitialValuesGeneric<T> for InitialValues_Sigmoid_TwoDecExp_ConstrainedConsts<T> {
    const LEN: usize = 6;

    fn from_vec(params: &ParamsG<T>) -> Self {
        match params.0[..] {
            [      amplitude_a, amplitude_b, shift, tau_a, tau_b, tau_c ] =>
            Self { amplitude_a, amplitude_b, shift, tau_a, tau_b, tau_c },
            _ => unreachable!()
        }
    }

    fn to_vec(&self) -> ParamsG<T> {
        let Self {        amplitude_a, amplitude_b, shift, tau_a, tau_b, tau_c } = *self;
        ParamsG::<T>(vec![amplitude_a, amplitude_b, shift, tau_a, tau_b, tau_c])
    }

    fn params_to_points(&self, params: &Params, points_len: usize, x_start_end: (float, float)) -> Deconvolved {
        type SelfF = InitialValues_Sigmoid_TwoDecExp_ConstrainedConsts<float>;
        let SelfF { amplitude_a, amplitude_b, shift, tau_a, tau_b, tau_c } = SelfF::from_vec(params);
        // TODO(optimization)?: fill by zeros and use index instead of push, or even use `fill_with`?
        let mut points = Vec::<float>::with_capacity(points_len);
        for i in 0..points_len {
            let x: float = i_to_x(i, points_len, x_start_end);
            let x_m_shift = x - shift;
            let y = amplitude_a / (1. + exp(-x_m_shift/tau_a)) * (amplitude_b*exp(-x_m_shift/tau_b) + (1.-amplitude_b)*exp(-x_m_shift/tau_c));
            points.push(y);
        }
        Deconvolved(points)
    }

    fn params_to_points_v(&self, params: &ParamsV, points_len: usize, x_start_end: (float, float)) -> DeconvolvedV {
        type SelfF = InitialValues_Sigmoid_TwoDecExp_ConstrainedConsts<float>;
        let SelfF { amplitude_a, amplitude_b, shift, tau_a, tau_b, tau_c } = SelfF::from_vec_vf(params);
        // TODO(optimization)?: use `from_fn`.
        let mut points = DVect::zeros(points_len);
        for i in 0..points_len {
            let x: float = i_to_x(i, points_len, x_start_end);
            let x_m_shift = x - shift;
            let y = amplitude_a / (1. + exp(-x_m_shift/tau_a)) * (amplitude_b*exp(-x_m_shift/tau_b) + (1.-amplitude_b)*exp(-x_m_shift/tau_c));
            points[i] = y;
        }
        DeconvolvedV(points)
    }
}

impl InitialValuesVAD for InitialValues_Sigmoid_TwoDecExp_ConstrainedConsts<ValueAndDomain> {}

impl From<InitialValues_Sigmoid_TwoDecExp_ConstrainedConsts<ValueAndDomain>> for InitialValues_Sigmoid_TwoDecExp_ConstrainedConsts<float> {
    fn from(value: InitialValues_Sigmoid_TwoDecExp_ConstrainedConsts<ValueAndDomain>) -> Self {
        Self::from_vec(&ParamsG::<float>(value.to_vec().0.iter().map(|v| v.value).collect::<Vec<float>>()))
    }
}


impl Load for InitialValues_Sigmoid_TwoDecExp_ConstrainedConsts<ValueAndDomain> {
    const TOML_NAME: &'static str = "initial_values";
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        let str = toml_value
            .as_str()
            .unwrap_or_else(|| stacktrace.panic_cant_parse_as("string"));
        let ivs: HashMap<String, ValueAndDomain> = str
            .trim_matches(|c: char| c.is_whitespace() || c == ',')
            .split(',')
            // TODO: add index to stacktrace
            .map(|part| ValueAndDomain::load_from_str(part, stacktrace))
            .collect();
        // TODO: assert `ivs.len` == Self::LEN
        let try_get = |name: &'static str| -> ValueAndDomain {
            *ivs
                .get(name)
                .unwrap_or_else(|| stacktrace.pushed(name).panic_not_found())
        };
        Self {
            amplitude_a: try_get("a"),
            amplitude_b: try_get("b"),
            shift: try_get("s"),
            tau_a: try_get("ta"),
            tau_b: try_get("tb"),
            tau_c: try_get("tc"),
        }
    }
}

