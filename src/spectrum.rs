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
            .unwrap()
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
    fn pad_zeros(&mut self) {
        let avg_index_of_max = Self::avg_index_of_max(&self.points);
        let avg_index_of_max: usize = avg_index_of_max as usize;
        let avg_index_of_max: isize = avg_index_of_max as isize;
        let shift_of_max: isize = avg_index_of_max - (self.points.len() / 2) as isize;
        if shift_of_max == 0 { return }
        let shift_of_max_abs: usize = shift_of_max.abs() as usize;
        let points = self.points.clone();
        let zeros_len = if points.len() % 2 == 0 {
            2*shift_of_max_abs+1
        } else {
            2*shift_of_max_abs
        };
        let zeros = vec![0.; zeros_len];
        self.points = match shift_of_max.cmp(&0) {
            Ordering::Less => [zeros, points].concat(),
            Ordering::Equal => points,
            Ordering::Greater => [points, zeros].concat(),
        };
    }

    pub fn load_from_file(filename: &str) -> Self {
        Self::try_load_from_file(filename).unwrap()
    }
    pub fn try_load_from_file(filename: &str) -> Result<Self, &'static str> {
        let Ok(file) = File::open(filename) else { return Err("Unable to open file") };
        let lines = BufReader::new(file).lines();
        let mut x_start: Option<float> = None;
        let mut x_prev: Option<float> = None;
        let mut step: Option<float> = None;
        let mut ys: Vec<float> = vec![];
        for line in lines.into_iter() {
            let Ok(line) = line else { return Err("Unable to unwrap line") };
            let line = line.trim();
            if line == "" { continue }
            let Some((x, y)) = line.split_once([' ', '\t']) else { return Err("Unable to split line once at space or tab.") };
            let Ok(x) = x.trim().replace(',', ".").parse::<float>() else { return Err("Unable to parse `x`") };
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
            let Ok(y) = y.trim().replace(',', ".").parse() else { return Err("Unable to parse `y`") };
            ys.push(y);
        }
        let Some(x_start) = x_start else { return Err("`start_x` is None") };
        let Some(step) = step else { return Err("`step` is None") };
        Ok(Spectrum {
            points: ys,
            x_start,
            step,
        })
    }
}

