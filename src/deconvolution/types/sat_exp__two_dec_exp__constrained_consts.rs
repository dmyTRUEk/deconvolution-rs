//! SatExp_TwoDecExp_ConstrainedConsts

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


/// a * (1-exp(-(x-s)/ta)) * (b*exp(-(x-s)/tb) + (1-b)*exp(-(x-s)/tc))
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SatExp_TwoDecExp_ConstrainedConsts {
    pub diff_function_type: DiffFunction,
    pub initial_vads: InitialValues_SatExp_TwoDecExp_ConstrainedConsts<ValueAndDomain>,
}

impl DeconvolutionType for SatExp_TwoDecExp_ConstrainedConsts {
    const NAME: &'static str = "saturated exponential and two decaying exponentials with constrained amplitudes";

    const FORMAT_FOR_DESMOS: &'static str = r"max(0,$a\left(1-e^{-\frac{x$ssn$s}{$ta}}\right)\left($be^{-\frac{x$ssn$s}{$tb}}+(1$bsn$ba)e^{-\frac{x$ssn$s}{$tc}}\right))";
    const FORMAT_FOR_ORIGIN: &'static str = r"max(0,$a*(1-exp(-(x$ssn$s)/($ta)))*($b*exp(-(x$ssn$s)/($tb))+(1$bsn$ba)*exp(-(x$ssn$s)/($tc))))";

    fn to_plottable_function(&self, params: &Vec<float>, significant_digits: u8, format: &'static str) -> String {
        let v = InitialValues_SatExp_TwoDecExp_ConstrainedConsts::from_vec(params);
        let sd = significant_digits;
        format_by_dollar_str(
            format,
            vec![
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

impl Load for SatExp_TwoDecExp_ConstrainedConsts {
    const TOML_NAME: &'static str = stringify!(SatExp_TwoDecExp_ConstrainedConsts);
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        Self {
            diff_function_type: DiffFunction::load_from_parent_handle_stacktrace(toml_value, stacktrace),
            initial_vads: InitialValues_SatExp_TwoDecExp_ConstrainedConsts::load_from_parent_handle_stacktrace(toml_value, stacktrace),
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InitialValues_SatExp_TwoDecExp_ConstrainedConsts<T> {
    // TODO: remove `pub`?
    pub amplitude_a: T,
    pub amplitude_b: T,
    pub shift: T,
    pub tau_a: T,
    pub tau_b: T,
    pub tau_c: T,
}

impl<T: Copy> InitialValuesGeneric<T> for InitialValues_SatExp_TwoDecExp_ConstrainedConsts<T> {
    const LEN: usize = 6;

    fn from_vec(params: &Vec<T>) -> Self {
        match params[..] {
            [amplitude_a, amplitude_b, shift, tau_a, tau_b, tau_c] => Self { amplitude_a, amplitude_b, shift, tau_a, tau_b, tau_c },
            _ => unreachable!()
        }
    }

    fn to_vec(&self) -> Vec<T> {
        let Self { amplitude_a, amplitude_b, shift, tau_a, tau_b, tau_c } = *self;
        vec![amplitude_a, amplitude_b, shift, tau_a, tau_b, tau_c]
    }

    fn params_to_points(&self, params: &Vec<float>, points_len: usize, x_start_end: (float, float)) -> Vec<float> {
        type SelfF = InitialValues_SatExp_TwoDecExp_ConstrainedConsts<float>;
        let SelfF { amplitude_a, amplitude_b, shift, tau_a, tau_b, tau_c } = SelfF::from_vec(params);
        let mut points = Vec::<float>::with_capacity(points_len);
        for i in 0..points_len {
            let x: float = i_to_x(i, points_len, x_start_end);
            let x_m_shift: float = x - shift;
            let y = amplitude_a * (1. - exp(-x_m_shift/tau_a)) * (amplitude_b*exp(-x_m_shift/tau_b) + (1.-amplitude_b)*exp(-x_m_shift/tau_c));
            points.push(y.max(0.));
        }
        points
    }
}

impl InitialValuesVAD for InitialValues_SatExp_TwoDecExp_ConstrainedConsts<ValueAndDomain> {}

impl From<InitialValues_SatExp_TwoDecExp_ConstrainedConsts<ValueAndDomain>> for InitialValues_SatExp_TwoDecExp_ConstrainedConsts<float> {
    fn from(value: InitialValues_SatExp_TwoDecExp_ConstrainedConsts<ValueAndDomain>) -> Self {
        Self::from_vec(&value.to_vec().iter().map(|v| v.value).collect())
    }
}


impl Load for InitialValues_SatExp_TwoDecExp_ConstrainedConsts<ValueAndDomain> {
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
