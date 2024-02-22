//! SatExp_TwoDecExp_SeparateConsts

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


/// (1-exp(-(x-s)/ta)) * (b*exp(-(x-s)/tb) + c*exp(-(x-s)/tc))
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SatExp_TwoDecExp_SeparateConsts {
    pub diff_function_type: DiffFunction,
    pub initial_vads: InitialValues_SatExp_TwoDecExp_SeparateConsts<ValueAndDomain>,
}

impl DeconvolutionType for SatExp_TwoDecExp_SeparateConsts {
    const NAME: &'static str = "saturated exponential and two decaying exponentials with individual amplitudes";

    const FORMAT_FOR_DESMOS: &'static str = r"max(0,\left(1-e^{-\frac{x$pm$s}{$ta}}\right)\left($be^{-\frac{x$pm$s}{$tb}}$pmc$ce^{-\frac{x$pm$s}{$tc}}\right))";
    const FORMAT_FOR_ORIGIN: &'static str = r"max(0,(1-exp(-(x$pm$s)/($ta)))*($b*exp(-(x$pm$s)/($tb))$pmc$c*exp(-(x$pm$s)/($tc))))";

    fn to_plottable_function(&self, params: &Params, significant_digits: u8, format: &'static str) -> String {
        let v = InitialValues_SatExp_TwoDecExp_SeparateConsts::from_vec(params);
        let sd = significant_digits;
        format_by_dollar_str(
            format,
            vec![
                ("b", &v.amplitude_b.to_string_with_significant_digits(sd)),
                ("c", &v.amplitude_c.abs().to_string_with_significant_digits(sd)),
                // TODO(refactor): sign -> s => pmc -> csn, n = negative
                ("pm", if !v.shift.is_sign_positive() { "+" } else { "-" }),
                ("pmc", if v.amplitude_c.is_sign_positive() { "+" } else { "-" }),
                ("s", &v.shift.abs().to_string_with_significant_digits(sd)),
                ("ta", &v.tau_a.to_string_with_significant_digits(sd)),
                ("tb", &v.tau_b.to_string_with_significant_digits(sd)),
                ("tc", &v.tau_c.to_string_with_significant_digits(sd)),
            ]
        )
    }
}

impl Load for SatExp_TwoDecExp_SeparateConsts {
    const TOML_NAME: &'static str = stringify!(SatExp_TwoDecExp_SeparateConsts);
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        Self {
            diff_function_type: DiffFunction::load_from_parent_handle_stacktrace(toml_value, stacktrace),
            initial_vads: InitialValues_SatExp_TwoDecExp_SeparateConsts::load_from_parent_handle_stacktrace(toml_value, stacktrace),
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InitialValues_SatExp_TwoDecExp_SeparateConsts<T> {
    // TODO: remove `pub`?
    pub amplitude_b: T,
    pub amplitude_c: T,
    pub shift: T,
    pub tau_a: T,
    pub tau_b: T,
    pub tau_c: T,
}

impl<T: Copy> InitialValuesGeneric<T> for InitialValues_SatExp_TwoDecExp_SeparateConsts<T> {
    const LEN: usize = 6;

    fn from_vec(params: &ParamsG<T>) -> Self {
        match params.0[..] {
            [amplitude_b, amplitude_c, shift, tau_a, tau_b, tau_c] => Self { amplitude_b, amplitude_c, shift, tau_a, tau_b, tau_c },
            _ => unreachable!()
        }
    }

    fn to_vec(&self) -> ParamsG<T> {
        let Self { amplitude_b, amplitude_c, shift, tau_a, tau_b, tau_c } = *self;
        ParamsG::<T>(vec![amplitude_b, amplitude_c, shift, tau_a, tau_b, tau_c])
    }

    fn params_to_points(&self, params: &Params, points_len: usize, x_start_end: (float, float)) -> Deconvolved {
        type SelfF = InitialValues_SatExp_TwoDecExp_SeparateConsts<float>;
        let SelfF { amplitude_b, amplitude_c, shift, tau_a, tau_b, tau_c } = SelfF::from_vec(params);
        let mut points = Vec::<float>::with_capacity(points_len);
        for i in 0..points_len {
            let x: float = i_to_x(i, points_len, x_start_end);
            let x_m_shift: float = x - shift;
            let y = (1. - exp(-x_m_shift/tau_a)) * (amplitude_b*exp(-x_m_shift/tau_b) + amplitude_c*exp(-x_m_shift/tau_c));
            points.push(y.max(0.));
        }
        Deconvolved(points)
    }

    fn params_to_points_v(&self, params: &ParamsV, points_len: usize, x_start_end: (float, float)) -> DeconvolvedV {
        todo!()
    }
}

impl InitialValuesVAD for InitialValues_SatExp_TwoDecExp_SeparateConsts<ValueAndDomain> {}

impl From<InitialValues_SatExp_TwoDecExp_SeparateConsts<ValueAndDomain>> for InitialValues_SatExp_TwoDecExp_SeparateConsts<float> {
    fn from(value: InitialValues_SatExp_TwoDecExp_SeparateConsts<ValueAndDomain>) -> Self {
        Self::from_vec(&ParamsG::<float>(value.to_vec().0.iter().map(|v| v.value).collect::<Vec<float>>()))
    }
}


impl Load for InitialValues_SatExp_TwoDecExp_SeparateConsts<ValueAndDomain> {
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
            amplitude_b: try_get("b"),
            amplitude_c: try_get("c"),
            shift: try_get("s"),
            tau_a: try_get("ta"),
            tau_b: try_get("tb"),
            tau_c: try_get("tc"),
        }
    }
}

