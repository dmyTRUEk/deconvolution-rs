//! Helper function to convert from index to coordinate.

use crate::types::float::float;

pub(super) fn i_to_x(
    i: usize,
    points_len: usize,
    (x_start, x_end): (float, float),
) -> float {
    let x_range: float = x_end - x_start;
    let t: float = (i as float) / ((points_len - 1) as float);
    let x: float = t * x_range + x_start;
    x
}

