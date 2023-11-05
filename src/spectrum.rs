//! Spectrum.

use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader, Write},
};

use crate::{
    float_type::float,
    unmut,
};


#[derive(Debug, Clone, PartialEq)]
pub struct Spectrum {
    pub points: Vec<float>,
    pub step: float,
    pub x_start: float,
}

impl Spectrum {
    pub fn get_x_range(&self) -> float {
        // self.get_x_end() - self.x_start
        self.step * (self.points.len().saturating_sub(1) as float)
    }

    pub fn get_x_end(&self) -> float {
        // self.start_x + self.step * (self.points.len() as float)
        self.get_x_from_index(self.points.len())
    }

    pub fn get_xy_from_index(&self, i: usize) -> (float, float) {
        (self.get_x_from_index(i), self.get_y_from_index(i))
    }

    pub fn get_x_from_index(&self, i: usize) -> float {
        self.x_start + self.step * (i as float)
    }

    pub fn get_y_from_index(&self, i: usize) -> float {
        self.points[i]
    }

    pub fn get_indices_of_closest_to_lhs_rhs(&self, x: float) -> (usize, usize) {
        assert!(self.x_start <= x && x <= self.get_x_end());
        let x_from_start: float = x - self.x_start;
        let index_as_float: float = x_from_start / self.step;
        (index_as_float.floor() as usize, index_as_float.ceil() as usize)
    }

    pub fn get_points_len_after_recalc_with_step(&self, step_new: float) -> usize {
        assert!(self.points.len() > 1);
        ((self.get_x_range()) / step_new).floor() as usize + 1
    }

    pub fn recalculated_with_step(mut self, step_new: float) -> Self {
        if !step_new.is_finite() { panic!() }
        let self_old = self.clone();
        self.points = vec![];
        self.step = step_new;
        let points_len_after_recalc: usize = self_old.get_points_len_after_recalc_with_step(step_new);
        for i in 0..points_len_after_recalc {
            let x: float = self.get_x_from_index(i);
            let (index_of_closest_lhs, index_of_closest_rhs): (usize, usize) = self_old.get_indices_of_closest_to_lhs_rhs(x);
            let y: float = if index_of_closest_lhs == index_of_closest_rhs {
                self_old.points[index_of_closest_lhs]
            } else {
                assert_eq!(1, index_of_closest_rhs - index_of_closest_lhs);
                let t: float = (x - self_old.get_x_from_index(index_of_closest_lhs)) / self_old.step;
                (1.-t) * self_old.points[index_of_closest_lhs] + t * self_old.points[index_of_closest_rhs]
            };
            self.points.push(y);
        }
        self
    }

    pub fn write_to_file(&self, filename: &str) {
        self.write_to_file_with_separators(filename, ".", "\t");
    }
    pub fn write_to_file_with_separators(&self, filepath: &str, decimal_point_symbol: &str, numbers_separator: &str) {
        let mut file_output = File::create(filepath).unwrap();
        for i in 0..self.points.len() {
            let (x, y) = self.get_xy_from_index(i);
            writeln!(
                file_output,
                "{x}{numbers_separator}{y}",
                x=format!("{x}").replace('.', decimal_point_symbol),
                y=format!("{y}").replace('.', decimal_point_symbol),
            ).unwrap();
        }
    }

    pub fn load_from_file_as_instrumental(filename: &str) -> Self {
        let mut self_ = Self::load_from_file(filename);
        self_.trim_zeros();
        self_.pad_zeros();
        if self_.points.len() % 2 != 1 { unreachable!() }
        // dbg!(self_.points.len() / 2);
        // dbg!(Self::avg_index_of_max(&self_.points));
        if self_.points.len() / 2 != Self::avg_index_of_max(&self_.points) as usize { unreachable!() }
        self_
    }
    fn trim_zeros(&mut self) {
        let index_of_first_non_zero: usize = self.points
            .iter()
            .position(|&v| v != 0.)
            .expect("instrumental function must have at least one non zero");
        let index_of_last_non_zero: usize = self.points
            .iter()
            .enumerate() // TODO(optimization): check if this is optimal, and if no, do math "by hands".
            .rev()
            .find(|(_i, &v)| v != 0.)
            .unwrap() // no need to use `.expect()` bc if it were to crash, it would do on line with `.expect()` above
            .0;
        self.points = self.points[index_of_first_non_zero..=index_of_last_non_zero].to_vec();
        let number_of_removed_points_from_start = index_of_first_non_zero;
        self.x_start += self.step * (number_of_removed_points_from_start as float);
    }
    fn avg_index_of_max(points: &Vec<float>) -> float {
        assert!(points.len() > 0);
        let mut indices_of_maxes: Vec<usize> = vec![0];
        for (i, v) in points.iter().enumerate().skip(1) {
            match v.partial_cmp(&points[indices_of_maxes[0]]).unwrap() {
                Ordering::Greater => {
                    indices_of_maxes.clear();
                    indices_of_maxes.push(i);
                }
                Ordering::Equal => {
                    indices_of_maxes.push(i);
                }
                Ordering::Less => {}
            }
        }
        unmut!(indices_of_maxes);
        let indices_of_maxes: Vec<float> = indices_of_maxes
            .into_iter()
            .map(|v| v as float)
            .collect();
        let avg_index_of_max: float = indices_of_maxes.iter().sum::<float>() / (indices_of_maxes.len() as float);
        avg_index_of_max
    }
    /// Add zeros from the side, so that max is centred.
    fn pad_zeros(&mut self) {
        let points = self.points.clone();
        if points.len() == 0 { unreachable!() }
        if points.len() == 1 { return }
        let avg_index_of_max: float = Self::avg_index_of_max(&points);
        let index_of_center: float = (points.len() as float) / 2. - 0.5;
        let shift_of_max: float = avg_index_of_max - index_of_center;
        if shift_of_max == 0. { return }
        let shift_of_max_abs: usize = shift_of_max.abs() as usize;
        let zeros_len = 2*shift_of_max_abs + if points.len() % 2 == 0 { 1 } else { 0 };
        let zeros = vec![0.; zeros_len];
        self.points = match shift_of_max.total_cmp(&0.) {
            Ordering::Less => {
                self.x_start -= self.step * (zeros_len as float);
                [zeros, points].concat()
            },
            Ordering::Equal => points,
            Ordering::Greater => [points, zeros].concat(),
        };
    }

    pub fn load_from_file(filename: &str) -> Self {
        Self::try_load_from_file(filename).unwrap()
    }
    pub fn try_load_from_file(filename: &str) -> Result<Self, &'static str> {
        let file = File::open(filename).map_err(|_| "Unable to open file")?;
        let lines = BufReader::new(file).lines();
        let mut x_start: Option<float> = None;
        let mut x_prev: Option<float> = None;
        let mut step: Option<float> = None;
        let mut ys: Vec<float> = vec![];
        for line in lines.into_iter() {
            let line = line.map_err(|_| "Unable to unwrap line")?;
            let line = line.trim();
            if line == "" { continue }
            let (x, y) = line
                .split_once([' ', '\t'])
                .ok_or("Unable to split line once at space or tab.")?;
            let x = x
                .trim()
                .replace(',', ".")
                .parse::<float>()
                .map_err(|_| "Unable to parse `x`")?;
            match x_start {
                None => {
                    x_start = Some(x);
                }
                Some(x_start) => {
                    match step {
                        None => {
                            step = Some(x - x_start);
                        }
                        Some(step) => {
                            let this_step = x - x_prev.unwrap();
                            let diff = (this_step - step).abs() / step.abs();
                            assert!(diff < 2e-2, "step={step}, this_step={this_step} => diff={diff}");
                        }
                    }
                    x_prev = Some(x);
                }
            }
            let y = y
                .trim()
                .replace(',', ".")
                .parse()
                .map_err(|_| "Unable to parse `y`")?;
            ys.push(y);
        }
        let x_start = x_start.ok_or("`start_x` is None")?;
        let step = step.ok_or("`step` is None")?;
        Ok(Spectrum {
            points: ys,
            x_start,
            step,
        })
    }
}



#[cfg(test)]
mod avg_index_of_max {
    #![allow(non_snake_case)]

    use super::Spectrum;

    #[should_panic]
    #[test]
    fn empty() {
        assert_eq!(
            0., // unreachable
            Spectrum::avg_index_of_max(&vec![])
        );
    }

    mod center_single {
        use super::*;

        #[test]
        fn M() {
            assert_eq!(
                0.,
                Spectrum::avg_index_of_max(&vec![1.])
            );
        }

        #[test]
        fn zMz() {
            assert_eq!(
                1.,
                Spectrum::avg_index_of_max(&vec![0., 1., 0.])
            );
        }
        #[test]
        fn zzMzz() {
            assert_eq!(
                2.,
                Spectrum::avg_index_of_max(&vec![0., 0., 1., 0., 0.])
            );
        }
        #[test]
        fn zzzzMzzzz() {
            assert_eq!(
                4.,
                Spectrum::avg_index_of_max(&vec![0., 0., 0., 0., 1., 0., 0., 0., 0.])
            );
        }

        #[test]
        fn Mz() {
            assert_eq!(
                0.,
                Spectrum::avg_index_of_max(&vec![1., 0.])
            );
        }

        #[test]
        fn zM() {
            assert_eq!(
                1.,
                Spectrum::avg_index_of_max(&vec![0., 1.])
            );
        }

        #[test]
        fn Mzz() {
            assert_eq!(
                0.,
                Spectrum::avg_index_of_max(&vec![1., 0., 0.])
            );
        }

        #[test]
        fn zzM() {
            assert_eq!(
                2.,
                Spectrum::avg_index_of_max(&vec![0., 0., 1.])
            );
        }

        #[test]
        fn Mzzzz() {
            assert_eq!(
                0.,
                Spectrum::avg_index_of_max(&vec![1., 0., 0., 0., 0.])
            );
        }

        #[test]
        fn zzzzM() {
            assert_eq!(
                4.,
                Spectrum::avg_index_of_max(&vec![0., 0., 0., 0., 1.])
            );
        }

        #[test]
        fn zMzz() {
            assert_eq!(
                1.,
                Spectrum::avg_index_of_max(&vec![0., 1., 0., 0.])
            );
        }

        #[test]
        fn zzMz() {
            assert_eq!(
                1.,
                Spectrum::avg_index_of_max(&vec![0., 1., 0., 0.])
            );
        }

        #[test]
        fn zzMzzzz() {
            assert_eq!(
                2.,
                Spectrum::avg_index_of_max(&vec![0., 0., 1., 0., 0., 0., 0.])
            );
        }

        #[test]
        fn zzzzMzz() {
            assert_eq!(
                4.,
                Spectrum::avg_index_of_max(&vec![0., 0., 0., 0., 1., 0., 0.])
            );
        }
    }

    mod center_doubled {
        use super::*;

        #[test]
        fn M() {
            assert_eq!(
                0.5,
                Spectrum::avg_index_of_max(&vec![1., 1.])
            );
        }

        #[test]
        fn zMz() {
            assert_eq!(
                1.5,
                Spectrum::avg_index_of_max(&vec![0., 1., 1., 0.])
            );
        }
        #[test]
        fn zzMzz() {
            assert_eq!(
                2.5,
                Spectrum::avg_index_of_max(&vec![0., 0., 1., 1., 0., 0.])
            );
        }
        #[test]
        fn zzzzMzzzz() {
            assert_eq!(
                4.5,
                Spectrum::avg_index_of_max(&vec![0., 0., 0., 0., 1., 1., 0., 0., 0., 0.])
            );
        }

        #[test]
        fn Mz() {
            assert_eq!(
                0.5,
                Spectrum::avg_index_of_max(&vec![1., 1., 0.])
            );
        }

        #[test]
        fn zM() {
            assert_eq!(
                1.5,
                Spectrum::avg_index_of_max(&vec![0., 1., 1.])
            );
        }

        #[test]
        fn Mzz() {
            assert_eq!(
                0.5,
                Spectrum::avg_index_of_max(&vec![1., 1., 0., 0.])
            );
        }

        #[test]
        fn zzM() {
            assert_eq!(
                2.5,
                Spectrum::avg_index_of_max(&vec![0., 0., 1., 1.])
            );
        }

        #[test]
        fn Mzzzz() {
            assert_eq!(
                0.5,
                Spectrum::avg_index_of_max(&vec![1., 1., 0., 0., 0., 0.])
            );
        }

        #[test]
        fn zzzzM() {
            assert_eq!(
                4.5,
                Spectrum::avg_index_of_max(&vec![0., 0., 0., 0., 1., 1.])
            );
        }

        #[test]
        fn zMzz() {
            assert_eq!(
                1.5,
                Spectrum::avg_index_of_max(&vec![0., 1., 1., 0., 0.])
            );
        }

        #[test]
        fn zzMz() {
            assert_eq!(
                1.5,
                Spectrum::avg_index_of_max(&vec![0., 1., 1., 0., 0.])
            );
        }

        #[test]
        fn zzMzzzz() {
            assert_eq!(
                2.5,
                Spectrum::avg_index_of_max(&vec![0., 0., 1., 1., 0., 0., 0., 0.])
            );
        }

        #[test]
        fn zzzzMzz() {
            assert_eq!(
                4.5,
                Spectrum::avg_index_of_max(&vec![0., 0., 0., 0., 1., 1., 0., 0.])
            );
        }
    }

    mod center_tripled {
        use super::*;

        #[test]
        fn M() {
            assert_eq!(
                1.,
                Spectrum::avg_index_of_max(&vec![1., 1., 1.])
            );
        }

        #[test]
        fn zMz() {
            assert_eq!(
                2.,
                Spectrum::avg_index_of_max(&vec![0., 1., 1., 1., 0.])
            );
        }
        #[test]
        fn zzMzz() {
            assert_eq!(
                3.,
                Spectrum::avg_index_of_max(&vec![0., 0., 1., 1., 1., 0., 0.])
            );
        }
        #[test]
        fn zzzzMzzzz() {
            assert_eq!(
                5.,
                Spectrum::avg_index_of_max(&vec![0., 0., 0., 0., 1., 1., 1., 0., 0., 0., 0.])
            );
        }

        #[test]
        fn Mz() {
            assert_eq!(
                1.,
                Spectrum::avg_index_of_max(&vec![1., 1., 1., 0.])
            );
        }

        #[test]
        fn zM() {
            assert_eq!(
                2.,
                Spectrum::avg_index_of_max(&vec![0., 1., 1., 1.])
            );
        }

        #[test]
        fn Mzz() {
            assert_eq!(
                1.,
                Spectrum::avg_index_of_max(&vec![1., 1., 1., 0., 0.])
            );
        }

        #[test]
        fn zzM() {
            assert_eq!(
                3.,
                Spectrum::avg_index_of_max(&vec![0., 0., 1., 1., 1.])
            );
        }

        #[test]
        fn Mzzzz() {
            assert_eq!(
                1.,
                Spectrum::avg_index_of_max(&vec![1., 1., 1., 0., 0., 0., 0.])
            );
        }

        #[test]
        fn zzzzM() {
            assert_eq!(
                5.,
                Spectrum::avg_index_of_max(&vec![0., 0., 0., 0., 1., 1., 1.])
            );
        }

        #[test]
        fn zMzz() {
            assert_eq!(
                2.,
                Spectrum::avg_index_of_max(&vec![0., 1., 1., 1., 0., 0.])
            );
        }

        #[test]
        fn zzMz() {
            assert_eq!(
                2.,
                Spectrum::avg_index_of_max(&vec![0., 1., 1., 1., 0., 0.])
            );
        }

        #[test]
        fn zzMzzzz() {
            assert_eq!(
                3.,
                Spectrum::avg_index_of_max(&vec![0., 0., 1., 1., 1., 0., 0., 0., 0.])
            );
        }

        #[test]
        fn zzzzMzz() {
            assert_eq!(
                5.,
                Spectrum::avg_index_of_max(&vec![0., 0., 0., 0., 1., 1., 1., 0., 0.])
            );
        }
    }

    mod center_split {
        use super::*;

        #[test]
        fn MzM() {
            assert_eq!(
                1.,
                Spectrum::avg_index_of_max(&vec![1., 0., 1.])
            );
        }

        #[test]
        fn zMzM() {
            assert_eq!(
                2.,
                Spectrum::avg_index_of_max(&vec![0., 1., 0., 1.])
            );
        }

        #[test]
        fn MzMz() {
            assert_eq!(
                1.,
                Spectrum::avg_index_of_max(&vec![1., 0., 1., 0.])
            );
        }

        #[test]
        fn zMzMz() {
            assert_eq!(
                2.,
                Spectrum::avg_index_of_max(&vec![0., 1., 0., 1., 0.])
            );
        }

        #[test]
        fn zzzzMzMzz() {
            assert_eq!(
                5.,
                Spectrum::avg_index_of_max(&vec![0., 0., 0., 0., 1., 0., 1., 0., 0.])
            );
        }

        #[test]
        fn zzMzMzzzz() {
            assert_eq!(
                3.,
                Spectrum::avg_index_of_max(&vec![0., 0., 1., 0., 1., 0., 0., 0., 0.])
            );
        }

        #[test]
        fn MzMzM() {
            assert_eq!(
                2.,
                Spectrum::avg_index_of_max(&vec![1., 0., 1., 0., 1.])
            );
        }

        #[test]
        fn zMzMzM() {
            assert_eq!(
                3.,
                Spectrum::avg_index_of_max(&vec![0., 1., 0., 1., 0., 1.])
            );
        }

        #[test]
        fn MzMzMz() {
            assert_eq!(
                2.,
                Spectrum::avg_index_of_max(&vec![1., 0., 1., 0., 1., 0.])
            );
        }

        #[test]
        fn zzMzMzM() {
            assert_eq!(
                4.,
                Spectrum::avg_index_of_max(&vec![0., 0., 1., 0., 1., 0., 1.])
            );
        }

        #[test]
        fn MzMzMzz() {
            assert_eq!(
                2.,
                Spectrum::avg_index_of_max(&vec![1., 0., 1., 0., 1., 0., 0.])
            );
        }

        #[test]
        fn zzzzMzMzM() {
            assert_eq!(
                6.,
                Spectrum::avg_index_of_max(&vec![0., 0., 0., 0., 1., 0., 1., 0., 1.])
            );
        }

        #[test]
        fn MzMzMzzzz() {
            assert_eq!(
                2.,
                Spectrum::avg_index_of_max(&vec![1., 0., 1., 0., 1., 0., 0., 0., 0.])
            );
        }

        #[test]
        fn zzMzMzMz() {
            assert_eq!(
                4.,
                Spectrum::avg_index_of_max(&vec![0., 0., 1., 0., 1., 0., 1., 0.])
            );
        }

        #[test]
        fn zMzMzMzz() {
            assert_eq!(
                3.,
                Spectrum::avg_index_of_max(&vec![0., 1., 0., 1., 0., 1., 0., 0.])
            );
        }

        #[test]
        fn zzzzMzMzMzz() {
            assert_eq!(
                6.,
                Spectrum::avg_index_of_max(&vec![0., 0., 0., 0., 1., 0., 1., 0., 1., 0., 0.])
            );
        }

        #[test]
        fn zzMzMzMzzzz() {
            assert_eq!(
                4.,
                Spectrum::avg_index_of_max(&vec![0., 0., 1., 0., 1., 0., 1., 0., 0., 0., 0.])
            );
        }
    }
}

#[cfg(test)]
mod trim_zeros {
    use super::Spectrum;

    #[should_panic]
    #[test]
    fn empty() {
        let expected = Spectrum {
            points: vec![],
            step: 1.,
            x_start: 0.,
        };
        let mut actual = Spectrum {
            points: vec![],
            step: 1.,
            x_start: 0.,
        };
        actual.trim_zeros();
        assert_eq!(expected, actual);
    }

    #[test]
    fn n() {
        let expected = Spectrum {
            points: vec![1.],
            step: 1.,
            x_start: 0.,
        };
        let mut actual = Spectrum {
            points: vec![1.],
            step: 1.,
            x_start: 0.,
        };
        actual.trim_zeros();
        assert_eq!(expected, actual);
    }

    #[test]
    fn zn() {
        let expected = Spectrum {
            points: vec![1.],
            step: 1.,
            x_start: 1.,
        };
        let mut actual = Spectrum {
            points: vec![0., 1.],
            step: 1.,
            x_start: 0.,
        };
        actual.trim_zeros();
        assert_eq!(expected, actual);
    }

    #[test]
    fn nz() {
        let expected = Spectrum {
            points: vec![1.],
            step: 1.,
            x_start: 0.,
        };
        let mut actual = Spectrum {
            points: vec![1., 0.],
            step: 1.,
            x_start: 0.,
        };
        actual.trim_zeros();
        assert_eq!(expected, actual);
    }

    #[test]
    fn zzn() {
        let expected = Spectrum {
            points: vec![1.],
            step: 1.,
            x_start: 2.,
        };
        let mut actual = Spectrum {
            points: vec![0., 0., 1.],
            step: 1.,
            x_start: 0.,
        };
        actual.trim_zeros();
        assert_eq!(expected, actual);
    }

    #[test]
    fn nzz() {
        let expected = Spectrum {
            points: vec![1.],
            step: 1.,
            x_start: 0.,
        };
        let mut actual = Spectrum {
            points: vec![1., 0., 0.],
            step: 1.,
            x_start: 0.,
        };
        actual.trim_zeros();
        assert_eq!(expected, actual);
    }

    #[test]
    fn znz() {
        let expected = Spectrum {
            points: vec![1.],
            step: 1.,
            x_start: 1.,
        };
        let mut actual = Spectrum {
            points: vec![0., 1., 0.],
            step: 1.,
            x_start: 0.,
        };
        actual.trim_zeros();
        assert_eq!(expected, actual);
    }

    #[test]
    fn znzz() {
        let expected = Spectrum {
            points: vec![1.],
            step: 1.,
            x_start: 1.,
        };
        let mut actual = Spectrum {
            points: vec![0., 1., 0., 0.],
            step: 1.,
            x_start: 0.,
        };
        actual.trim_zeros();
        assert_eq!(expected, actual);
    }

    #[test]
    fn zznz() {
        let expected = Spectrum {
            points: vec![1.],
            step: 1.,
            x_start: 2.,
        };
        let mut actual = Spectrum {
            points: vec![0., 0., 1., 0.],
            step: 1.,
            x_start: 0.,
        };
        actual.trim_zeros();
        assert_eq!(expected, actual);
    }

    #[test]
    fn zznzz() {
        let expected = Spectrum {
            points: vec![1.],
            step: 1.,
            x_start: 2.,
        };
        let mut actual = Spectrum {
            points: vec![0., 0., 1., 0., 0.],
            step: 1.,
            x_start: 0.,
        };
        actual.trim_zeros();
        assert_eq!(expected, actual);
    }

    #[test]
    fn zzzznz() {
        let expected = Spectrum {
            points: vec![1.],
            step: 1.,
            x_start: 4.,
        };
        let mut actual = Spectrum {
            points: vec![0., 0., 0., 0., 1., 0.],
            step: 1.,
            x_start: 0.,
        };
        actual.trim_zeros();
        assert_eq!(expected, actual);
    }

    #[test]
    fn znzzzz() {
        let expected = Spectrum {
            points: vec![1.],
            step: 1.,
            x_start: 1.,
        };
        let mut actual = Spectrum {
            points: vec![0., 1., 0., 0., 0., 0.],
            step: 1.,
            x_start: 0.,
        };
        actual.trim_zeros();
        assert_eq!(expected, actual);
    }

    #[test]
    fn zzzznzz() {
        let expected = Spectrum {
            points: vec![1.],
            step: 1.,
            x_start: 4.,
        };
        let mut actual = Spectrum {
            points: vec![0., 0., 0., 0., 1., 0., 0.],
            step: 1.,
            x_start: 0.,
        };
        actual.trim_zeros();
        assert_eq!(expected, actual);
    }

    #[test]
    fn zznzzzz() {
        let expected = Spectrum {
            points: vec![1.],
            step: 1.,
            x_start: 2.,
        };
        let mut actual = Spectrum {
            points: vec![0., 0., 1., 0., 0., 0., 0.],
            step: 1.,
            x_start: 0.,
        };
        actual.trim_zeros();
        assert_eq!(expected, actual);
    }

    #[test]
    fn zzzznzzzz() {
        let expected = Spectrum {
            points: vec![1.],
            step: 1.,
            x_start: 4.,
        };
        let mut actual = Spectrum {
            points: vec![0., 0., 0., 0., 1., 0., 0., 0., 0.],
            step: 1.,
            x_start: 0.,
        };
        actual.trim_zeros();
        assert_eq!(expected, actual);
    }

    #[test]
    fn zzzznzznzz() {
        let expected = Spectrum {
            points: vec![1., 0., 0., 0.5],
            step: 1.,
            x_start: 4.,
        };
        let mut actual = Spectrum {
            points: vec![0., 0., 0., 0., 1., 0., 0., 0.5, 0., 0.],
            step: 1.,
            x_start: 0.,
        };
        actual.trim_zeros();
        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod pad_zeros {
    #![allow(non_snake_case)]

    use super::Spectrum;

    #[should_panic]
    #[test]
    fn empty() {
        let expected = Spectrum {
            points: vec![],
            step: 1.,
            x_start: 0.,
        };
        let mut actual = Spectrum {
            points: vec![],
            step: 1.,
            x_start: 0.,
        };
        actual.pad_zeros();
        assert_eq!(expected, actual);
    }

    mod center_single {
        use super::Spectrum;

        #[test]
        fn M() {
            let expected = Spectrum {
                points: vec![1.],
                step: 1.,
                x_start: 0.,
            };
            let mut actual = Spectrum {
                points: vec![1.],
                step: 1.,
                x_start: 0.,
            };
            actual.pad_zeros();
            assert_eq!(expected, actual);
        }

        #[test]
        fn nMn() {
            let expected = Spectrum {
                points: vec![0.1, 1., 0.1],
                step: 1.,
                x_start: 0.,
            };
            let mut actual = Spectrum {
                points: vec![0.1, 1., 0.1],
                step: 1.,
                x_start: 0.,
            };
            actual.pad_zeros();
            assert_eq!(expected, actual);
        }

        #[test]
        fn nnMnn() {
            let expected = Spectrum {
                points: vec![0.1, 0.1, 1., 0.1, 0.1],
                step: 1.,
                x_start: 0.,
            };
            let mut actual = Spectrum {
                points: vec![0.1, 0.1, 1., 0.1, 0.1],
                step: 1.,
                x_start: 0.,
            };
            actual.pad_zeros();
            assert_eq!(expected, actual);
        }

        #[test]
        fn nnnnMnnnn() {
            let expected = Spectrum {
                points: vec![0.1, 0.1, 0.1, 0.1, 1., 0.1, 0.1, 0.1, 0.1],
                step: 1.,
                x_start: 0.,
            };
            let mut actual = Spectrum {
                points: vec![0.1, 0.1, 0.1, 0.1, 1., 0.1, 0.1, 0.1, 0.1],
                step: 1.,
                x_start: 0.,
            };
            actual.pad_zeros();
            assert_eq!(expected, actual);
        }

        #[test]
        fn nM() {
            let expected = Spectrum {
                points: vec![0.1, 1., 0.],
                step: 1.,
                x_start: 0.,
            };
            let mut actual = Spectrum {
                points: vec![0.1, 1.],
                step: 1.,
                x_start: 0.,
            };
            actual.pad_zeros();
            assert_eq!(expected, actual);
        }

        #[test]
        fn Mn() {
            let expected = Spectrum {
                points: vec![0., 1., 0.1],
                step: 1.,
                x_start: -1.,
            };
            let mut actual = Spectrum {
                points: vec![1., 0.1],
                step: 1.,
                x_start: 0.,
            };
            actual.pad_zeros();
            assert_eq!(expected, actual);
        }

        #[test]
        fn nnM() {
            let expected = Spectrum {
                points: vec![0.1, 0.1, 1., 0., 0.],
                step: 1.,
                x_start: 0.,
            };
            let mut actual = Spectrum {
                points: vec![0.1, 0.1, 1.],
                step: 1.,
                x_start: 0.,
            };
            actual.pad_zeros();
            assert_eq!(expected, actual);
        }

        #[test]
        fn Mnn() {
            let expected = Spectrum {
                points: vec![0., 0., 1., 0.1, 0.1],
                step: 1.,
                x_start: -2.,
            };
            let mut actual = Spectrum {
                points: vec![1., 0.1, 0.1],
                step: 1.,
                x_start: 0.,
            };
            actual.pad_zeros();
            assert_eq!(expected, actual);
        }

        #[test]
        fn nnnnM() {
            let expected = Spectrum {
                points: vec![0.1, 0.1, 0.1, 0.1, 1., 0., 0., 0., 0.],
                step: 1.,
                x_start: 0.,
            };
            let mut actual = Spectrum {
                points: vec![0.1, 0.1, 0.1, 0.1, 1.],
                step: 1.,
                x_start: 0.,
            };
            actual.pad_zeros();
            assert_eq!(expected, actual);
        }

        #[test]
        fn Mnnnn() {
            let expected = Spectrum {
                points: vec![0., 0., 0., 0., 1., 0.1, 0.1, 0.1, 0.1],
                step: 1.,
                x_start: -4.,
            };
            let mut actual = Spectrum {
                points: vec![1., 0.1, 0.1, 0.1, 0.1],
                step: 1.,
                x_start: 0.,
            };
            actual.pad_zeros();
            assert_eq!(expected, actual);
        }

        #[test]
        fn nMnn() {
            let expected = Spectrum {
                points: vec![0., 0.1, 1., 0.1, 0.1],
                step: 1.,
                x_start: -1.,
            };
            let mut actual = Spectrum {
                points: vec![0.1, 1., 0.1, 0.1],
                step: 1.,
                x_start: 0.,
            };
            actual.pad_zeros();
            assert_eq!(expected, actual);
        }

        #[test]
        fn nnMn() {
            let expected = Spectrum {
                points: vec![0.1, 0.1, 1., 0.1, 0.],
                step: 1.,
                x_start: 0.,
            };
            let mut actual = Spectrum {
                points: vec![0.1, 0.1, 1., 0.1],
                step: 1.,
                x_start: 0.,
            };
            actual.pad_zeros();
            assert_eq!(expected, actual);
        }

        #[test]
        fn nMnnnn() {
            let expected = Spectrum {
                points: vec![0., 0., 0., 0.1, 1., 0.1, 0.1, 0.1, 0.1],
                step: 1.,
                x_start: -3.,
            };
            let mut actual = Spectrum {
                points: vec![0.1, 1., 0.1, 0.1, 0.1, 0.1],
                step: 1.,
                x_start: 0.,
            };
            actual.pad_zeros();
            assert_eq!(expected, actual);
        }

        #[test]
        fn nnnnMn() {
            let expected = Spectrum {
                points: vec![0.1, 0.1, 0.1, 0.1, 1., 0.1, 0., 0., 0.],
                step: 1.,
                x_start: 0.,
            };
            let mut actual = Spectrum {
                points: vec![0.1, 0.1, 0.1, 0.1, 1., 0.1],
                step: 1.,
                x_start: 0.,
            };
            actual.pad_zeros();
            assert_eq!(expected, actual);
        }

        #[test]
        fn nnMnnnn() {
            let expected = Spectrum {
                points: vec![0., 0., 0.1, 0.1, 1., 0.1, 0.1, 0.1, 0.1],
                step: 1.,
                x_start: -2.,
            };
            let mut actual = Spectrum {
                points: vec![0.1, 0.1, 1., 0.1, 0.1, 0.1, 0.1],
                step: 1.,
                x_start: 0.,
            };
            actual.pad_zeros();
            assert_eq!(expected, actual);
        }

        #[test]
        fn nnnnMnn() {
            let expected = Spectrum {
                points: vec![0.1, 0.1, 0.1, 0.1, 1., 0.1, 0.1, 0., 0.],
                step: 1.,
                x_start: 0.,
            };
            let mut actual = Spectrum {
                points: vec![0.1, 0.1, 0.1, 0.1, 1., 0.1, 0.1],
                step: 1.,
                x_start: 0.,
            };
            actual.pad_zeros();
            assert_eq!(expected, actual);
        }
    }
}

