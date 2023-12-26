//! Convolution.

use crate::float_type::float;


/// Must be used only in `tests` & `DeconvolutionData::convolve()`.
pub(super) fn convolve_by_points(
    points_instrument: &Vec<float>,
    points_deconvolved: &Vec<float>,
) -> Vec<float> {
    let points_instrument_len: usize = points_instrument.len();
    let points_deconvolved_len: usize = points_deconvolved.len();
    assert!(points_instrument_len % 2 == 1, "points_instrument_len = {}", points_instrument_len); // why?
    let points_convolved_len = points_deconvolved_len;
    let mut points_convolved = vec![0.; points_convolved_len];
    for i in 0..points_convolved_len {
        let mut point_convolved = 0.;
        for j in 0..points_instrument_len {
            let d: i32 = j as i32 - points_instrument_len as i32 / 2;
            let pii: i32 = j as i32;     // points_instrument_index
            let psi: i32 = i as i32 - d; // points_spectrum_index
            let is_psi_in_range: bool = 0 <= psi && psi < points_deconvolved_len as i32;
            if is_psi_in_range {
                let point_instrument  = points_instrument [pii as usize];
                let point_deconvolved = points_deconvolved[psi as usize];
                point_convolved += point_instrument * point_deconvolved;
            }
        }
        points_convolved[i] = point_convolved;
    }
    points_convolved
}



#[cfg(test)]
mod convolve {
    #![allow(non_snake_case)]
    mod per_point {
        use crate::{diff_function::DiffFunction, float};
        use super::super::convolve_by_points;
        mod instrument_is_identity__1 {
            use super::*;
            const POINTS_INSTRUMENT: [float; 1] = [1.];
            mod original_spectrum_is_delta_21 {
                use super::*;
                const EPSILON: float = 1e-6;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 10], vec![1.], vec![0.; 10]].concat();
                    let points_convolved_expected = points_spectrum_original.clone();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![1.], vec![0.; 20]].concat();
                    let points_convolved_expected = points_spectrum_original.clone();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 20], vec![1.]].concat();
                    let points_convolved_expected = points_spectrum_original.clone();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
            mod original_spectrum_is_two_deltas_20 {
                use super::*;
                const EPSILON: float = 1e-6;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 6], vec![1.], vec![0.; 6], vec![1.], vec![0.; 6]].concat();
                    let points_convolved_expected = points_spectrum_original.clone();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![1.], vec![0.; 6], vec![1.], vec![0.; 12]].concat();
                    let points_convolved_expected = points_spectrum_original.clone();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 12], vec![1.], vec![0.; 6], vec![1.]].concat();
                    let points_convolved_expected = points_spectrum_original.clone();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
        }
        mod instrument_is_delta3__0_1_0 {
            use super::*;
            const POINTS_INSTRUMENT: [float; 3] = [0., 1., 0.];
            mod original_spectrum_is_delta_21 {
                use super::*;
                const EPSILON: float = 1e-6;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 10], vec![1.], vec![0.; 10]].concat();
                    let points_convolved_expected = points_spectrum_original.clone();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![1.], vec![0.; 20]].concat();
                    let points_convolved_expected = points_spectrum_original.clone();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 20], vec![1.]].concat();
                    let points_convolved_expected = points_spectrum_original.clone();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
            mod original_spectrum_is_two_deltas_20 {
                use super::*;
                const EPSILON: float = 1e-6;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 6], vec![1.], vec![0.; 6], vec![1.], vec![0.; 6]].concat();
                    let points_convolved_expected = points_spectrum_original.clone();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![1.], vec![0.; 6], vec![1.], vec![0.; 12]].concat();
                    let points_convolved_expected = points_spectrum_original.clone();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 12], vec![1.], vec![0.; 6], vec![1.]].concat();
                    let points_convolved_expected = points_spectrum_original.clone();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
        }
        mod instrument_is_delta7__0_0_0_1_0_0_0 {
            use super::*;
            const POINTS_INSTRUMENT: [float; 7] = [0., 0., 0., 1., 0., 0., 0.];
            mod original_spectrum_is_delta_21 {
                use super::*;
                const EPSILON: float = 1e-6;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 10], vec![1.], vec![0.; 10]].concat();
                    let points_convolved_expected = points_spectrum_original.clone();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![1.], vec![0.; 20]].concat();
                    let points_convolved_expected = points_spectrum_original.clone();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 20], vec![1.]].concat();
                    let points_convolved_expected = points_spectrum_original.clone();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
            mod original_spectrum_is_two_deltas_20 {
                use super::*;
                const EPSILON: float = 1e-6;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 6], vec![1.], vec![0.; 6], vec![1.], vec![0.; 6]].concat();
                    let points_convolved_expected = points_spectrum_original.clone();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![1.], vec![0.; 6], vec![1.], vec![0.; 12]].concat();
                    let points_convolved_expected = points_spectrum_original.clone();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 12], vec![1.], vec![0.; 6], vec![1.]].concat();
                    let points_convolved_expected = points_spectrum_original.clone();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
        }
        mod instrument_is_symmetric_triangle5__0_05_1_05_0 {
            use super::*;
            const POINTS_INSTRUMENT: [float; 5] = [0., 0.5, 1., 0.5, 0.];
            mod original_spectrum_is_delta_21 {
                use super::*;
                const EPSILON: float = 1e-6;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 10], vec![1.], vec![0.; 10]].concat();
                    let points_convolved_expected = [vec![0.; 9], vec![0.5, 1., 0.5], vec![0.; 9]].concat();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![1.], vec![0.; 20]].concat();
                    let points_convolved_expected = [vec![1., 0.5], vec![0.; 19]].concat();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 20], vec![1.]].concat();
                    let points_convolved_expected = [vec![0.; 19], vec![0.5, 1.]].concat();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
            mod original_spectrum_is_two_deltas_20 {
                use super::*;
                const EPSILON: float = 1e-6;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 6], vec![1.], vec![0.; 6], vec![1.], vec![0.; 6]].concat();
                    let points_convolved_expected = [vec![0.; 5], vec![0.5, 1., 0.5], vec![0.; 4], vec![0.5, 1., 0.5], vec![0.; 5]].concat();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![1.], vec![0.; 6], vec![1.], vec![0.; 12]].concat();
                    let points_convolved_expected = [vec![1., 0.5], vec![0.; 4], vec![0.5, 1., 0.5], vec![0.; 11]].concat();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 12], vec![1.], vec![0.; 6], vec![1.]].concat();
                    let points_convolved_expected = [vec![0.; 11], vec![0.5, 1., 0.5], vec![0.; 4], vec![0.5, 1.]].concat();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
        }
        mod instrument_is_triangle3__0_1_05 {
            use super::*;
            const POINTS_INSTRUMENT: [float; 3] = [0., 1., 0.5];
            mod original_spectrum_is_delta_21 {
                use super::*;
                const EPSILON: float = 1e-6;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 10], vec![1.], vec![0.; 10]].concat();
                    let points_convolved_expected = [vec![0.; 10], vec![1., 0.5], vec![0.; 9]].concat();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![1.], vec![0.; 20]].concat();
                    let points_convolved_expected = [vec![1., 0.5], vec![0.; 19]].concat();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 20], vec![1.]].concat();
                    let points_convolved_expected = [vec![0.; 20], vec![1.]].concat();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
            mod original_spectrum_is_two_deltas_20 {
                use super::*;
                const EPSILON: float = 1e-6;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 6], vec![1.], vec![0.; 6], vec![1.], vec![0.; 6]].concat();
                    let points_convolved_expected = [vec![0.; 6], vec![1., 0.5], vec![0.; 5], vec![1., 0.5], vec![0.; 5]].concat();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![1.], vec![0.; 6], vec![1.], vec![0.; 12]].concat();
                    let points_convolved_expected = [vec![1., 0.5], vec![0.; 5], vec![1., 0.5], vec![0.; 11]].concat();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 12], vec![1.], vec![0.; 6], vec![1.]].concat();
                    let points_convolved_expected = [vec![0.; 12], vec![1., 0.5], vec![0.; 5], vec![1.]].concat();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
        }
        mod instrument_is_triangle5__0_0_1_05_02 {
            use super::*;
            const POINTS_INSTRUMENT: [float; 5] = [0., 0., 1., 0.5, 0.2];
            mod original_spectrum_is_delta_21 {
                use super::*;
                const EPSILON: float = 1e-6;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 10], vec![1.], vec![0.; 10]].concat();
                    let points_convolved_expected = [vec![0.; 10], vec![1., 0.5, 0.2], vec![0.; 8]].concat();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![1.], vec![0.; 20]].concat();
                    let points_convolved_expected = [vec![1., 0.5, 0.2], vec![0.; 18]].concat();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 20], vec![1.]].concat();
                    let points_convolved_expected = [vec![0.; 20], vec![1.]].concat();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
            mod original_spectrum_is_two_deltas_20 {
                use super::*;
                const EPSILON: float = 1e-6;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 6], vec![1.], vec![0.; 6], vec![1.], vec![0.; 6]].concat();
                    let points_convolved_expected = [vec![0.; 6], vec![1., 0.5, 0.2], vec![0.; 4], vec![1., 0.5, 0.2], vec![0.; 4]].concat();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![1.], vec![0.; 6], vec![1.], vec![0.; 12]].concat();
                    let points_convolved_expected = [vec![1., 0.5, 0.2], vec![0.; 4], vec![1., 0.5, 0.2], vec![0.; 10]].concat();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 12], vec![1.], vec![0.; 6], vec![1.]].concat();
                    let points_convolved_expected = [vec![0.; 12], vec![1., 0.5, 0.2], vec![0.; 4], vec![1.]].concat();
                    let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
        }
    }
    mod exponents {
        // TODO:
        // use crate::{DiffFunction, ExponentFunction, convolve_exponents, float};
        // mod instrument_is_identity {
        //     use super::*;
        //     const POINTS_INSTRUMENT: [float; 1] = [1.];
        //     #[test]
        //     fn original_spectrum_is_one_exp_1_1_1() {
        //         const EPSILON: float = 1e-6;
        //         const RES_LEN: usize = 10;
        //         println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
        //         let exponents = vec![ExponentFunction::new(1., 1., 1.)];
        //         let points_convolved_expected = [vec![0.; RES_LEN]].concat();
        //         let points_convolved_actual = convolve_exponents(&POINTS_INSTRUMENT.to_vec(), &exponents, RES_LEN);
        //         println!("points_convolved_expected = {:?}", points_convolved_expected);
        //         println!("points_convolved_actual = {:?}", points_convolved_actual);
        //         let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
        //         println!("diff = {}", diff);
        //         assert!(diff < 0.1);
        //         assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
        //     }
        // }
    }
}

