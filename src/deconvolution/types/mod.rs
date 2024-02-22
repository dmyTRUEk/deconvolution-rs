//! Deconvolution types

pub mod exponents;
pub mod per_points;
#[allow(non_snake_case)]
pub mod sat_exp__dec_exp;
#[allow(non_snake_case)]
pub mod sat_exp__dec_exp_plus_const;
#[allow(non_snake_case)]
pub mod sat_exp__two_dec_exp;
#[allow(non_snake_case)]
pub mod sat_exp__two_dec_exp__constrained_consts;
#[allow(non_snake_case)]
pub mod sat_exp__two_dec_exp__separate_consts;
#[allow(non_snake_case)]
pub mod sat_exp__two_dec_exp_plus_const;
#[allow(non_snake_case)]
pub mod two__sat_exp__dec_exp;

use rand::{thread_rng, Rng, rngs::ThreadRng};

use crate::{
    extensions::SplitAndKeep,
    stacktrace::Stacktrace,
    types::{
        float::float,
        linalg::DVect,
        named_wrappers::{Deconvolved, DeconvolvedV, Params, ParamsG, ParamsV},
    },
};


pub(super) trait DeconvolutionType {
    /// Human readable name, used for output file.
    const NAME: &'static str;

    const FORMAT_FOR_DESMOS: &'static str;

    const FORMAT_FOR_ORIGIN: &'static str;

    fn to_plottable_function(&self, params: &Params, significant_digits: u8, format: &'static str) -> String;

    fn to_desmos_function(&self, params: &Params, significant_digits: u8) -> String {
        self.to_plottable_function(params, significant_digits, Self::FORMAT_FOR_DESMOS)
    }

    fn to_origin_function(&self, params: &Params, significant_digits: u8) -> String {
        self.to_plottable_function(params, significant_digits, Self::FORMAT_FOR_ORIGIN)
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
// Domain, Limits, Bounds
pub(super) enum ValueDomain {
    /// -∞ < x < ∞
    Free,

    /// x == x0
    Fixed, // no need in `float`, bc it is stored in `value` field of `ValueAndDomain`

    /// x_min < x  <=>  x > x_min
    RangeWithMin(float),

    /// x < x_max
    RangeWithMax(float),

    /// x_min < x < x_max
    RangeClosed(float, float),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ValueAndDomain {
    pub value: float,
    pub domain: ValueDomain,
}

impl ValueAndDomain {
    pub const fn free(value: float) -> Self {
        Self {
            value,
            domain: ValueDomain::Free,
        }
    }

    pub const fn fixed(value: float) -> Self {
        Self {
            value,
            domain: ValueDomain::Fixed,
        }
    }

    pub const fn range_with_min(value: float, min: float) -> Self {
        Self {
            value,
            domain: ValueDomain::RangeWithMin(min),
        }
    }

    pub const fn range_with_max(value: float, max: float) -> Self {
        Self {
            value,
            domain: ValueDomain::RangeWithMax(max),
        }
    }

    pub const fn range_closed(value: float, (min, max): (float, float)) -> Self {
        Self {
            value,
            domain: ValueDomain::RangeClosed(min, max),
        }
    }

    pub fn contains(&self, value: float) -> bool {
        match self.domain {
            ValueDomain::Free => true,
            ValueDomain::Fixed => self.value == value,
            ValueDomain::RangeWithMin(min) => min <= value,
            ValueDomain::RangeWithMax(max) => value <= max,
            ValueDomain::RangeClosed(min, max) => min <= value && value <= max,
        }
    }

    #[deprecated] // TODO: remove if not needed?
    pub fn get_randomized(&self, initial_values_random_scale: float) -> float {
        self.get_randomized_with_rng(initial_values_random_scale, &mut thread_rng())
    }

    pub fn get_randomized_with_rng(&self, initial_values_random_scale: float, rng: &mut ThreadRng) -> float {
        match self.domain {
            ValueDomain::Fixed => self.value,
            ValueDomain::Free => self.value * rng.gen_range(1./initial_values_random_scale .. initial_values_random_scale),
            ValueDomain::RangeClosed(..)
            | ValueDomain::RangeWithMin(..)
            | ValueDomain::RangeWithMax(..)
            => {
                loop {
                    let new_value = self.value * rng.gen_range(1./initial_values_random_scale .. initial_values_random_scale);
                    if self.contains(new_value) {
                        return new_value;
                    }
                }
            }
        }
    }

    pub fn load_from_str(str: &str, stacktrace: &Stacktrace) -> (String, Self) {
        let by_eq = |c: char| c == '=';
        let str = str.trim();
        let parts = str.split_and_keep(|c| c=='<' || c=='>');
        enum ValueDomainStr<'a> {
            Free,
            Fixed,
            RangeWithMax(&'a str),
            RangeWithMin(&'a str),
            RangeClosed(&'a str, &'a str),
        }
        let (name, value_str, domain_str): (&str, &str, ValueDomainStr) = match parts.as_slice() {
            [v] => match v.split_and_keep(by_eq).as_slice() {
                [name, "=", "=", num] => (name, num, ValueDomainStr::Fixed),
                [name, "=", num] => (name, num, ValueDomainStr::Free),
                _ => stacktrace.pushed("{var}").panic_cant_parse_as(r#""{var_name} = {var_value}" or "{var_name} == {var_value}""#)
            },
            [v, "<", max] => match v.split_and_keep(by_eq).as_slice() {
                [name, "=", num] => (name, num, ValueDomainStr::RangeWithMax(max)),
                _ => stacktrace.pushed("{var}").panic_cant_parse_as(r#""{var_name} < {var_value_max}""#)
            }
            [v, ">", min] => match v.split_and_keep(by_eq).as_slice() {
                [name, "=", num] => (name, num, ValueDomainStr::RangeWithMin(min)),
                _ => stacktrace.pushed("{var}").panic_cant_parse_as(r#""{var_name} > {var_value_min}""#)
            }
            [min, "<", v, "<", max] => match v.split_and_keep(by_eq).as_slice() {
                [name, "=", num] => (name, num, ValueDomainStr::RangeClosed(min, max)),
                _ => stacktrace.pushed("{var}").panic_cant_parse_as(r#""{var_value_min} < {var_value} < {var_value_max}""#)
            }
            _ => stacktrace.panic_cant_parse_as(r#""{var_free}" or "{var_fixed}" or "{var} < {var_value_max}" or "{var} > {var_value_min}" or "{var_value_min} < {var} < {var_value_max}""#)
        };
        let parse_float = |value_str: &str, value_name: &'static str| -> float {
            let stacktrace = stacktrace.pushed(value_name);
            value_str
                .trim()
                .parse::<float>()
                .unwrap_or_else(|_| stacktrace.panic_cant_parse_as("float"))
        };
        let value: float = parse_float(value_str, "value");
        let domain: ValueDomain = match domain_str {
            ValueDomainStr::Free => ValueDomain::Free,
            ValueDomainStr::Fixed => ValueDomain::Fixed,
            ValueDomainStr::RangeWithMax(max) => ValueDomain::RangeWithMax(parse_float(max, "max")),
            ValueDomainStr::RangeWithMin(min) => ValueDomain::RangeWithMin(parse_float(min, "min")),
            ValueDomainStr::RangeClosed(min, max) => ValueDomain::RangeClosed(parse_float(min, "min"), parse_float(max, "max")),
        };
        (name.trim().to_string(), Self { value, domain })
    }
}


// `T` is `float` or `ValueAndDomain`.
pub trait InitialValuesGeneric<T> {
    /// Number of initial values (don't depend on `self` => static)
    const LEN: usize;

    /// Number of initial values (depends on `self` => dynamic)
    fn len(&self) -> usize {
        Self::LEN
    }

    /// From vector
    fn from_vec(params: &ParamsG<T>) -> Self;
    // fn from_vec_v(params: &DVect) -> Self;

    /// To vector
    fn to_vec(&self) -> ParamsG<T>;

    // TODO:
    // /// From array of ValueAndDomain
    // fn from_array<const N: usize>(params: [T; N]) -> Self;

    /// Convert params to points
    ///
    /// `self` here needed just for `var.params_to_points()` instead of `Type::params_to_points()`,
    /// which prevents from mistakes (accidentaly using wrong type and getting wrong result).
    fn params_to_points(&self, params: &Params, points_len: usize, x_start_end: (float, float)) -> Deconvolved;
    fn params_to_points_v(&self, params: &ParamsV, points_len: usize, x_start_end: (float, float)) -> DeconvolvedV;
}


pub trait InitialValuesVAD
where Self: Sized + InitialValuesGeneric<ValueAndDomain>
{
    /// Check if given params are satisfying conditions
    fn is_params_ok(&self, params: &Params) -> bool {
        self.to_vec().0.iter()
            .zip(&params.0)
            .all(|(vad, &value)| vad.contains(value))
    }

    fn is_params_ok_v(&self, params: &ParamsV) -> bool {
        self.to_vec().0.iter()
            .zip(&params.0)
            .all(|(vad, &value)| vad.contains(value))
    }

    /// Get randomized initial values with given `ThreadRng`
    fn get_randomized_with_rng(&self, initial_values_random_scale: float, rng: &mut ThreadRng) -> Params {
        ParamsG::<float>( // == Params
            self.to_vec().0
                .iter()
                .map(|vad| vad.get_randomized_with_rng(initial_values_random_scale, rng))
                .collect()
        )
    }

    fn get_randomized_with_rng_v(&self, initial_values_random_scale: float, rng: &mut ThreadRng) -> ParamsV {
        let v = self.to_vec();
        ParamsV(
            DVect::from_iterator(
                v.0.len(),
                v.0.iter().map(|vad| vad.get_randomized_with_rng(initial_values_random_scale, rng))
            )
        )
    }
}



pub(self) fn i_to_x(
    i: usize,
    points_len: usize,
    (x_start, x_end): (float, float),
) -> float {
    let x_range: float = x_end - x_start;
    let t: float = (i as float) / ((points_len - 1) as float);
    let x: float = t * x_range + x_start;
    x
}

