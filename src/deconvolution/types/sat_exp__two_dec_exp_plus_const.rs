//! SatExp_TwoDecExpPlusConst

use toml::Value as TomlValue;

use crate::{
    aliases_method_to_function::exp,
    config::Load,
    diff_function::DiffFunction,
    extensions::ToStringWithSignificantDigits,
    float_type::float,
    utils_io::format_by_dollar_str,
};

use super::{InitialValuesGeneric, InitialValuesF, InitialValuesVAD, ValueAndDomain, DeconvolutionType, i_to_x};


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
    const FORMAT_FOR_ORIGIN: &'static str = todo!();

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

    fn load_from_self_toml_value(toml_value: &TomlValue) -> Self {
        let diff_function_type = DiffFunction::load_from_self_toml_value(
            toml_value
                .get("diff_function_type")
                .expect("deconvolution_function -> SatExp_TwoDecExpPlusConst: `diff_function_type` not found")
        );
        // let initial_values = toml_value
        //     .get("initial_values")
        //     .expect("deconvolution_function -> SatExp_TwoDecExpPlusConst: `initial_values` not found")
        //     .as_array()
        //     .expect("deconvolution_function -> SatExp_TwoDecExpPlusConst -> initial_values: can't parse as list")
        //     .iter()
        //     .enumerate()
        //     .map(|(i, initial_value)| {
        //         initial_value
        //             .as_float()
        //             .expect(&format!("deconvolution_function -> SatExp_TwoDecExpPlusConst -> initial_values[{i}]: can't parse as float"))
        //     })
        //     .collect::<Vec<_>>()//[..6]
        //     .try_into()
        //     .expect("deconvolution_function -> SatExp_TwoDecExpPlusConst -> initial_values: len != 6");
        let initial_vads = InitialValues_SatExp_TwoDecExpPlusConst::load_from_parent_toml_value(toml_value);
        Self {
            diff_function_type,
            initial_vads,
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InitialValues_SatExp_TwoDecExpPlusConst<T> {
    amplitude: T,
    shift: T,
    height: T,
    tau_a: T,
    tau_b: T,
    tau_c: T,
}

impl<T: Copy> InitialValuesGeneric<T> for InitialValues_SatExp_TwoDecExpPlusConst<T> {
    fn len_stat() -> usize {
        6
    }

    fn from_vec(params: &Vec<T>) -> Self {
        match params[..] {
            [amplitude, shift, height, tau_a, tau_b, tau_c] => Self { amplitude, shift, height, tau_a, tau_b, tau_c },
            _ => unreachable!()
        }   
    }

    fn to_vec(&self) -> Vec<T> {
        todo!()
    }

    fn params_to_points(&self, params: &Vec<float>, points_len: usize, x_start_end: (float, float)) -> Vec<float> {
        todo!();
        // let Self { amplitude, shift, height, tau_a, tau_b, tau_c } = Self::from_vec(params);
        // let mut points = Vec::<float>::with_capacity(points_len);
        // for i in 0..points_len {
        //     let x: float = i_to_x(i, points_len, x_start_end);
        //     let x_m_shift: float = x - shift;
        //     let y = amplitude * (1. - exp(-x_m_shift/tau_a)) * (exp(-x_m_shift/tau_b) + exp(-x_m_shift/tau_c) + height);
        //     points.push(y.max(0.));
        // }
        // points
    }
}

impl InitialValuesVAD for InitialValues_SatExp_TwoDecExpPlusConst<ValueAndDomain> {
    // fn is_params_ok(&self, params: &Vec<float>) -> bool {
    //     // let (amplitude, _, height, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4], params[5]);
    //     // amplitude >= 0. && height >= 0. && tau_a >= 0. && tau_b >= 0. && tau_c >= 0.
    //     todo!()
    // }
}

impl From<InitialValues_SatExp_TwoDecExpPlusConst<ValueAndDomain>> for InitialValues_SatExp_TwoDecExpPlusConst<float> {
    fn from(value: InitialValues_SatExp_TwoDecExpPlusConst<ValueAndDomain>) -> Self {
        Self::from_vec(&value.to_vec().iter().map(|v| v.value).collect())
    }
}


impl Load for InitialValues_SatExp_TwoDecExpPlusConst<ValueAndDomain> {
    const TOML_NAME: &'static str = "initial_values";
    fn load_from_self_toml_value(toml_value: &TomlValue) -> Self {
        todo!()
    }
}

