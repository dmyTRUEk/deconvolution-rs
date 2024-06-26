//! Exponents

use toml::Value as TomlValue;

use crate::{
    aliases_method_to_function::exp,
    diff_function::DiffFunction,
    extensions::ToStringWithSignificantDigits,
    load::{LoadAutoImplFns, Load},
    stacktrace::Stacktrace,
    types::{float::float, named_wrappers::{DeconvolvedV, Params, ParamsG, ParamsV}},
    utils_io::format_by_dollar_str,
};

use super::super::initial_values::{InitialValuesGeneric, InitialValuesVAD};

use super::{Function, ValueAndDomain};


/// a1*exp(-(x-s1)/t1) + …
#[derive(Debug, Clone, PartialEq)]
pub struct Exponents {
    pub diff_function_type: DiffFunction,
    pub initial_vads: InitialValues_Exponents<ValueAndDomain>,
}

impl Function for Exponents {
    const NAME: &'static str = "exponents";

    const FORMAT_FOR_DESMOS: &'static str = r"max\left(0,x$comp$ns:0,$a\exp\left(-\frac{x$p$ns}{$t}\right)\right)";
    const FORMAT_FOR_ORIGIN: &'static str = r"max($a*exp(-(x$p$ns)/($t)))";

    fn to_plottable_function(&self, params: &Params, significant_digits: u8, format: &'static str) -> String {
        let sd = significant_digits;
        params.0
            .chunks(3)
            .into_iter()
            .map(|parts| {
                let params = ExponentFunction::from_slice(parts);
                let neg_shift = -params.shift;
                assert_ne!(0., params.tau);
                format_by_dollar_str(
                    format,
                    vec![
                        ("a", &params.amplitude.to_string_with_significant_digits(sd)),
                        ("comp", if params.tau > 0. { "<" } else { ">" }),
                        ("ns", &neg_shift.to_string_with_significant_digits(sd)),
                        ("p", if neg_shift.is_sign_positive() { "+" } else { "" }),
                        ("t", &params.tau.to_string_with_significant_digits(sd)),
                    ]
                );
                todo!("rewrite using `max(0,…)`");
            })
            .reduce(|acc, el| format!("{acc}+{el}"))
            .unwrap()
    }
}

impl Load for Exponents {
    const TOML_NAME: &'static str = stringify!(Exponents);
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        Self {
            diff_function_type: DiffFunction::load_from_parent_handle_stacktrace(toml_value, stacktrace),
            initial_vads: InitialValues_Exponents::load_from_parent_handle_stacktrace(toml_value, stacktrace),
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub struct InitialValues_Exponents<T> (
    // TODO: Vec<(T, T, T)>
    Vec<T>
);

impl<T> InitialValuesGeneric<T> for InitialValues_Exponents<T> {
    const LEN: usize = unreachable!();

    fn len(&self) -> usize {
        self.0.len()
    }

    fn from_vec(params: &ParamsG<T>) -> Self {
        todo!()
    }

    fn to_vec(&self) -> ParamsG<T> {
        todo!()
    }

    fn params_to_points_v(&self, params: &ParamsV, points_len: usize, x_start_end: (float, float)) -> DeconvolvedV {
        assert_eq!(0, params.0.len() % 3);
        unimplemented!()
        // let exponents: Vec<ExponentFunction> = params.0
        //     .chunks(3)
        //     .into_iter()
        //     .map(|parts| ExponentFunction::from_slice(parts))
        //     .collect();
        // let mut points = Vec::<float>::with_capacity(points_len);
        // for i in 0..points_len {
        //     let x: float = i_to_x(i, points_len, (x_start, x_end));
        //     let sum: float = exponents.iter()
        //         .map(|exponent| exponent.eval_at(x))
        //         .sum();
        //     points.push(sum);
        // }
        // Deconvolved(points)
    }
}

impl InitialValuesVAD for InitialValues_Exponents<ValueAndDomain> {}

impl From<InitialValues_Exponents<ValueAndDomain>> for InitialValues_Exponents<float> {
    fn from(value: InitialValues_Exponents<ValueAndDomain>) -> Self {
        Self::from_vec(&ParamsG::<float>(value.to_vec().0.iter().map(|v| v.value).collect::<Vec<float>>()))
    }
}


impl Load for InitialValues_Exponents<ValueAndDomain> {
    const TOML_NAME: &'static str = "initial_values";
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        // let initial_values = toml_value
        //     .get("initial_values")
        //     .expect("deconvolution_function -> Exponents: `initial_values` not found")
        //     .as_array()
        //     .expect("deconvolution_function -> Exponents -> initial_values: can't parse as list")
        //     .iter()
        //     .enumerate()
        //     .map(|(i, initial_value)| {
        //         initial_value
        //             .as_float()
        //             .expect(&format!("deconvolution_function -> Exponents -> initial_values[{i}]: can't parse as float"))
        //     })
        //     .collect::<Vec<_>>();
        todo!()
    }
}


pub struct ExponentFunction {
    amplitude: float,
    shift: float,
    tau: float,
}

impl ExponentFunction {
    fn from_slice(params: &[float]) -> Self {
        match params[..] {
            [amplitude, shift, tau] => Self { amplitude, shift, tau },
            _ => unreachable!()
        }
    }

    fn eval_at(&self, x: float) -> float {
        Self::eval_at_(self.amplitude, self.tau, self.shift, x)
    }

    /// This is to prevent memory "segmentation":
    /// [`ExponentFunction`] have 3 floats, but whole struct will be aligned to 4 floats (i guess?)
    /// + 1 float as arg => 5 floats in memory,
    /// whereas this method uses only 4 floats, as expected.
    ///
    /// Also this maybe improves cache locality a tiny bit (no extra ghost float in memory).
    ///
    /// Unfortunately, no performance gain was measured.
    fn eval_at_(amplitude: float, tau: float, shift: float, x: float) -> float {
        let in_exp = -(x - shift) / tau;
        if in_exp <= 0. {
            amplitude * exp(in_exp)
        } else {
            0.
        }
    }
}

