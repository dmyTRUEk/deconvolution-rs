//! Spectrum.

use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

use crate::float_type::float;


#[derive(Debug, Clone, PartialEq)]
pub struct Spectrum {
    pub points: Vec<float>,
    pub step: float,
    pub x_start: float,
}

impl Spectrum {
    #[allow(dead_code)]
    #[deprecated = "explicitly use field names to not mix up them"]
    pub fn new(points: Vec<float>, step: float, x_start: float) -> Self {
        Self { points, step, x_start }
    }

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

    pub fn load_from_file(filename: &str) -> Self {
        Self::try_load_from_file(filename).unwrap()
    }
    pub fn try_load_from_file(filename: &str) -> Result<Self, &'static str> {
        let Ok(file) = File::open(filename) else { return Err("Unable to open file") };
        let lines = BufReader::new(file).lines();
        let mut x_start: Option<float> = None;
        let mut x_prev: Option<float> = None;
        let mut step: Option<float> = None;
        let mut ys = Vec::<float>::with_capacity(20);
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

