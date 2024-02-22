//! Convolution.

use crate::types::{
    float::float,
    linalg::DVect,
    named_wrappers::{ConvolvedV, DeconvolvedV, InstrumentRevV},
};


// /// Must be used only in `tests` & `DeconvolutionData::convolve()`.
// #[deprecated = "use `convolve_by_points_v` instead"]
// pub(super) fn convolve_by_points( // TODO: `pub(super)`.
//     instrument_rev: &InstrumentRev,
//     deconvolved: Deconvolved,
// ) -> Convolved {
//     Convolved(
//         convolve_by_points_v(
//             instrument_rev.into(),
//             DeconvolvedV::from(deconvolved),
//         ).0.data.as_vec().to_vec()
//     )
// }



/// Must be used only in `tests` & `DeconvolutionData::convolve()`.
pub(super) fn convolve_by_points_v( // TODO: `pub(super)`.
    instrument_rev: &InstrumentRevV,
    deconvolved: DeconvolvedV,
) -> ConvolvedV {
    let instrument_len: usize = instrument_rev.0.len();
    assert!(instrument_len % 2 == 1, "instrument_len = {}", instrument_len); // why?
    // let instrument_rev: DVect = instrument.rows(0, instrument_len).reversed();
    // assert_eq!(instrument.len(), instrument_rev.len());
    let ilh = instrument_len / 2;
    let deconvolved_len: usize = deconvolved.0.len();
    let convolved_len = deconvolved_len;
    let mut convolved: DVect = DVect::zeros(convolved_len);
    for i in 0_usize..convolved_len {
        // println!("i={i}");

        // TODO?: support case when `instrument` have more points than `deconvolved`.

        let d_first_index = i.saturating_sub(ilh);
        let d_last_index = (i+ilh+1).min(deconvolved_len);
        // println!("d_first_index={d_first_index}, d_last_index={d_last_index}");

        // iterative approach:
        // let mut convolved_: float = 0.;
        // for di in d_first_index..d_last_index { // deconvolved_index
        //     let ii = (i + ilh) - di; // instrument_index
        //     // println!("di={di}, ii={ii}");
        //     let instrument  = instrument [ii];
        //     let deconvolved = deconvolved[di];
        //     convolved_ += instrument * deconvolved;
        // }

        // vector approach:
        let slice_len: usize = d_last_index - d_first_index;
        let deconvolved_slice = deconvolved.0.rows(d_first_index, slice_len);
        // dbg!(deconvolved_slice.shape());

        let i_first_index = (i + ilh + 1) - d_last_index;
        // let i_first_index = (i.saturating_sub(ilh)+1).clamp(0, instrument_len);
        // println!("i_first_index={i_first_index}");
        // let instrument_slice = instrument.rows(i_first_index, slice_len);

        // let instrument_slice_rev = DVect::from_iterator(slice_len, instrument_slice.iter().rev().copied());
        // let instrument_slice_rev = instrument_slice.reversed();

        //     i - - - -       => i_first_index = 2, slice_len = 5, slice = [2,3,4,5,6]
        // 0 1 2 3 4 5 6 7 8 9 => len = 10
        // 9 8 7 6 5 4 3 2 1 0
        //       - - - - i     => i_last_index = 7
        //       i - - - -     => i_first_index_rev = 3 = 10 - 2 - 5
        let i_first_index_rev = instrument_len - i_first_index - slice_len;
        let instrument_slice_rev = instrument_rev.0.rows(i_first_index_rev, slice_len);
        // dbg!(instrument_slice_rev.shape());

        let convolved__: float = deconvolved_slice.dot(&instrument_slice_rev);

        // assert_eq!(convolved_, convolved__);

        convolved[i] = convolved__;
        // println!();
    }
    ConvolvedV(convolved)
}



#[cfg(test)]
mod convolve {
    #![allow(non_snake_case)]

    use crate::types::float::float;

    fn convolve(instrument: &Vec<float>, deconvolved: &Vec<float>) -> Vec<float> {
        use crate::types::named_wrappers::{Deconvolved, Instrument};
        use super::convolve_by_points_v;
        convolve_by_points_v(
            &Instrument(instrument.to_vec()).into(),
            Deconvolved(deconvolved.to_vec()).into(),
        ).0.data.as_vec().to_vec()
    }

    mod per_point {
        use super::*;
        use crate::{diff_function::DiffFunction, float};
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
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
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
        }
        mod custom {
            use super::*;
            mod ones_len_is_3 {
                use super::*;
                const POINTS_INSTRUMENT: [float; 9] = [0., 0., 0., 1., 1., 1., 0., 0., 0.];
                const EPSILON: float = 1e-6;
                #[ignore]
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 3], vec![1.; 3], vec![0.; 3]].concat();
                    // let points_convolved_expected = [vec![0.; 6], vec![1., 2., 3., 2., 1.], vec![0.; 6]].concat();
                    let points_convolved_expected = [vec![0.; 2], vec![1., 2., 3., 2., 1.], vec![0.; 2]].concat();
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
            }
            mod ones_len_is_10 {
                use super::*;
                const POINTS_INSTRUMENT: [float; 5] = [0., 0.1, 1., 0.5, 0.2];
                const EPSILON: float = 1e-6;
                #[ignore]
                #[test]
                fn at_center() {
                    println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                    let points_spectrum_original = [vec![0.; 5], vec![1.; 10], vec![0.; 5]].concat();
                    // let points_convolved_expected = [vec![0.; 6], vec![0.1, 0.1+1., 0.1+1.+0.5], vec![0.1+1.+0.5+0.2; 7], vec![1.+0.5+0.2, 0.5+0.2, 0.2], vec![0.; 1]].concat();
                    let points_convolved_expected = [vec![0.; 4], vec![0.1, 0.1+1., 0.1+1.+0.5], vec![0.1+1.+0.5+0.2; 7], vec![1.+0.5+0.2, 0.5+0.2, 0.2], vec![0.; 3]].concat();
                    let points_convolved_actual = convolve(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                    println!("points_convolved_expected = {:?}", points_convolved_expected);
                    println!("points_convolved_actual   = {:?}", points_convolved_actual);
                    let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                    println!("diff = {}", diff);
                    assert!(diff < 0.1);
                    assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                }
                // #[test]
                // fn at_left() {
                //     println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                //     let points_spectrum_original = [vec![1.; 10], vec![0.; 5]].concat();
                //     let points_convolved_expected = [vec![0.1+1.+0.5+0.2; 10], vec![1.+0.5+0.2, 0.5+0.2, 0.2], vec![0.; 2]].concat();
                //     let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                //     println!("points_convolved_expected = {:?}", points_convolved_expected);
                //     println!("points_convolved_actual   = {:?}", points_convolved_actual);
                //     let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                //     println!("diff = {}", diff);
                //     assert!(diff < 0.1);
                //     assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                // }
                // #[test]
                // fn at_right() {
                //     println!("POINTS_INSTRUMENT = {:?}", POINTS_INSTRUMENT);
                //     let points_spectrum_original = [vec![0.; 5], vec![1.; 10]].concat();
                //     let points_convolved_expected = [vec![0.; 2], vec![0.2, 0.5+0.2, 1.+0.5+0.2], vec![0.1+1.+0.5+0.2; 10]].concat();
                //     let points_convolved_actual = convolve_by_points(&POINTS_INSTRUMENT.to_vec(), &points_spectrum_original);
                //     println!("points_convolved_expected = {:?}", points_convolved_expected);
                //     println!("points_convolved_actual   = {:?}", points_convolved_actual);
                //     let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
                //     println!("diff = {}", diff);
                //     assert!(diff < 0.1);
                //     assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
                // }
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
        //         println!("points_convolved_actual   = {:?}", points_convolved_actual);
        //         let diff = DiffFunction::DySqrPerEl.calc_diff(&points_convolved_expected, &points_convolved_actual);
        //         println!("diff = {}", diff);
        //         assert!(diff < 0.1);
        //         assert!(diff < EPSILON, "expected `diff < EPSILON, but diff={} and EPSILON={}`", diff, EPSILON);
        //     }
        // }
    }
}

