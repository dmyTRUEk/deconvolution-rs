//! Convolution.

use crate::float_type::float;


/// Must be used only in `tests` & `DeconvolutionData::convolve()`.
pub fn convolve_by_points(points_instrument: &Vec<float>, points_deconvolved: &Vec<float>) -> Vec<float> {
    let points_instrument_len: usize = points_instrument.len();
    let points_deconvolved_len: usize = points_deconvolved.len();
    assert!(points_instrument_len % 2 == 1, "points_instrument_len = {}", points_instrument_len); // why?
    let mut points_convolved = vec![0.; points_deconvolved_len];
    for i in 0..points_deconvolved_len {
        let mut point_convolved = 0.;
        for j in 0..points_instrument_len {
            let d: i32 = j as i32 - points_instrument_len as i32 / 2;
            let pii: i32 = j as i32;     // points_instrument_index
            let psi: i32 = i as i32 - d; // points_spectrum_index
            let is_pii_in_range: bool = 0 <= pii && pii < points_instrument_len as i32;
            let is_psi_in_range: bool = 0 <= psi && psi < points_deconvolved_len as i32;
            if is_pii_in_range && is_psi_in_range {
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
    mod per_point {
        use crate::{diff_function::DiffFunction, convolution::convolve_by_points, float};
        mod instrument_is_identity {
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
        mod instrument_is_delta3 {
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
        mod instrument_is_delta7 {
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
        mod instrument_is_triangle5 {
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

#[cfg(test)]
mod deconvolution_data {
    use crate::{Deconvolution, DeconvolutionData, diff_function::DiffFunction, Spectrum};
    mod align_steps_to_smaller {
        use super::*;
        #[test]
        fn align_i0_4_to_m0_2() {
            assert_eq!(
                DeconvolutionData {
                    instrument: Spectrum {
                        points: vec![0., 0., 0., 0.4999999999999999, 1., 0.5000000000000001, 0., 0., 0.],
                        step: 0.2,
                        x_start: 0.7,
                    },
                    measured: Spectrum {
                        points: vec![0., 0.1, 0.2, 0.3, 0.4, 0.3, 0.2, 0.1, 0.],
                        step: 0.2,
                        x_start: 0.3,
                    },
                    deconvolution: Deconvolution::PerPoint {
                        diff_function_type: DiffFunction::DySqr,
                        antispikes: None,
                        initial_value: 0.,
                    },
                },
                DeconvolutionData {
                    instrument: Spectrum {
                        points: vec![0., 0., 1., 0., 0.,],
                        step: 0.4,
                        x_start: 0.7,
                    },
                    measured: Spectrum {
                        points: vec![0., 0.1, 0.2, 0.3, 0.4, 0.3, 0.2, 0.1, 0.],
                        step: 0.2,
                        x_start: 0.3,
                    },
                    deconvolution: Deconvolution::PerPoint {
                        diff_function_type: DiffFunction::DySqr,
                        antispikes: None,
                        initial_value: 0.,
                    },
                }.aligned_steps_to_smaller()
            );
        }
        #[test]
        fn align_m0_4_to_i0_2() {
            assert_eq!(
                DeconvolutionData {
                    instrument: Spectrum {
                        points: vec![0., 0.1, 0.2, 0.3, 0.4, 0.3, 0.2, 0.1, 0.],
                        step: 0.2,
                        x_start: 0.5,
                    },
                    measured: Spectrum {
                        points: vec![0., 0., 0., 0.4999999999999999, 1., 0.5000000000000007, 0., 0., 0.],
                        step: 0.2,
                        x_start: 0.9,
                    },
                    deconvolution: Deconvolution::PerPoint {
                        diff_function_type: DiffFunction::DySqr,
                        antispikes: None,
                        initial_value: 0.,
                    },
                },
                DeconvolutionData {
                    instrument: Spectrum {
                        points: vec![0., 0.1, 0.2, 0.3, 0.4, 0.3, 0.2, 0.1, 0.],
                        step: 0.2,
                        x_start: 0.5,
                    },
                    measured: Spectrum {
                        points: vec![0., 0., 1., 0., 0.,],
                        step: 0.4,
                        x_start: 0.9,
                    },
                    deconvolution: Deconvolution::PerPoint {
                        diff_function_type: DiffFunction::DySqr,
                        antispikes: None,
                        initial_value: 0.,
                    },
                }.aligned_steps_to_smaller()
            );
        }
    }
    mod align_steps_to_bigger {
        use super::*;
        #[test]
        fn align_m0_2_to_i0_4() {
            assert_eq!(
                DeconvolutionData {
                    instrument: Spectrum {
                        points: vec![0., 0., 1., 0., 0.,],
                        step: 0.4,
                        x_start: 0.1,
                    },
                    measured: Spectrum {
                        points: vec![0., 0.2, 0.4, 0.2, 0.],
                        step: 0.4,
                        x_start: 0.5,
                    },
                    deconvolution: Deconvolution::PerPoint {
                        diff_function_type: DiffFunction::DySqr,
                        antispikes: None,
                        initial_value: 0.,
                    },
                },
                DeconvolutionData {
                    instrument: Spectrum {
                        points: vec![0., 0., 1., 0., 0.,],
                        step: 0.4,
                        x_start: 0.1,
                    },
                    measured: Spectrum {
                        points: vec![0., 0.1, 0.2, 0.3, 0.4, 0.3, 0.2, 0.1, 0.],
                        step: 0.2,
                        x_start: 0.5,
                    },
                    deconvolution: Deconvolution::PerPoint {
                        diff_function_type: DiffFunction::DySqr,
                        antispikes: None,
                        initial_value: 0.,
                    },
                }.aligned_steps_to_bigger()
            );
        }
        #[test]
        fn align_i0_2_to_m0_4() {
            assert_eq!(
                DeconvolutionData {
                    instrument: Spectrum {
                        points: vec![0., 0.2, 0.4, 0.2, 0.],
                        step: 0.4,
                        x_start: 0.5,
                    },
                    measured: Spectrum {
                        points: vec![0., 0., 1., 0., 0.,],
                        step: 0.4,
                        x_start: 0.9,
                    },
                    deconvolution: Deconvolution::PerPoint {
                        diff_function_type: DiffFunction::DySqr,
                        antispikes: None,
                        initial_value: 0.,
                    },
                },
                DeconvolutionData {
                    instrument: Spectrum {
                        points: vec![0., 0.1, 0.2, 0.3, 0.4, 0.3, 0.2, 0.1, 0.],
                        step: 0.2,
                        x_start: 0.5,
                    },
                    measured: Spectrum {
                        points: vec![0., 0., 1., 0., 0.,],
                        step: 0.4,
                        x_start: 0.9,
                    },
                    deconvolution: Deconvolution::PerPoint {
                        diff_function_type: DiffFunction::DySqr,
                        antispikes: None,
                        initial_value: 0.,
                    },
                }.aligned_steps_to_bigger()
            );
        }
    }
}

#[cfg(test)]
mod deconvolve {
    mod per_point {
        use crate::{
            Deconvolution,
            DeconvolutionData,
            Spectrum,
            deconvolution_data::DeconvolutionResultOrError,
            diff_function::DiffFunction,
            fit_algorithm::{FitAlgorithm, PatternSearchParams},
            float,
        };
        const DECONVOLUTION: Deconvolution = Deconvolution::PerPoint {
            diff_function_type: DiffFunction::DySqr,
            antispikes: None,
            initial_value: 0.,
        };
        const FIT_ALGORITHM: FitAlgorithm = FitAlgorithm::PatternSearch(PatternSearchParams {
            fit_algorithm_min_step: 1e-4,
            fit_residue_evals_max: 1_000_000,
            fit_residue_max_value: 1e6,
            initial_step: 1.,
            alpha: 1.1,
            beta: None,
        });
        fn deconvolve(points_instrument: Vec<float>, points_spectrum: Vec<float>) -> DeconvolutionResultOrError {
            let instrument: Spectrum = Spectrum {
                points: points_instrument,
                step: 1.,
                x_start: 0.,
            };
            let measured: Spectrum = Spectrum {
                points: points_spectrum,
                step: 1.,
                x_start: 0.,
            };
            let deconvolution_data: DeconvolutionData = DeconvolutionData {
                instrument,
                measured,
                deconvolution: DECONVOLUTION,
            };
            deconvolution_data.deconvolve(&FIT_ALGORITHM)
        }
        mod instrument_is_identity {
            use super::*;
            const POINTS_INSTRUMENT: [float; 1] = [1.];
            mod original_spectrum_is_delta_21 {
                use super::*;
                const EPSILON: float = 1e-4;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 10], vec![1.], vec![0.; 10]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![1.], vec![0.; 20]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 20], vec![1.]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
            mod original_spectrum_is_two_deltas_20 {
                use super::*;
                const EPSILON: float = 1e-4;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 6], vec![1.], vec![0.; 6], vec![1.], vec![0.; 6]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![1.], vec![0.; 6], vec![1.], vec![0.; 12]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 12], vec![1.], vec![0.; 6], vec![1.]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
        }
        mod instrument_is_delta3 {
            use super::*;
            const POINTS_INSTRUMENT: [float; 3] = [0., 1., 0.];
            mod original_spectrum_is_delta_21 {
                use super::*;
                const EPSILON: float = 1e-4;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 10], vec![1.], vec![0.; 10]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![1.], vec![0.; 20]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 20], vec![1.]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
            mod original_spectrum_is_two_deltas_20 {
                use super::*;
                const EPSILON: float = 1e-4;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 6], vec![1.], vec![0.; 6], vec![1.], vec![0.; 6]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![1.], vec![0.; 6], vec![1.], vec![0.; 12]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 12], vec![1.], vec![0.; 6], vec![1.]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
        }
        mod instrument_is_delta7 {
            use super::*;
            const POINTS_INSTRUMENT: [float; 7] = [0., 0., 0., 1., 0., 0., 0.];
            mod original_spectrum_is_delta_21 {
                use super::*;
                const EPSILON: float = 1e-4;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 10], vec![1.], vec![0.; 10]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![1.], vec![0.; 20]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 20], vec![1.]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
            mod original_spectrum_is_two_deltas_20 {
                use super::*;
                const EPSILON: float = 1e-4;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 6], vec![1.], vec![0.; 6], vec![1.], vec![0.; 6]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![1.], vec![0.; 6], vec![1.], vec![0.; 12]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 12], vec![1.], vec![0.; 6], vec![1.]].concat();
                    let points_deconvolved_expected = points_spectrum_convolved.clone();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
        }
        mod instrument_is_triangle5 {
            use super::*;
            const POINTS_INSTRUMENT: [float; 5] = [0., 0.5, 1., 0.5, 0.];
            mod original_spectrum_is_delta_21 {
                use super::*;
                const EPSILON: float = 1e-4;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 9], vec![0.5, 1., 0.5], vec![0.; 9]].concat();
                    let points_deconvolved_expected = [vec![0.; 10], vec![1.], vec![0.; 10]].concat();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![1., 0.5], vec![0.; 19]].concat();
                    let points_deconvolved_expected = [vec![1.], vec![0.; 20]].concat();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 19], vec![0.5, 1.]].concat();
                    let points_deconvolved_expected = [vec![0.; 20], vec![1.]].concat();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
            mod original_spectrum_is_two_deltas_20 {
                use super::*;
                const EPSILON: float = 1e-4;
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 5], vec![0.5, 1., 0.5], vec![0.; 4], vec![0.5, 1., 0.5], vec![0.; 5]].concat();
                    let points_deconvolved_expected = [vec![0.; 6], vec![1.], vec![0.; 6], vec![1.], vec![0.; 6]].concat();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_left() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![1., 0.5], vec![0.; 4], vec![0.5, 1., 0.5], vec![0.; 11]].concat();
                    let points_deconvolved_expected = [vec![1.], vec![0.; 6], vec![1.], vec![0.; 12]].concat();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                #[test]
                fn at_right() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_convolved = [vec![0.; 11], vec![0.5, 1., 0.5], vec![0.; 4], vec![0.5, 1.]].concat();
                    let points_deconvolved_expected = [vec![0.; 12], vec![1.], vec![0.; 6], vec![1.]].concat();
                    let deconvolve_results = deconvolve(POINTS_INSTRUMENT.to_vec(), points_spectrum_convolved).unwrap();
                    let points_deconvolved_actual = deconvolve_results.params;
                    println!("fit_residue_evals = {}", deconvolve_results.fit_residue_evals);
                    println!("points_deconvolved_expected = {:?}", points_deconvolved_expected);
                    println!("points_deconvolved_actual = {:?}", points_deconvolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_deconvolved_expected, &points_deconvolved_actual);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
        }
    }
}

#[cfg(test)]
mod spectrum {
    use crate::Spectrum;
    mod get_x_from_index {
        use super::*;
        #[test]
        fn _5_0() {
            assert_eq!(
                0.7,
                Spectrum {
                    points: vec![],
                    step: 0.4,
                    x_start: 0.7,
                }.get_x_from_index(0)
            );
        }
        #[test]
        fn _5_1() {
            assert_eq!(
                1.1,
                Spectrum {
                    points: vec![],
                    step: 0.4,
                    x_start: 0.7,
                }.get_x_from_index(1)
            );
        }
        #[test]
        fn _5_2() {
            assert_eq!(
                1.5,
                Spectrum {
                    points: vec![],
                    step: 0.4,
                    x_start: 0.7,
                }.get_x_from_index(2)
            );
        }
        #[test]
        fn _5_3() {
            assert_eq!(
                1.9000000000000001,
                Spectrum {
                    points: vec![],
                    step: 0.4,
                    x_start: 0.7,
                }.get_x_from_index(3)
            );
        }
        #[test]
        fn _5_4() {
            assert_eq!(
                2.3,
                Spectrum {
                    points: vec![],
                    step: 0.4,
                    x_start: 0.7,
                }.get_x_from_index(4)
            );
        }
        #[test]
        fn _5_5() {
            assert_eq!(
                2.7,
                Spectrum {
                    points: vec![],
                    step: 0.4,
                    x_start: 0.7,
                }.get_x_from_index(5)
            );
        }
    }
    mod get_x_end {
        use super::*;
        #[test]
        fn _0() {
            assert_eq!(
                0.7,
                Spectrum {
                    points: vec![],
                    step: 0.4,
                    x_start: 0.7,
                }.get_x_end()
            );
        }
        #[test]
        fn _1() {
            assert_eq!(
                1.1,
                Spectrum {
                    points: vec![0.],
                    step: 0.4,
                    x_start: 0.7,
                }.get_x_end()
            );
        }
        #[test]
        fn _2() {
            assert_eq!(
                1.5,
                Spectrum {
                    points: vec![0., 0.1],
                    step: 0.4,
                    x_start: 0.7,
                }.get_x_end()
            );
        }
        #[test]
        fn _3() {
            assert_eq!(
                1.9000000000000001,
                Spectrum {
                    points: vec![0., 0.1, 0.2],
                    step: 0.4,
                    x_start: 0.7,
                }.get_x_end()
            );
        }
        #[test]
        fn _4() {
            assert_eq!(
                2.3,
                Spectrum {
                    points: vec![0., 0.1, 0.2, 0.1],
                    step: 0.4,
                    x_start: 0.7,
                }.get_x_end()
            );
        }
        #[test]
        fn _5() {
            assert_eq!(
                2.7,
                Spectrum {
                    points: vec![0., 0.1, 0.2, 0.1, 0.],
                    step: 0.4,
                    x_start: 0.7,
                }.get_x_end()
            );
        }
    }
    mod get_x_range {
        use super::*;
        #[test]
        fn _0() {
            assert_eq!(
                0.,
                Spectrum {
                    points: vec![],
                    step: 0.4,
                    x_start: 0.7,
                }.get_x_range()
            );
        }
        #[test]
        fn _1() {
            assert_eq!(
                0.,
                Spectrum {
                    points: vec![0.],
                    step: 0.4,
                    x_start: 0.7,
                }.get_x_range()
            );
        }
        #[test]
        fn _2() {
            assert_eq!(
                0.4,
                Spectrum {
                    points: vec![0., 0.1],
                    step: 0.4,
                    x_start: 0.7,
                }.get_x_range()
            );
        }
        #[test]
        fn _3() {
            assert_eq!(
                0.8,
                Spectrum {
                    points: vec![0., 0.1, 0.2],
                    step: 0.4,
                    x_start: 0.7,
                }.get_x_range()
            );
        }
        #[test]
        fn _4() {
            assert_eq!(
                1.2000000000000002,
                Spectrum {
                    points: vec![0., 0.1, 0.2, 0.1],
                    step: 0.4,
                    x_start: 0.7,
                }.get_x_range()
            );
        }
        #[test]
        fn _5() {
            assert_eq!(
                1.6,
                Spectrum {
                    points: vec![0., 0.1, 0.2, 0.1, 0.],
                    step: 0.4,
                    x_start: 0.7,
                }.get_x_range()
            );
        }
    }
    #[allow(non_snake_case)]
    mod get_indices_of_closest_to_lhs_rhs {
        use super::*;
        #[test]
        fn _5__0_1() {
            for x in [0.8, 0.9, 1.] {
                dbg!(x);
                assert_eq!(
                    (0, 1),
                    Spectrum {
                        points: vec![0., 0.1, 0.2, 0.1, 0.],
                        step: 0.4,
                        x_start: 0.7,
                    }.get_indices_of_closest_to_lhs_rhs(x)
                );
            }
        }
        #[test]
        fn _5__1_2() {
            for x in [1.2, 1.3, 1.4] {
                dbg!(x);
                assert_eq!(
                    (1, 2),
                    Spectrum {
                        points: vec![0., 0.1, 0.2, 0.1, 0.],
                        step: 0.4,
                        x_start: 0.7,
                    }.get_indices_of_closest_to_lhs_rhs(x)
                );
            }
        }
        #[test]
        fn _5__2_3() {
            for x in [1.6, 1.7, 1.8] {
                dbg!(x);
                assert_eq!(
                    (2, 3),
                    Spectrum {
                        points: vec![0., 0.1, 0.2, 0.1, 0.],
                        step: 0.4,
                        x_start: 0.7,
                    }.get_indices_of_closest_to_lhs_rhs(x)
                );
            }
        }
        #[test]
        fn _5__3_4() {
            for x in [2., 2.1, 2.2] {
                dbg!(x);
                assert_eq!(
                    (3, 4),
                    Spectrum {
                        points: vec![0., 0.1, 0.2, 0.1, 0.],
                        step: 0.4,
                        x_start: 0.7,
                    }.get_indices_of_closest_to_lhs_rhs(x)
                );
            }
        }
    }
    #[allow(non_snake_case)]
    mod get_points_len_after_recalc_with_step {
        #[test]
        fn _2__0_2() {
            assert_eq!(
                6, // dx: 0. 0.2 0.4 0.6 0.8 1.0
                Spectrum {
                    // dx:       0.  1.
                    points: vec![0., 10.],
                    step: 1.,
                    x_start: 0.7,
                }.get_points_len_after_recalc_with_step(0.2)
            );
        }
        #[test]
        fn _2__0_199() {
            assert_eq!(
                6, // dx: 0. 0.199 0.398 0.597 0.796 0.995
                Spectrum {
                    // dx:       0.  1.
                    points: vec![0., 10.],
                    step: 1.,
                    x_start: 0.7,
                }.get_points_len_after_recalc_with_step(0.199)
            );
        }
        #[test]
        fn _2__0_201() {
            assert_eq!(
                5, // dx: 0. 0.201 0.402 0.603 0.804
                Spectrum {
                    // dx:       0.  1.
                    points: vec![0., 10.],
                    step: 1.,
                    x_start: 0.7,
                }.get_points_len_after_recalc_with_step(0.201)
            );
        }
        #[test]
        fn _3__0_2() {
            assert_eq!(
                11, // dx: 0. 0.2 0.4 0.6 0.8 1. 1.2 1.4 1.6 1.8 2.0
                Spectrum {
                    // dx:       0.  1.   2.
                    points: vec![0., 10., 20.],
                    step: 1.,
                    x_start: 0.7,
                }.get_points_len_after_recalc_with_step(0.2)
            );
        }
        #[test]
        fn _3__0_199() {
            assert_eq!(
                11,
                Spectrum {
                    // dx:       0.  1.   2.
                    points: vec![0., 10., 20.],
                    step: 1.,
                    x_start: 0.7,
                }.get_points_len_after_recalc_with_step(0.199)
            );
        }
        #[test]
        fn _3__0_201() {
            assert_eq!(
                10,
                Spectrum {
                    // dx:       0.  1.   2.
                    points: vec![0., 10., 20.],
                    step: 1.,
                    x_start: 0.7,
                }.get_points_len_after_recalc_with_step(0.201)
            );
        }
        #[test]
        fn _4__0_2() {
            assert_eq!(
                16, // dx: 0. 0.2 0.4 0.6 0.8 1. 1.2 1.4 1.6 1.8 2. 2.2 2.4 2.6 2.8 3.0
                Spectrum {
                    // dx:       0.  1.   2.   3.
                    points: vec![0., 10., 20., 30.],
                    step: 1.,
                    x_start: 0.7,
                }.get_points_len_after_recalc_with_step(0.2)
            );
        }
        #[test]
        fn _4__0_199() {
            assert_eq!(
                16,
                Spectrum {
                    // dx:       0.  1.   2.   3.
                    points: vec![0., 10., 20., 30.],
                    step: 1.,
                    x_start: 0.7,
                }.get_points_len_after_recalc_with_step(0.199)
            );
        }
        #[test]
        fn _4__0_201() {
            assert_eq!(
                15,
                Spectrum {
                    // dx:       0.  1.   2.   3.
                    points: vec![0., 10., 20., 30.],
                    step: 1.,
                    x_start: 0.7,
                }.get_points_len_after_recalc_with_step(0.201)
            );
        }
        use super::*;
        #[test]
        fn _5__0_2() {
            assert_eq!(
                9,
                Spectrum {
                    points: vec![0., 0.2, 0.4, 0.2, 0.],
                    step: 0.4,
                    x_start: 0.7,
                }.get_points_len_after_recalc_with_step(0.2)
            );
        }
        #[test]
        fn _5__0_199() {
            assert_eq!(
                9,
                Spectrum {
                    points: vec![0., 0.2, 0.4, 0.2, 0.],
                    step: 0.4,
                    x_start: 0.7,
                }.get_points_len_after_recalc_with_step(0.199)
            );
        }
        #[test]
        fn _5__0_201() {
            assert_eq!(
                8,
                Spectrum {
                    points: vec![0., 0.2, 0.4, 0.2, 0.],
                    step: 0.4,
                    x_start: 0.7,
                }.get_points_len_after_recalc_with_step(0.201)
            );
        }
    }
    mod recalculate_with_step {
        use super::*;
        #[test]
        fn _5_into_9() {
            assert_eq!(
                Spectrum {
                    // points: vec![0., 0.1, 0.2, 0.3, 0.4, 0.3, 0.2, 0.1, 0.],
                    points: vec![0., 0.09999999999999998, 0.2, 0.3, 0.4, 0.30000000000000004, 0.2, 0.10000000000000003, 1.5543122344752193e-16],
                    step: 0.2,
                    x_start: 0.7,
                },
                Spectrum {
                    points: vec![0., 0.2, 0.4, 0.2, 0.],
                    step: 0.4,
                    x_start: 0.7,
                }.recalculated_with_step(0.2)
            );
        }
        #[test]
        fn _2_into_6() {
            assert_eq!(
                Spectrum {
                    // dx:       0. 0.2 0.4 0.6 0.8 1.0
                    // points: vec![0., 2., 4., 6., 8., 10.],
                    points: vec![0., 1.9999999999999996, 4.000000000000002, 6.000000000000001, 8., 10.],
                    step: 0.2,
                    x_start: 0.7,
                },
                Spectrum {
                    // dx:       0.  1.
                    points: vec![0., 10.],
                    step: 1.,
                    x_start: 0.7,
                }.recalculated_with_step(0.2)
            );
        }
        #[test]
        fn _9_into_4() {
            assert_eq!(
                Spectrum {
                    points: vec![0., 0.3, 0.8999999999999997, 0.2, 0.],
                    step: 0.8,
                    x_start: 0.7,
                },
                Spectrum {
                    points: vec![0., 0.1, 0.3, 0.5, 0.9, 0.6, 0.2, 0.1, 0.],
                    step: 0.4,
                    x_start: 0.7,
                }.recalculated_with_step(0.8)
            );
        }
    }
}

