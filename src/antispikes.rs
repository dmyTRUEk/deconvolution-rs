//! Antispikes.

use toml::Value as TomlValue;

use crate::{
    float_type::float,
    load::Load,
    stacktrace::Stacktrace,
};


#[derive(Debug, Clone, PartialEq)]
pub struct Antispikes {
    antispikes_type: AntispikesType,
    antispikes_k: float,
}

impl Antispikes {
    pub fn calc(&self, points_1: &Vec<float>, points_2: &Vec<float>) -> float {
        self.antispikes_k * self.antispikes_type.calc(points_1, points_2)
    }
}


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum AntispikesType {
    DySqr,
    DyAbs,
}

impl AntispikesType {
    fn calc(&self, points_1: &Vec<float>, points_2: &Vec<float>) -> float {
        assert_eq!(points_1.len(), points_2.len());
        match self {
            Self::DySqr => {
                let mut res: float = 0.;
                for points in [points_1, points_2] {
                    for [point_prev, point_next] in points.array_windows() {
                        let delta = point_next - point_prev;
                        let delta = delta.powi(2); // TODO(refactor): rename var
                        res += delta;
                    }
                }
                res.sqrt()
            }
            Self::DyAbs => {
                let mut res: float = 0.;
                for points in [points_1, points_2] {
                    for [point_prev, point_next] in points.array_windows() {
                        let delta = point_next - point_prev;
                        let delta = delta.abs(); // TODO(refactor): rename var
                        res += delta;
                    }
                }
                res
            }
        }
    }
}


impl Load for Antispikes {
    const TOML_NAME: &'static str = "antispikes";
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        todo!()
    }
}

