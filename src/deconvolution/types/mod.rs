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
pub mod sat_exp__two_dec_exp__separate_consts;
#[allow(non_snake_case)]
pub mod sat_exp__two_dec_exp_plus_const;
#[allow(non_snake_case)]
pub mod two__sat_exp__dec_exp;

use rand::{thread_rng, Rng};

use crate::{
    extensions::SplitAndKeep,
    float_type::float,
};


pub(super) trait DeconvolutionType {
    /// Human readable name, used for output file.
    const NAME: &'static str;
    const FORMAT_FOR_DESMOS: &'static str;
    const FORMAT_FOR_ORIGIN: &'static str;
    fn to_plottable_function(&self, params: &Vec<float>, significant_digits: u8, format: &'static str) -> String;
    fn to_desmos_function(&self, params: &Vec<float>, significant_digits: u8) -> String {
        self.to_plottable_function(params, significant_digits, Self::FORMAT_FOR_DESMOS)
    }
    fn to_origin_function(&self, params: &Vec<float>, significant_digits: u8) -> String {
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
    /// x_min < x < x_max 
    Range(float, float),
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
            domain: ValueDomain::Fixed
        }
    }

    pub const fn range(value: float, (min, max): (float, float)) -> Self {
        Self {
            value,
            domain: ValueDomain::Range(min, max)
        }
    }

    pub fn contains(&self, value: float) -> bool {
        match self.domain {
            ValueDomain::Free => true,
            ValueDomain::Fixed => self.value == value,
            ValueDomain::Range(min, max) => min <= value && value <= max,
        }
    }

    pub fn randomize(&mut self, initial_values_random_scale: float) {
        let mut rng = thread_rng();
        match self.domain {
            ValueDomain::Fixed => return,
            ValueDomain::Range(min, max) => {
                self.value = rng.gen_range(min..max);
            }
            ValueDomain::Free => {
                self.value *= rng.gen_range(1./initial_values_random_scale .. initial_values_random_scale);
            }
        }
    }

    pub fn load_from_str(str: &str) -> (String, Self) {
        let by_eq = |c: char| c == '=';
        let parts = str.split_and_keep(|c| c=='<' || c=='>');
        match parts.as_slice() {
            [v] => match v.split_and_keep(by_eq).as_slice() {
                [name, "=", "=", num] => (name.to_string(), Self::fixed(num.parse().expect("can't parse as number"))),
                [name, "=", num]      => (name.to_string(), Self::free (num.parse().expect("can't parse as number"))),
                _ => panic!()
            },
            [v, "<", max] => match v.split_and_keep(by_eq).as_slice() {
                [name, "=", num] => (name.to_string(), Self::range(num.parse().expect("can't parse as number"), (float::MIN, max.parse().expect("can't parse as number")))),
                _ => panic!("{str}")
            }
            [v, ">", min] => match v.split_and_keep(by_eq).as_slice() {
                [name, "=", num] => (name.to_string(), Self::range(num.parse().expect("can't parse as number"), (min.parse().expect("can't parse as number"), float::MAX))),
                _ => panic!()
            }
            [min, "<", v, "<", max] => match v.split_and_keep(by_eq).as_slice() {
                [name, "=", num] => (name.to_string(), Self::range(num.parse().expect("can't parse as number"), (min.parse().expect("can't parse as number"), max.parse().expect("can't parse as number")))),
                _ => panic!()
            }
            _ => panic!()
        }
    }
}


// pub(super) trait InitialValues<T> =
//     InitialValuesGeneric<T>
//     + InitialValuesVAD
//     + InitialValuesF
//     + From<>;


pub trait InitialValuesGeneric<T> {
    /// From vector
    fn from_vec(params: &Vec<T>) -> Self;

    /// To vector
    fn to_vec(&self) -> Vec<T>;

    // TODO:
    // /// From array of ValueAndDomain
    // fn from_array<const N: usize>(params: [T; N]) -> Self;

    /// Number of initial values (don't depend on `self` => static)
    fn len_stat() -> usize;

    /// Number of initial values (depends on `self` => dynamic)
    fn len_dyn(&self) -> usize {
        Self::len_stat()
    }

    /// Convert params to points
    ///
    /// `self` here needed just for `var.params_to_points()` instead of `Type::params_to_points()`,
    /// which prevents from mistakes (accidentaly using wrong type and getting wrong result).
    fn params_to_points(&self, params: &Vec<float>, points_len: usize, x_start_end: (float, float)) -> Vec<float>;
}


pub trait InitialValuesVAD
where Self: InitialValuesGeneric<ValueAndDomain>
{
    /// Check if given params are satisfying conditions
    fn is_params_ok(&self, params: &Vec<float>) -> bool {
        self.to_vec().iter()
            .zip(params)
            .all(|(d, &p)| d.contains(p))
    }

    /// Randomize initial values
    fn randomize(&mut self, initial_values_random_scale: float) {
        self.to_vec().iter_mut()
            .for_each(|vad| vad.randomize(initial_values_random_scale));
    }
}


pub trait InitialValuesF {}



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

