//! Exponents

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


/// a1*exp(-(x-s1)/t1) + …
#[derive(Debug, Clone, PartialEq)]
pub struct Exponents {
    pub diff_function_type: DiffFunction,
    pub initial_vads: InitialValues_Exponents<ValueAndDomain>,
}

impl DeconvolutionType for Exponents {
    const NAME: &'static str = "exponents";

    const FORMAT_FOR_DESMOS: &'static str = r"\left\{x$comp$ns:0,$ae^{-\frac{x$p$ns}{$t}}\right\}";
    const FORMAT_FOR_ORIGIN: &'static str = todo!();

    fn to_plottable_function(&self, params: &Vec<float>, significant_digits: u8, format: &'static str) -> String {
        let sd = significant_digits;
        params
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
    Vec<T>
);

impl<T> InitialValuesGeneric<T> for InitialValues_Exponents<T> {
    fn len_stat() -> usize {
        unreachable!()
    }

    fn len_dyn(&self) -> usize {
        self.0.len()
    }

    fn from_vec(params: &Vec<T>) -> Self {
        todo!()
    }

    fn to_vec(&self) -> Vec<T> {
        todo!()
    }

    fn params_to_points(&self, params: &Vec<float>, points_len: usize, (x_start, x_end): (float, float)) -> Vec<float> {
        assert_eq!(0, params.len() % 3);
        let exponents: Vec<ExponentFunction> = params
            .chunks(3)
            .into_iter()
            .map(|parts| ExponentFunction::from_slice(parts))
            .collect();
        let mut points = Vec::<float>::with_capacity(points_len);
        for i in 0..points_len {
            let x: float = i_to_x(i, points_len, (x_start, x_end));
            let sum: float = exponents.iter()
                .map(|exponent| exponent.eval_at(x))
                .sum();
            points.push(sum);
        }
        points
    }
}

impl InitialValuesVAD for InitialValues_Exponents<ValueAndDomain> {
    fn is_params_ok(&self, params: &Vec<float>) -> bool {
        todo!();
        params.chunks(3).into_iter().all(|parts| {
            let (amplitude, _tau, _shift) = (parts[0], parts[1], parts[2]);
            amplitude >= 0.
        })
    }
}

impl From<InitialValues_Exponents<ValueAndDomain>> for InitialValues_Exponents<float> {
    fn from(value: InitialValues_Exponents<ValueAndDomain>) -> Self {
        Self::from_vec(&value.to_vec().iter().map(|v| v.value).collect())
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

