//! SatExp_DecExp

use std::collections::HashMap;

use toml::Value as TomlValue;

use crate::{
    aliases_method_to_function::exp,
    diff_function::DiffFunction,
    extensions::ToStringWithSignificantDigits,
    load::Load,
    stacktrace::Stacktrace,
    types::{float::float, named_wrappers::{Deconvolved, DeconvolvedV, Params, ParamsG, ParamsV}},
    utils_io::format_by_dollar_str,
};

use super::{DeconvolutionType, InitialValuesGeneric, InitialValuesVAD, ValueAndDomain, i_to_x};


// a * (1-exp(-(x-s)/ta)) * exp(-(x-s)/tb)
/// (1-exp(-(x-s)/ta)) * exp(-(x-s)/tb)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SatExp_DecExp {
    pub diff_function_type: DiffFunction,
    pub initial_vads: InitialValues_SatExp_DecExp<ValueAndDomain>,
}

impl DeconvolutionType for SatExp_DecExp {
    const NAME: &'static str = "saturated decaying exponential";

    const FORMAT_FOR_DESMOS: &'static str = r"max(0,$a\left(1-\exp\left(-\frac{x$pm$s}{$ta}\right)\right)\left(\exp\left(-\frac{x$pm$s}{$tb}\right)\right))";
    const FORMAT_FOR_ORIGIN: &'static str = r"max(0,$a*(1-exp(-(x$pm$s)/($ta)))*(exp(-(x$pm$s)/($tb))))";

    fn to_plottable_function(&self, params: &Params, significant_digits: u8, format: &'static str) -> String {
        let params = InitialValues_SatExp_DecExp::from_vec(params);
        let sd = significant_digits;
        format_by_dollar_str(
            format,
            vec![
                ("a", &params.amplitude.to_string_with_significant_digits(sd)),
                ("pm", if !params.shift.is_sign_positive() { "+" } else { "-" }),
                ("s", &params.shift.abs().to_string_with_significant_digits(sd)),
                ("ta", &params.tau_a.to_string_with_significant_digits(sd)),
                ("tb", &params.tau_b.to_string_with_significant_digits(sd)),
            ]
        )
    }
}

impl Load for SatExp_DecExp {
    const TOML_NAME: &'static str = stringify!(SatExp_DecExp);
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        Self {
            diff_function_type: DiffFunction::load_from_parent_handle_stacktrace(toml_value, stacktrace),
            initial_vads: InitialValues_SatExp_DecExp::load_from_parent_handle_stacktrace(toml_value, stacktrace),
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InitialValues_SatExp_DecExp<T> {
    // TODO: remove `pub`?
    pub amplitude: T,
    pub shift: T,
    pub tau_a: T,
    pub tau_b: T,
}

impl<T: Copy> InitialValuesGeneric<T> for InitialValues_SatExp_DecExp<T> {
    const LEN: usize = 4;

    fn from_vec(params: &ParamsG<T>) -> Self {
        match params.0[..] {
            [amplitude, shift, tau_a, tau_b] => Self { amplitude, shift, tau_a, tau_b },
            _ => unreachable!()
        }
    }

    fn to_vec(&self) -> ParamsG<T> {
        let Self { amplitude, shift, tau_a, tau_b } = *self;
        ParamsG::<T>(vec![amplitude, shift, tau_a, tau_b])
    }

    fn params_to_points(&self, params: &Params, points_len: usize, x_start_end: (float, float)) -> Deconvolved {
        type SelfF = InitialValues_SatExp_DecExp<float>;
        let SelfF { amplitude, shift, tau_a, tau_b } = SelfF::from_vec(params);
        let mut points = Vec::<float>::with_capacity(points_len);
        for i in 0..points_len {
            let x: float = i_to_x(i, points_len, x_start_end);
            let x_m_shift: float = x - shift;
            let y = amplitude * (1. - exp(-x_m_shift/tau_a)) * exp(-x_m_shift/tau_b);
            points.push(y.max(0.));
        }
        Deconvolved(points)
    }

    fn params_to_points_v(&self, params: &ParamsV, points_len: usize, x_start_end: (float, float)) -> DeconvolvedV {
        todo!()
    }
}

impl InitialValuesVAD for InitialValues_SatExp_DecExp<ValueAndDomain> {}

impl From<InitialValues_SatExp_DecExp<ValueAndDomain>> for InitialValues_SatExp_DecExp<float> {
    fn from(value: InitialValues_SatExp_DecExp<ValueAndDomain>) -> Self {
        Self::from_vec(&ParamsG::<float>(value.to_vec().0.iter().map(|v| v.value).collect::<Vec<float>>()))
    }
}


impl Load for InitialValues_SatExp_DecExp<ValueAndDomain> {
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
            amplitude: try_get("a"),
            shift: try_get("s"),
            tau_a: try_get("ta"),
            tau_b: try_get("tb"),
        }

    }
}

