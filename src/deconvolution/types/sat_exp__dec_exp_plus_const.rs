//! SatExp_DecExpPlusConst

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


/// a * (1-exp(-(x-s)/ta)) * (exp(-(x-s)/tb) + h)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SatExp_DecExpPlusConst {
    pub diff_function_type: DiffFunction,
    pub initial_vads: InitialValues_SatExp_DecExpPlusConst<ValueAndDomain>,
    pub allow_tb_less_than_ta: AllowTbLessThanTa,
}

impl DeconvolutionType for SatExp_DecExpPlusConst {
    const NAME: &'static str = "saturated decaying exponential plus const";

    const FORMAT_FOR_DESMOS: &'static str = r"max(0,$a\left(1-e^{-\frac{x$pm$s}{$ta}}\right)\left(e^{-\frac{x$pm$s}{$tb}}+$h\right))";
    const FORMAT_FOR_ORIGIN: &'static str = r"max(0,$a*(1-exp(-(x$pm$s)/($ta)))*(exp(-(x$pm$s)/($tb))+$h))";

    fn to_plottable_function(&self, params: &Vec<float>, significant_digits: u8, format: &'static str) -> String {
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
        // let allow_tb_less_than_ta = toml_value
        //     .get("allow_tb_less_than_ta")
        //     .expect("deconvolution_function -> SatExp_DecExpPlusConst: `allow_tb_less_than_ta` not found")
        //     .as_bool()
        //     .expect("deconvolution_function -> SatExp_DecExpPlusConst -> allow_tb_less_than_ta: can't parse as boolean");
        todo!()
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

impl<T: Copy> InitialValuesGeneric<T> for InitialValues_SatExp_DecExpPlusConst<T> {
    const LEN: usize = 5;

    fn from_vec(params: &Vec<T>) -> Self {
        match params[..] {
            [amplitude, shift, height, tau_a, tau_b] => Self { amplitude, shift, height, tau_a, tau_b },
            _ => unreachable!()
        }
    }

    fn to_vec(&self) -> Vec<T> {
        todo!()
    }

    fn params_to_points(&self, params: &Vec<float>, points_len: usize, x_start_end: (float, float)) -> Vec<float> {
        todo!();
        // let Self { amplitude, shift, height, tau_a, tau_b } = Self::from_vec(params);
        // let mut points = Vec::<float>::with_capacity(points_len);
        // for i in 0..points_len {
        //     let x: float = i_to_x(i, points_len, x_start_end);
        //     let x_m_shift: float = x - shift;
        //     let y = amplitude * (1. - exp(-x_m_shift/tau_a)) * (exp(-x_m_shift/tau_b) + height);
        //     points.push(y.max(0.));
        // }
        // points
    }
}

impl InitialValuesVAD for InitialValues_SatExp_DecExpPlusConst<ValueAndDomain> {
    fn is_params_ok(&self, params: &Vec<float>) -> bool {
        // let (amplitude, _, height, tau_a, tau_b) = (params[0], params[1], params[2], params[3], params[4]);
        // amplitude >= 0. && height >= 0. && tau_a >= 0. && tau_b >= 0. && if *allow_tb_less_than_ta { true } else { tau_a < tau_b }
        todo!();
        // self.to_vec().iter()
        //     .zip(params)
        //     .all(|(d, &p)| d.contains(p))
        // && if self.allow_tb_less_than_ta { true } else { self.tau_a < self.tau_b }
    }

    fn randomize(&mut self, initial_values_random_scale: float) {
        todo!()
    }
}

impl From<InitialValues_SatExp_DecExpPlusConst<ValueAndDomain>> for InitialValues_SatExp_DecExpPlusConst<float> {
    fn from(value: InitialValues_SatExp_DecExpPlusConst<ValueAndDomain>) -> Self {
        Self::from_vec(&value.to_vec().iter().map(|v| v.value).collect())
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
            tau_a: try_get("ta"),
            tau_b: try_get("tb"),
        }
    }
}

