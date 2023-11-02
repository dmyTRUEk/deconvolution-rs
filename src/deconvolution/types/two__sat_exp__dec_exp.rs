//! Two_SatExp_DecExp

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


/// a1 * (1-exp(-(x-s1)/ta1)) * exp(-(x-s1)/tb1) + a2 * (1-exp(-(x-s2)/ta2)) * exp(-(x-s2)/tb2)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Two_SatExp_DecExp {
    pub diff_function_type: DiffFunction,
    pub initial_vads: InitialValues_Two_SatExp_DecExp<ValueAndDomain>,
}

impl DeconvolutionType for Two_SatExp_DecExp {
    const NAME: &'static str = "two saturated decaying exponentials";

    const FORMAT_FOR_DESMOS: &'static str = concat!(
        r"max(0,$a1\left(1-e^{-\frac{x$pm1$s1}{$ta1}}\right)\left(e^{-\frac{x$pm1$s1}{$tb1}}\right))",
        r"+",
        r"max(0,$a2\left(1-e^{-\frac{x$pm2$s2}{$ta2}}\right)\left(e^{-\frac{x$pm2$s2}{$tb2}}\right))",
    );
    const FORMAT_FOR_ORIGIN: &'static str = concat!(
        r"max(0,$a1*(1-exp(-(x$pm1$s1)/($ta1)))*exp(-(x$pm1$s1)/($tb1)))",
        r"+",
        r"max(0,$a2*(1-exp(-(x$pm2$s2)/($ta2)))*exp(-(x$pm2$s2)/($tb2)))",
    );

    fn to_plottable_function(&self, params: &Vec<float>, significant_digits: u8, format: &'static str) -> String {
        let values = InitialValues_Two_SatExp_DecExp::from_vec(params);
        let sd = significant_digits;
        format_by_dollar_str(
            format,
            vec![
                ("a1", &values.amplitude_1.to_string_with_significant_digits(sd)),
                ("pm1", if !values.shift_1.is_sign_positive() { "+" } else { "-" }),
                ("s1", &values.shift_1.abs().to_string_with_significant_digits(sd)),
                ("ta1", &values.tau_a1.to_string_with_significant_digits(sd)),
                ("tb1", &values.tau_b1.to_string_with_significant_digits(sd)),

                ("a2", &values.amplitude_2.to_string_with_significant_digits(sd)),
                ("pm2", if !values.shift_2.is_sign_positive() { "+" } else { "-" }),
                ("s2", &values.shift_2.abs().to_string_with_significant_digits(sd)),
                ("ta2", &values.tau_a2.to_string_with_significant_digits(sd)),
                ("tb2", &values.tau_b2.to_string_with_significant_digits(sd)),
            ]
        )
    }
}

impl Load for Two_SatExp_DecExp {
    const TOML_NAME: &'static str = stringify!(Two_SatExp_DecExp);
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        Self {
            diff_function_type: DiffFunction::load_from_parent_handle_stacktrace(toml_value, stacktrace),
            initial_vads: InitialValues_Two_SatExp_DecExp::load_from_parent_handle_stacktrace(toml_value, stacktrace),
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InitialValues_Two_SatExp_DecExp<T> {
    // TODO: remove `pub`?
    pub amplitude_1: T,
    pub shift_1: T,
    pub tau_a1: T,
    pub tau_b1: T,
    pub amplitude_2: T,
    pub shift_2: T,
    pub tau_a2: T,
    pub tau_b2: T,
}

impl<T: Copy> InitialValuesGeneric<T> for InitialValues_Two_SatExp_DecExp<T> {
    const LEN: usize = 8;

    fn from_vec(params: &Vec<T>) -> Self {
        match params[..] {
            [
                amplitude_1, shift_1, tau_a1, tau_b1,
                amplitude_2, shift_2, tau_a2, tau_b2,
            ] => InitialValues_Two_SatExp_DecExp::<T> {
                amplitude_1, shift_1, tau_a1, tau_b1,
                amplitude_2, shift_2, tau_a2, tau_b2,
            },
            _ => unreachable!()
        }
    }

    fn to_vec(&self) -> Vec<T> {
        let Self { amplitude_1, shift_1, tau_a1, tau_b1, amplitude_2, shift_2, tau_a2, tau_b2 } = *self;
        vec![amplitude_1, shift_1, tau_a1, tau_b1, amplitude_2, shift_2, tau_a2, tau_b2]
    }

    fn params_to_points(&self, params: &Vec<float>, points_len: usize, x_start_end: (float, float)) -> Vec<float> {
        type SelfF = InitialValues_Two_SatExp_DecExp<float>;
        let SelfF { amplitude_1, shift_1, tau_a1, tau_b1, amplitude_2, shift_2, tau_a2, tau_b2 } = SelfF::from_vec(params);
        let mut points = Vec::<float>::with_capacity(points_len);
        for i in 0..points_len {
            let x: float = i_to_x(i, points_len, x_start_end);
            let x_m_shift_1: float = x - shift_1;
            let x_m_shift_2: float = x - shift_2;
            let y1 = amplitude_1 * (1. - exp(-(x_m_shift_1)/tau_a1)) * exp(-(x_m_shift_1)/tau_b1);
            let y2 = amplitude_2 * (1. - exp(-(x_m_shift_2)/tau_a2)) * exp(-(x_m_shift_2)/tau_b2);
            points.push(y1.max(0.) + y2.max(0.));
        }
        points
    }
}

impl InitialValuesVAD for InitialValues_Two_SatExp_DecExp<ValueAndDomain> {}

impl From<InitialValues_Two_SatExp_DecExp<ValueAndDomain>> for InitialValues_Two_SatExp_DecExp<float> {
    fn from(value: InitialValues_Two_SatExp_DecExp<ValueAndDomain>) -> Self {
        Self::from_vec(&value.to_vec().iter().map(|v| v.value).collect())
    }
}


impl Load for InitialValues_Two_SatExp_DecExp<ValueAndDomain> {
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
            amplitude_1: try_get("a1"),
            shift_1: try_get("s1"),
            tau_a1: try_get("ta1"),
            tau_b1: try_get("tb1"),
            amplitude_2: try_get("a2"),
            shift_2: try_get("s2"),
            tau_a2: try_get("ta2"),
            tau_b2: try_get("tb2"),
        }
    }
}

