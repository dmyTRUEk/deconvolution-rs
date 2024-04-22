//! SatExp_DecExpPlusConst

use std::collections::HashMap;

use toml::Value as TomlValue;

use crate::{
    aliases_method_to_function::exp,
    diff_function::DiffFunction,
    extensions::ToStringWithSignificantDigits,
    load::{LoadAutoImplFns, Load},
    stacktrace::Stacktrace,
    types::{float::float, linalg::DVect, named_wrappers::{DeconvolvedV, Params, ParamsG, ParamsV}},
    utils_io::format_by_dollar_str,
};

use super::super::initial_values::{InitialValuesGeneric, InitialValuesVAD};

use super::{Function, ValueAndDomain, i_to_x::i_to_x};


/// a * (1-exp(-(x-s)/ta)) * (exp(-(x-s)/tb) + h)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SatExp_DecExpPlusConst {
    pub diff_function_type: DiffFunction,
    pub initial_vads: InitialValues_SatExp_DecExpPlusConst<ValueAndDomain>,
    pub allow_tb_less_than_ta: AllowTbLessThanTa,
}

impl Function for SatExp_DecExpPlusConst {
    const NAME: &'static str = "saturated decaying exponential plus const";

    const FORMAT_FOR_DESMOS: &'static str = r"max(0,$a\left(1-\exp\left(-\frac{x$pm$s}{$ta}\right)\right)\left(\exp\left(-\frac{x$pm$s}{$tb}\right)+$h\right))";
    const FORMAT_FOR_ORIGIN: &'static str = r"max(0,$a*(1-exp(-(x$pm$s)/($ta)))*(exp(-(x$pm$s)/($tb))+$h))";

    fn to_plottable_function(&self, params: &Params, significant_digits: u8, format: &'static str) -> String {
        let values = InitialValues_SatExp_DecExpPlusConst::from_vec(params);
        let sd = significant_digits;
        format_by_dollar_str(
            format,
            vec![
                ("a", &values.amplitude.to_string_with_significant_digits(sd)),
                ("h", &values.height.to_string_with_significant_digits(sd)),
                ("pm", if !values.shift.is_sign_positive() { "+" } else { "-" }),
                ("s", &values.shift.abs().to_string_with_significant_digits(sd)),
                ("ta", &values.tau_a.to_string_with_significant_digits(sd)),
                ("tb", &values.tau_b.to_string_with_significant_digits(sd)),
            ]
        )
    }
}

impl Load for SatExp_DecExpPlusConst {
    const TOML_NAME: &'static str = stringify!(SatExp_DecExpPlusConst);
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        Self {
            diff_function_type: DiffFunction::load_from_parent_handle_stacktrace(toml_value, stacktrace),
            initial_vads: InitialValues_SatExp_DecExpPlusConst::load_from_parent_handle_stacktrace(toml_value, stacktrace),
            allow_tb_less_than_ta: AllowTbLessThanTa::load_from_parent_handle_stacktrace(toml_value, stacktrace),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AllowTbLessThanTa(bool);

impl Load for AllowTbLessThanTa {
    const TOML_NAME: &'static str = "allow_tb_less_than_ta";
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        Self (
            toml_value
                .as_bool()
                .unwrap_or_else(|| stacktrace.panic_cant_parse_as("boolean"))
        )
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InitialValues_SatExp_DecExpPlusConst<T> {
    // TODO: remove `pub`?
    pub amplitude: T, // or disable it?
    pub shift: T,
    pub height: T,
    pub tau_a: T,
    pub tau_b: T,
}

impl InitialValues_SatExp_DecExpPlusConst<float> {
    fn from_vec_vf(params: &ParamsV) -> Self {
        match params.0.as_slice()[..] {
            [      amplitude, shift, height, tau_a, tau_b ] =>
            Self { amplitude, shift, height, tau_a, tau_b },
            _ => unreachable!()
        }
    }
}

impl<T: Copy> InitialValuesGeneric<T> for InitialValues_SatExp_DecExpPlusConst<T> {
    const LEN: usize = 5;

    fn from_vec(params: &ParamsG<T>) -> Self {
        match params.0[..] {
            [      amplitude, shift, height, tau_a, tau_b ] =>
            Self { amplitude, shift, height, tau_a, tau_b },
            _ => unreachable!()
        }
    }

    fn to_vec(&self) -> ParamsG<T> {
        let Self {        amplitude, shift, height, tau_a, tau_b } = *self;
        ParamsG::<T>(vec![amplitude, shift, height, tau_a, tau_b])
    }

    fn params_to_points_v(&self, params: &ParamsV, points_len: usize, x_start_end: (float, float)) -> DeconvolvedV {
        type SelfF = InitialValues_SatExp_DecExpPlusConst<float>;
        let SelfF { amplitude, shift, height, tau_a, tau_b } = SelfF::from_vec_vf(params);
        let mut points = DVect::zeros(points_len);
        for i in 0..points_len {
            let x: float = i_to_x(i, points_len, x_start_end);
            let x_m_shift = x - shift;
            let y = amplitude * (1. - exp(-x_m_shift/tau_a)) * (exp(-x_m_shift/tau_b) + height);
            let y = y.max(0.);
            points[i] = y;
        }
        DeconvolvedV(points)
    }
}

impl InitialValuesVAD for InitialValues_SatExp_DecExpPlusConst<ValueAndDomain> {
    fn is_params_ok_v(&self, params: &ParamsV) -> bool {
        // self.to_vec().0.iter()
        //     .zip(&params.0)
        //     .all(|(vad, &value)| vad.contains(value))
        // && if self.allow_tb_less_than_ta { true } else { self.tau_a.value < self.tau_b.value }
        unimplemented!()
    }
}

impl From<InitialValues_SatExp_DecExpPlusConst<ValueAndDomain>> for InitialValues_SatExp_DecExpPlusConst<float> {
    fn from(value: InitialValues_SatExp_DecExpPlusConst<ValueAndDomain>) -> Self {
        Self::from_vec(&ParamsG::<float>(value.to_vec().0.iter().map(|v| v.value).collect::<Vec<float>>()))
    }
}


impl Load for InitialValues_SatExp_DecExpPlusConst<ValueAndDomain> {
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
            height: try_get("h"),
            tau_a: try_get("ta"),
            tau_b: try_get("tb"),
        }
    }
}

