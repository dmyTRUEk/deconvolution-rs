//! Value and Domain.

use rand::{Rng, rngs::ThreadRng};

use crate::{
    extensions::SplitAndKeep,
    stacktrace::Stacktrace,
    types::float::float,
};


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
    domain: ValueDomain,
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

    // pub fn get_randomized(&self, initial_values_random_scale: float) -> float {
    //     self.get_randomized_with_rng(initial_values_random_scale, &mut thread_rng())
    // }

    pub fn get_randomized_with_rng(&self, initial_values_random_scale: float, rng: &mut ThreadRng) -> float {
        match self.domain {
            ValueDomain::Fixed => self.value,
            ValueDomain::Free => self.value * rng.gen_range(1./initial_values_random_scale .. initial_values_random_scale),
            ValueDomain::RangeClosed(..)
            | ValueDomain::RangeWithMin(..)
            | ValueDomain::RangeWithMax(..)
            => {
                // TODO: optimize
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

