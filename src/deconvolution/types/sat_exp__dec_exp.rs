//! SatExp_DecExp

use std::collections::HashMap;

use toml::Value as TomlValue;

use crate::{
    aliases_method_to_function::exp,
    diff_function::DiffFunction,
    extensions::ToStringWithSignificantDigits,
    float_type::float,
    load::Load,
    utils_io::format_by_dollar_str, stacktrace::Stacktrace,
};

use super::{InitialValuesGeneric, InitialValuesF, InitialValuesVAD, ValueAndDomain, DeconvolutionType, i_to_x};


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

    const FORMAT_FOR_DESMOS: &'static str = r"max(0,\left(1-e^{-\frac{x$pm$s}{$ta}}\right)\left(e^{-\frac{x$pm$s}{$tb}}\right))";
    const FORMAT_FOR_ORIGIN: &'static str = todo!();

    fn to_plottable_function(&self, params: &Vec<float>, significant_digits: u8, format: &'static str) -> String {
        let params = InitialValues_SatExp_DecExp::from_vec(params);
        let sd = significant_digits;
        format_by_dollar_str(
            format,
            vec![
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
    // amplitude: T,
    shift: T,
    tau_a: T,
    tau_b: T,
}

impl<T> InitialValues_SatExp_DecExp<T> {
    const LEN: usize = 3;
}

impl<T: Copy> InitialValuesGeneric<T> for InitialValues_SatExp_DecExp<T> {
    fn from_vec(params: &Vec<T>) -> Self {
        match params[..] {
            [shift, tau_a, tau_b] => Self { shift, tau_a, tau_b },
            _ => unreachable!()
        }
    }

    fn to_vec(&self) -> Vec<T> {
        let Self { shift, tau_a, tau_b } = *self;
        vec![shift, tau_a, tau_b]
    }

    fn len_stat() -> usize {
        Self::LEN
    }

    fn params_to_points(&self, params: &Vec<float>, points_len: usize, x_start_end: (float, float)) -> Vec<float> {
        type SelfG<T> = InitialValues_SatExp_DecExp<T>;
        let SelfG { shift, tau_a, tau_b } = SelfG::from_vec(params);
        let mut points = Vec::<float>::with_capacity(points_len);
        for i in 0..points_len {
            let x: float = i_to_x(i, points_len, x_start_end);
            let x_m_shift: float = x - shift;
            let y = (1. - exp(-x_m_shift/tau_a)) * exp(-x_m_shift/tau_b);
            points.push(y.max(0.));
        }
        points
    }
}

impl InitialValuesVAD for InitialValues_SatExp_DecExp<ValueAndDomain> {
    // fn is_params_ok(&self, params: &Vec<float>) -> bool {
    //     // let (amplitude, _, tau_a, tau_b) = (params[0], params[1], params[2], params[3]);
    //     // let (_, tau_a, tau_b) = (params[0], params[1], params[2]);
    //     // let Self::F { s, ta, tb } = Self::from_vec_v(params);
    //     // amplitude >= 0. && tau_a >= 0. && tau_b >= 0.
    //     // tau_a >= 0. && tau_b >= 0.
    //     self.to_vec().iter()
    //         .zip(params)
    //         .all(|(d, &p)| d.contains(p))
    // }

    // fn randomize(&mut self, initial_values_random_scale: float) {
    // }
}

impl InitialValuesF for InitialValues_SatExp_DecExp<float> {}

impl From<InitialValues_SatExp_DecExp<ValueAndDomain>> for InitialValues_SatExp_DecExp<float> {
    fn from(value: InitialValues_SatExp_DecExp<ValueAndDomain>) -> Self {
        Self::from_vec(&value.to_vec().iter().map(|v| v.value).collect())
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
            .map(|part| part.trim())
            .map(ValueAndDomain::load_from_str)
            .collect();
        let try_get = |name: &'static str| -> ValueAndDomain {
            *ivs
                .get(name)
                .unwrap_or_else(|| stacktrace.pushed(name).panic_not_found())
        };
        Self {
            // amplitude: try_get("a"),
            shift: try_get("s"),
            tau_a: try_get("ta"),
            tau_b: try_get("tb"),
        }

    }
}

