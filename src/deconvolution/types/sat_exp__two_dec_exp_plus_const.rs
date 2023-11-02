//! SatExp_TwoDecExpPlusConst

use std::collections::HashMap;

use toml::Value as TomlValue;

use crate::{
    aliases_method_to_function::exp,
    diff_function::DiffFunction,
    extensions::ToStringWithSignificantDigits,
    float_type::float,
    load::Load,
    stacktrace::Stacktrace,
    utils_io::format_by_dollar_str,
};

use super::{DeconvolutionType, InitialValuesGeneric, InitialValuesVAD, ValueAndDomain, i_to_x};


/// a * (1-exp(-(x-s)/ta)) * (exp(-(x-s)/tb) + exp(-(x-s)/tc) + h)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SatExp_TwoDecExpPlusConst {
    pub diff_function_type: DiffFunction,
    pub initial_vads: InitialValues_SatExp_TwoDecExpPlusConst<ValueAndDomain>,
}

impl DeconvolutionType for SatExp_TwoDecExpPlusConst {
    const NAME: &'static str = "saturated exponential and two decaying exponentials plus const";

    const FORMAT_FOR_DESMOS: &'static str = r"max(0,$a\left(1-e^{-\frac{x$pm$s}{$ta}}\right)\left(e^{-\frac{x$pm$s}{$tb}}+e^{-\frac{x$pm$s}{$tc}}$pmh$h\right))";
    const FORMAT_FOR_ORIGIN: &'static str = r"max(0,$a*(1-exp(-(x$pm$s)/($ta)))*(exp(-(x$pm$s)/($tb))+exp(-(x$pm$s)/($tc))$pmh$h))";

    fn to_plottable_function(&self, params: &Vec<float>, significant_digits: u8, format: &'static str) -> String {
        let params = InitialValues_SatExp_TwoDecExpPlusConst::from_vec(params);
        let sd = significant_digits;
        format_by_dollar_str(
            format,
            vec![
                ("a", &params.amplitude.to_string_with_significant_digits(sd)),
                ("h", &params.height.abs().to_string_with_significant_digits(sd)),
                ("pm", if !params.shift.is_sign_positive() { "+" } else { "-" }),
                ("pmh", if params.height.is_sign_positive() { "+" } else { "-" }),
                ("s", &params.shift.abs().to_string_with_significant_digits(sd)),
                ("ta", &params.tau_a.to_string_with_significant_digits(sd)),
                ("tb", &params.tau_b.to_string_with_significant_digits(sd)),
                ("tc", &params.tau_c.to_string_with_significant_digits(sd)),
            ]
        )
    }
}

impl Load for SatExp_TwoDecExpPlusConst {
    const TOML_NAME: &'static str = stringify!(SatExp_TwoDecExpPlusConst);
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        Self {
            diff_function_type: DiffFunction::load_from_parent_handle_stacktrace(toml_value, stacktrace),
            initial_vads: InitialValues_SatExp_TwoDecExpPlusConst::load_from_parent_handle_stacktrace(toml_value, stacktrace),
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InitialValues_SatExp_TwoDecExpPlusConst<T> {
    // TODO: remove `pub`?
    pub amplitude: T,
    pub shift: T,
    pub height: T,
    pub tau_a: T,
    pub tau_b: T,
    pub tau_c: T,
}

impl<T: Copy> InitialValuesGeneric<T> for InitialValues_SatExp_TwoDecExpPlusConst<T> {
    const LEN: usize = 6;

    fn from_vec(params: &Vec<T>) -> Self {
        match params[..] {
            [amplitude, shift, height, tau_a, tau_b, tau_c] => Self { amplitude, shift, height, tau_a, tau_b, tau_c },
            _ => unreachable!()
        }
    }

    fn to_vec(&self) -> Vec<T> {
        let Self { amplitude, shift, height, tau_a, tau_b, tau_c } = *self;
        vec![amplitude, shift, height, tau_a, tau_b, tau_c]
    }

    fn params_to_points(&self, params: &Vec<float>, points_len: usize, x_start_end: (float, float)) -> Vec<float> {
        type SelfF = InitialValues_SatExp_TwoDecExpPlusConst<float>;
        let SelfF { amplitude, shift, height, tau_a, tau_b, tau_c } = SelfF::from_vec(params);
        let mut points = Vec::<float>::with_capacity(points_len);
        for i in 0..points_len {
            let x: float = i_to_x(i, points_len, x_start_end);
            let x_m_shift: float = x - shift;
            let y = amplitude * (1. - exp(-x_m_shift/tau_a)) * (exp(-x_m_shift/tau_b) + exp(-x_m_shift/tau_c) + height);
            points.push(y.max(0.));
        }
        points
    }
}

impl InitialValuesVAD for InitialValues_SatExp_TwoDecExpPlusConst<ValueAndDomain> {}

impl From<InitialValues_SatExp_TwoDecExpPlusConst<ValueAndDomain>> for InitialValues_SatExp_TwoDecExpPlusConst<float> {
    fn from(value: InitialValues_SatExp_TwoDecExpPlusConst<ValueAndDomain>) -> Self {
        Self::from_vec(&value.to_vec().iter().map(|v| v.value).collect())
    }
}


impl Load for InitialValues_SatExp_TwoDecExpPlusConst<ValueAndDomain> {
    const TOML_NAME: &'static str = "initial_values";
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        let str = toml_value
            .as_str()
            .unwrap_or_else(|| stacktrace.panic_cant_parse_as("string"));
        let ivs: HashMap<String, ValueAndDomain> = str
            .trim_matches(|c: char| c.is_whitespace() || c == ',')
            .split(',')
            .map(|part| ValueAndDomain::load_from_str(part, stacktrace))
            .collect();
        let try_get = |name: &'static str| -> ValueAndDomain {
            *ivs
                .get(name)
                .unwrap_or_else(|| stacktrace.pushed(name).panic_not_found())
        };
        Self {
            amplitude: try_get("a"),
            shift: try_get("s"),
            height: try_get("h"),
            tau_a: try_get("a"),
            tau_b: try_get("b"),
            tau_c: try_get("c"),
        }
    }
}

