//! Diff Function.

use std::str::FromStr;

use toml::Value as TomlValue;

use crate::{
    antispikes::Antispikes,
    float_type::float,
    load::Load,
    stacktrace::Stacktrace,
};


#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DiffFunction {
    DySqr,
    DyAbs,
    DySqrPerEl,
    DyAbsPerEl,
    LeastDist,
}

impl DiffFunction {
    pub fn calc_diff(&self, points_1: &Vec<float>, points_2: &Vec<float>) -> float {
        assert_eq!(points_1.len(), points_2.len());
        match self {
            Self::DySqr => {
                let mut res: float = 0.;
                for (point_1, point_2) in points_1.into_iter().zip(points_2) {
                    let delta = point_2 - point_1;
                    let delta = delta.powi(2); // TODO(refactor): rename var
                    res += delta;
                }
                res.sqrt()
            }
            Self::DyAbs => {
                let mut res: float = 0.;
                for (point_1, point_2) in points_1.into_iter().zip(points_2) {
                    let delta = point_2 - point_1;
                    let delta = delta.abs(); // TODO(refactor): rename var
                    res += delta;
                }
                res
            }
            Self::DySqrPerEl => Self::DySqr.calc_diff(points_1, points_2) / points_1.len() as float,
            Self::DyAbsPerEl => Self::DyAbs.calc_diff(points_1, points_2) / points_1.len() as float,
            Self::LeastDist => { unimplemented!() }
        }
    }

    pub fn calc_diff_with_antispikes(&self, points_1: &Vec<float>, points_2: &Vec<float>, antispikes: &Option<Antispikes>) -> float {
        let diff_main: float = self.calc_diff(points_1, points_2);
        let diff_antispikes: float = antispikes.as_ref().map_or(
            0.,
            |antispikes| antispikes.calc(points_1, points_2)
        );
        diff_main + diff_antispikes
    }
}


impl FromStr for DiffFunction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DySqr" | "dy_sqr" => Ok(Self::DySqr),
            "DyAbs" | "dy_abs" => Ok(Self::DyAbs),
            "DySqrPerEl" | "dy_sqr_per_el" => Ok(Self::DySqrPerEl),
            "DyAbsPerEl" | "dy_abs_per_el" => Ok(Self::DyAbsPerEl),
            "LeastDist" | "least_dist" => Ok(Self::LeastDist),
            _ => Err(())
        }
    }
}


impl Load for DiffFunction {
    const TOML_NAME: &'static str = "diff_function_type";
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        let diff_function_str: &str = toml_value
            .as_str()
            .unwrap_or_else(|| stacktrace.panic_cant_parse_as("string"));
        const KNOWN_TYPES: [&str; 10] = ["DySqr", "dy_sqr", "DyAbs", "dy_abs", "DySqrPerEl", "dy_sqr_per_el", "DyAbsPerEl", "dy_abs_per_el", "LeastDist", "least_dist"];
        DiffFunction::from_str(diff_function_str)
            .unwrap_or_else(|_| stacktrace.panic_unknown_type(diff_function_str, KNOWN_TYPES))
    }
}

