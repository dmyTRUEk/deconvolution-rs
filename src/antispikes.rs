//! Antispikes.

use crate::float_type::float;


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
                    for point_prev_next in points.windows(2).into_iter() {
                        let (point_prev, point_next) = (point_prev_next[0], point_prev_next[1]);
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
                    for point_prev_next in points.windows(2).into_iter() {
                        let (point_prev, point_next) = (point_prev_next[0], point_prev_next[1]);
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

