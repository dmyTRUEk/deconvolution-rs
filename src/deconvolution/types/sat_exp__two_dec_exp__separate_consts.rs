//! SatExp_TwoDecExp_SeparateConsts

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

use super::{InitialValuesGeneric, InitialValuesF, InitialValuesVAD, ValueAndDomain, DeconvolutionType, i_to_x};


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
    const FORMAT_FOR_ORIGIN: &'static str = todo!();

    fn to_plottable_function(&self, params: &Vec<float>, significant_digits: u8, format: &'static str) -> String {
        let v = InitialValues_SatExp_TwoDecExp_SeparateConsts::from_vec(params);
        let sd = significant_digits;
        format_by_dollar_str(
            format,
            vec![
                ("b", &v.amplitude_b.to_string_with_significant_digits(sd)),
                ("c", &v.amplitude_c.abs().to_string_with_significant_digits(sd)),
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
    amplitude_b: T,
    amplitude_c: T,
    shift: T,
    tau_a: T,
    tau_b: T,
    tau_c: T,
}

impl<T: Copy> InitialValuesGeneric<T> for InitialValues_SatExp_TwoDecExp_SeparateConsts<T> {
    fn len_stat() -> usize {
        6
    }

    fn from_vec(params: &Vec<T>) -> Self {
        match params[..] {
            [amplitude_b, amplitude_c, shift, tau_a, tau_b, tau_c] => Self { amplitude_b, amplitude_c, shift, tau_a, tau_b, tau_c },
            _ => unreachable!()
        }
    }

    fn to_vec(&self) -> Vec<T> {
        let Self { amplitude_b, amplitude_c, shift, tau_a, tau_b, tau_c } = *self;
        vec![amplitude_b, amplitude_c, shift, tau_a, tau_b, tau_c]
    }

    fn params_to_points(&self, params: &Vec<float>, points_len: usize, x_start_end: (float, float)) -> Vec<float> {
        let InitialValues_SatExp_TwoDecExp_SeparateConsts { amplitude_b, amplitude_c, shift, tau_a, tau_b, tau_c } = InitialValues_SatExp_TwoDecExp_SeparateConsts::from_vec(params);
        let mut points = Vec::<float>::with_capacity(points_len);
        for i in 0..points_len {
            let x: float = i_to_x(i, points_len, x_start_end);
            let x_m_shift: float = x - shift;
            let y = (1. - exp(-x_m_shift/tau_a)) * (amplitude_b*exp(-x_m_shift/tau_b) + amplitude_c*exp(-x_m_shift/tau_c));
            points.push(y.max(0.));
        }
        points
    }
}

impl InitialValuesVAD for InitialValues_SatExp_TwoDecExp_SeparateConsts<ValueAndDomain> {
    fn is_params_ok(&self, params: &Vec<float>) -> bool {
        // let (b, c, _, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4], params[5]);
        // b >= 0. && c >= 0. && tau_a >= 0. && tau_b >= 0. && tau_c >= 0.
        todo!()
    }
}

impl From<InitialValues_SatExp_TwoDecExp_SeparateConsts<ValueAndDomain>> for InitialValues_SatExp_TwoDecExp_SeparateConsts<float> {
    fn from(value: InitialValues_SatExp_TwoDecExp_SeparateConsts<ValueAndDomain>) -> Self {
        Self::from_vec(&value.to_vec().iter().map(|v| v.value).collect())
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
            .map(|part| part.trim())
            .map(ValueAndDomain::load_from_str)
            .collect();
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
