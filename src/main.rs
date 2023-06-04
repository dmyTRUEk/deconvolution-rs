//! Deconvolution.

use std::{
    env,
    fs::File,
    io::{Write, BufReader, BufRead},
    path::Path,
};

use rand::{Rng, rngs::ThreadRng, thread_rng};

mod aliases_method_to_function;
mod antispikes;
mod convolution;
mod deconvolution;
mod deconvolution_data;
mod diff_function;
mod exponent_function;
mod extensions;
mod fit_algorithm;
mod float_type;
mod spectrum;
mod utils_io;

use deconvolution::Deconvolution;
use deconvolution_data::DeconvolutionData;
use extensions::ToStringUnderscoreSeparated;
use fit_algorithm::FitResult;
use float_type::float;
use spectrum::Spectrum;
use utils_io::flush;


mod deconvolution_params {
    use crate::diff_function::DiffFunction;
    use super::{Deconvolution, float};

    pub const EXPONENTS_AMOUNT: usize = 2; // only for `Deconvolution::Exponents`.

    pub const DECONVOLUTION: Deconvolution = {

        // Deconvolution::PerPoint {
        //     diff_function_type: DiffFunctionType::DySqr,
        //     // antispikes: None,
        //     antispikes: Some(Antispikes {
        //         antispikes_type: AntispikesType::DySqr,
        //         antispikes_k: 1.,
        //     }),
        //     initial_value: 0.,
        // }

        // Deconvolution::Exponents {
        //     diff_function_type: DiffFunctionType::DySqr,
        //     // exponents_amount: 2,
        //     initial_values: [
        //         // 30., -10., 30.,
        //         // 30., 1., -2.,
        //         1., 1., 1.,
        //         1., 1., 1.,
        //     ],
        // }

        // Deconvolution::SatExp_DecExp {
        //     // WARNING?: set `FIT_ALGORITHM_MIN_STEP` to `1e-3`.
        //     diff_function_type: DiffFunctionType::DySqr,
        //     //               a   s   t1  t2
        //     // initial_values: [1., 0., 1., 1.],
        //     // initial_values: [0.04, -12., 1., 30.], // fr: 1.870
        //     // initial_values: [1., 1., 1., 1.],
        //     initial_values: [0.1, -10., 1., 10.],
        // }

        // Deconvolution::Two_SatExp_DecExp {
        //     // WARNING?: set `FIT_ALGORITHM_MIN_STEP` to `1e-3`.
        //     diff_function_type: DiffFunctionType::DySqr,
        //     initial_values: [
        //         100., -10., 100., 10.,
        //         100., -10., 100., 10.,
        //     ],
        // }

        // Deconvolution::SatExp_DecExpPlusConst {
        //     diff_function_type: DiffFunctionType::DySqr,
        //     initial_values: [0.1, -1., 1e-2, 0.1, 10.],
        //     allow_tb_less_than_ta: false,
        // }

        // Deconvolution::SatExp_TwoDecExp {
        //     diff_function_type: DiffFunctionType::DySqr,
        //     // initial_values: [0.1, -10., 0.01, 1., 1.],
        //     initial_values: [0.02, -9., 6e-6, 35., 8.],
        // }

        // Deconvolution::SatExp_TwoDecExpPlusConst {
        //     diff_function_type: DiffFunctionType::DySqr,
        //     // initial_values: [0.1, -5., 1e-2, 0.1, 10., 20.],    // ../data3/AIS3col_e650.dat
        //     initial_values: [0.01, -10., 0.1, 5e-3, 5., 20.],      // ../data3/AIS3fil_e650.dat
        //     // initial_values: [0.03, -8., 0.085, 5e-3, 6., 18.3], // ../data3/AIS3fil_e650.dat
        // }

        Deconvolution::SatExp_TwoDecExp_SeparateConsts {
            diff_function_type: DiffFunction::DySqr,
            // initial_values: [0.1, -10., 0.01, 1., 1.],
            initial_values: [0.02, 0.02, -9., 0.1, 35., 100.],
        }

    };

    pub const TRY_RANDOMIZED_INITIAL_VALUES: bool = false;
    pub const INITIAL_VALUES_RANDOM_SCALE: float = 10.;
    pub const CHANGE_SING_PROBABILITY: float = 0.05;
}

mod output_params {
    pub const SIGNIFICANT_DIGITS: usize = 4;
}

mod fit_params {
    use super::{fit_algorithm::FitAlgorithm, float};
    // pub const INITIAL_VALUES: float = 0.0015;
    pub const FIT_ALGORITHM_TYPE: FitAlgorithm = FitAlgorithm::PatternSearch;
    // pub const FIT_RESIDUE_GOAL   : float = 1e-1; // for Pattern Search
    pub const FIT_ALGORITHM_MIN_STEP: float = 1e-4; // for Pattern Search & Downhill Simplex
    pub const FIT_RESIDUE_EVALS_MAX : u64 = 1_000_000;
    pub const FIT_RESIDUE_MAX_VALUE : float = 1e6;
}

mod pattern_search_params {
    use super::float;
    pub const INITIAL_STEP: float = 1.;
    pub const ALPHA: float = 1.1;        // step increase coefficient
    pub const BETA : float = 1. / ALPHA; // step decrease coefficient
}

mod downhill_simplex_params {
    use super::{diff_function::DiffFunction, float};
    pub const INITIAL_SIMPLEX_SCALE: float = 0.815;
    pub const PARAMS_DIFF_TYPE: DiffFunction = DiffFunction::DySqr;
}


fn main() {
    let args: Vec<_> = env::args().collect();
    let (filepathstr_instrument, filepathstr_measured): (&str, &str) = match &args[..] {
        [_, filepathstr_instrument, filepathstr_measured] => (filepathstr_instrument, filepathstr_measured),
        [_, _] => panic!("Expected two filename, provided only one."),
        [_] => panic!("Filenames not provided."),
        [] => unreachable!("Unexpected CLI args number."),
        _ => panic!("Too many CLI args.") // TODO(feat): support multiple files to deconvolve.
    };

    print!("Loading instrumental spectrum  from `{}`...", filepathstr_instrument); flush();
    let instrument = Spectrum::load_from_file(filepathstr_instrument);
    println!(" done");

    print!("Loading spectrum to deconvolve from `{}`...", filepathstr_measured); flush();
    let measured = Spectrum::load_from_file(filepathstr_measured);
    println!(" done");

    // TODO: warning if points in instr more than in spectrum.
    // assert!(measured.points.len() > instrument.points.len());

    println!("FIT_ALGORITHM_TYPE    : {:#?}", fit_params::FIT_ALGORITHM_TYPE);
    println!("FIT_ALGORITHM_MIN_STEP: {:.2e}", fit_params::FIT_ALGORITHM_MIN_STEP);
    // if fit_params::FIT_ALGORITHM_TYPE == FitAlgorithmType::PatternSearch {
    //     println!("FIT_RESIDUE_GOAL     : {:.2e}", fit_params::FIT_RESIDUE_GOAL);
    // }
    println!("FIT_RESIDUE_EVALS_MAX : {}", fit_params::FIT_RESIDUE_EVALS_MAX.to_string_underscore_separated());

    let file_instrument = Path::new(filepathstr_instrument);
    let file_spectrum   = Path::new(filepathstr_measured);
    assert_eq!(
        file_instrument.parent().unwrap().canonicalize().unwrap().to_str().unwrap(),
        file_spectrum  .parent().unwrap().canonicalize().unwrap().to_str().unwrap()
    );
    const FILENAME_PREFIX: &str = "result";
    let filepath_output = file_spectrum.with_file_name(format!(
        "{FILENAME_PREFIX}_{}_{}.dat",
        file_instrument.file_stem().unwrap().to_str().unwrap(),
        file_spectrum.file_stem().unwrap().to_str().unwrap()
    ));
    let filepathstr_output: &str = filepath_output.to_str().unwrap();
    let filepath_output_convolved = file_spectrum.with_file_name(format!(
        "{FILENAME_PREFIX}_{}_{}_convolved.dat",
        file_instrument.file_stem().unwrap().to_str().unwrap(),
        file_spectrum.file_stem().unwrap().to_str().unwrap()
    ));
    let filepathstr_output_convolved: &str = filepath_output_convolved.to_str().unwrap();

    let deconvolution = deconvolution_params::DECONVOLUTION;

    let deconvolution_data: DeconvolutionData = DeconvolutionData {
        instrument,
        measured,
        deconvolution,
    }.aligned_steps_to_smaller();


    println!();
    let fit_residue_with_initial_values = deconvolution_data.calc_residue_function(&deconvolution_data.get_initial_params());
    println!("fit_residue @ initial_values: {}", fit_residue_with_initial_values);
    println!();

    let deconvolve_results = deconvolution_data.deconvolve(fit_params::FIT_ALGORITHM_TYPE);
    match deconvolve_results {
        Err(err) => println!("ERROR: {}", err),
        Ok(ref deconvolution_results_unwrapped) => {
            output_results(
                &deconvolution_data,
                deconvolution_results_unwrapped,
                filepathstr_output,
                filepathstr_output_convolved,
            );
        }
    }
    if !deconvolution_params::TRY_RANDOMIZED_INITIAL_VALUES { return }

    println!();
    println!("------- NOW TRYING RANDOM INITIAL VALUES -------");
    println!();

    let mut rng = thread_rng();
    let mut best_fit_residue: float = if deconvolve_results.is_ok() { deconvolve_results.unwrap().fit_residue } else { float::MAX };
    let mut initial_values_tried: u64 = 0;
    loop {
        initial_values_tried += 1;
        let mut deconvolution_data = deconvolution_data.clone();
        fn randomize_array<const N: usize>(array: &mut [float; N], rng: &mut ThreadRng) {
            for i in 0..N {
                let is_change_sign: bool = rng.gen_bool(deconvolution_params::CHANGE_SING_PROBABILITY);
                let random_scale: float = rng.gen_range(
                    1./deconvolution_params::INITIAL_VALUES_RANDOM_SCALE ..= deconvolution_params::INITIAL_VALUES_RANDOM_SCALE
                );
                array[i] *= if is_change_sign { -1. } else { 1. } * random_scale;
            }
        }
        match deconvolution_data.deconvolution {
            Deconvolution::PerPoint { .. } => panic!("there is no need to try different initial params"),
            Deconvolution::Exponents { ref mut initial_values, .. } => randomize_array(initial_values, &mut rng),
            Deconvolution::SatExp_DecExp { ref mut initial_values, .. } => randomize_array(initial_values, &mut rng),
            Deconvolution::Two_SatExp_DecExp { ref mut initial_values, .. } => randomize_array(initial_values, &mut rng),
            Deconvolution::SatExp_DecExpPlusConst { ref mut initial_values, .. } => randomize_array(initial_values, &mut rng),
            Deconvolution::SatExp_TwoDecExp { ref mut initial_values, .. } => randomize_array(initial_values, &mut rng),
            Deconvolution::SatExp_TwoDecExpPlusConst { ref mut initial_values, .. } => randomize_array(initial_values, &mut rng),
            Deconvolution::SatExp_TwoDecExp_SeparateConsts { ref mut initial_values, .. } => randomize_array(initial_values, &mut rng),
            Deconvolution::Fourier {} => unimplemented!(),
        }
        let deconvolution_results = deconvolution_data.deconvolve(fit_params::FIT_ALGORITHM_TYPE);
        match deconvolution_results {
            Ok(deconvolution_results_unwrapped) if deconvolution_results_unwrapped.fit_residue < best_fit_residue => {
                best_fit_residue = deconvolution_results_unwrapped.fit_residue;
                println!("{}", "-".repeat(42));
                println!("initial_values_tried: {}", initial_values_tried);
                // let Deconvolution::Exponents { initial_values, .. } = deconvolution_data.deconvolution else { unreachable!() };
                // dbg!(initial_values);
                output_results(
                    &deconvolution_data,
                    &deconvolution_results_unwrapped,
                    filepathstr_output,
                    filepathstr_output_convolved,
                );
                println!("{}", "-".repeat(42));
            }
            _ => {
                println!(
                    "fit_residue: {}",
                    deconvolution_results.as_ref()
                        .map(|dr| format!("{:.4}", dr.fit_residue))
                        .unwrap_or_else(|err| format!("Error: {err}"))
                );
            },
        }
    }
}


fn output_results(
    deconvolution_data: &DeconvolutionData,
    deconvolution_results: &FitResult,
    filepathstr_output: &str,
    filepathstr_output_convolved: &str,
) {
    println!("deconvolution_results = {deconvolution_results:#?}");
    // println!("fit_residue_evals = {}", deconvolution_results.fit_residue_evals.to_string_underscore_separated());

    let params = &deconvolution_results.params;

    let desmos_function_str = deconvolution_data.deconvolution.to_desmos_function(&params);
    if let Ok(ref desmos_function_str) = desmos_function_str {
        println!("{}", desmos_function_str);
    }

    // let mut file_output = File::create(filepath_output).unwrap();
    // assert_eq!((1010..=1089).count(), deconvolve_results.points.len());
    // // TODO(refactor): `zip_exact`.
    // for (x, point) in (1010..=1089).zip(deconvolve_results.points) {
    //     writeln!(file_output, "{x}\t{p}", p=point).unwrap();
    // }
    let fit_residue_and_evals_msg = format!(
        "fit residue {fr:.3} achieved in {fre} fit residue function evals",
        fr=deconvolution_results.fit_residue,
        fre=deconvolution_results.fit_residue_evals,
    );
    // TODO(refactor):
    // - extract function name into separate method?
    // - extract common logic
    match &deconvolution_data.deconvolution {
        Deconvolution::PerPoint { .. } => {
            let sd_deconvolved = Spectrum {
                points: deconvolution_results.params.clone(),
                step: deconvolution_data.get_step(),
                x_start: deconvolution_data.measured.x_start,
            };
            sd_deconvolved.write_to_file(filepathstr_output);
            todo!("also print `fit_residue` and `fit_residue_evals`");
        }
        self_ @ Deconvolution::Exponents { .. } => {
            let mut file_output = File::create(filepathstr_output).unwrap();
            writeln!(file_output, "{name} params ({fit_residue_and_evals_msg}):", name=self_.get_name()).unwrap();
            for parts in deconvolution_results.params.chunks(3) {
                let (amplitude, shift, tau) = (parts[0], parts[1], parts[2]);
                writeln!(file_output, "amplitude={amplitude}").unwrap();
                writeln!(file_output, "shift={shift}").unwrap();
                writeln!(file_output, "tau={tau}").unwrap();
            }
            if let Ok(desmos_function_str) = desmos_function_str {
                writeln!(file_output, "{desmos_function_str}").unwrap();
            }
        }
        self_ @ Deconvolution::SatExp_DecExp { .. } => {
            let mut file_output = File::create(filepathstr_output).unwrap();
            writeln!(file_output, "{name} params ({fit_residue_and_evals_msg}):", name=self_.get_name()).unwrap();
            let (amplitude, shift, tau_a, tau_b) = (params[0], params[1], params[2], params[3]);
            writeln!(file_output, "amplitude={amplitude}").unwrap();
            writeln!(file_output, "shift={shift}").unwrap();
            writeln!(file_output, "tau_a={tau_a}").unwrap();
            writeln!(file_output, "tau_b={tau_b}").unwrap();
            if let Ok(desmos_function_str) = desmos_function_str {
                writeln!(file_output, "{desmos_function_str}").unwrap();
            }
        }
        self_ @ Deconvolution::Two_SatExp_DecExp { .. } => {
            let mut file_output = File::create(filepathstr_output).unwrap();
            writeln!(file_output, "{name} params ({fit_residue_and_evals_msg}):", name=self_.get_name()).unwrap();
            let (amplitude_1, shift_1, tau_a1, tau_b1) = (params[0], params[1], params[2], params[3]);
            let (amplitude_2, shift_2, tau_a2, tau_b2) = (params[4], params[5], params[6], params[7]);
            writeln!(file_output, "amplitude_1={amplitude_1}").unwrap();
            writeln!(file_output, "shift_1={shift_1}").unwrap();
            writeln!(file_output, "tau_a1={tau_a1}").unwrap();
            writeln!(file_output, "tau_b1={tau_b1}").unwrap();
            writeln!(file_output, "amplitude_2={amplitude_2}").unwrap();
            writeln!(file_output, "shift_2={shift_2}").unwrap();
            writeln!(file_output, "tau_a2={tau_a2}").unwrap();
            writeln!(file_output, "tau_b2={tau_b2}").unwrap();
            if let Ok(desmos_function_str) = desmos_function_str {
                writeln!(file_output, "{desmos_function_str}").unwrap();
            }
        }
        self_ @ Deconvolution::SatExp_DecExpPlusConst { .. } => {
            let mut file_output = File::create(filepathstr_output).unwrap();
            writeln!(file_output, "{name} params ({fit_residue_and_evals_msg}):", name=self_.get_name()).unwrap();
            let (amplitude, shift, height, tau_a, tau_b) = (params[0], params[1], params[2], params[3], params[4]);
            writeln!(file_output, "amplitude={amplitude}").unwrap();
            writeln!(file_output, "shift={shift}").unwrap();
            writeln!(file_output, "height={height}").unwrap();
            writeln!(file_output, "tau_a={tau_a}").unwrap();
            writeln!(file_output, "tau_b={tau_b}").unwrap();
            if let Ok(desmos_function_str) = desmos_function_str {
                writeln!(file_output, "{desmos_function_str}").unwrap();
            }
        }
        self_ @ Deconvolution::SatExp_TwoDecExp { .. } => {
            let mut file_output = File::create(filepathstr_output).unwrap();
            writeln!(file_output, "{name} params ({fit_residue_and_evals_msg}):", name=self_.get_name()).unwrap();
            let (amplitude, shift, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4]);
            writeln!(file_output, "amplitude={amplitude}").unwrap();
            writeln!(file_output, "shift={shift}").unwrap();
            writeln!(file_output, "tau_a={tau_a}").unwrap();
            writeln!(file_output, "tau_b={tau_b}").unwrap();
            writeln!(file_output, "tau_c={tau_c}").unwrap();
            if let Ok(desmos_function_str) = desmos_function_str {
                writeln!(file_output, "{desmos_function_str}").unwrap();
            }
        }
        self_ @ Deconvolution::SatExp_TwoDecExpPlusConst { .. } => {
            let mut file_output = File::create(filepathstr_output).unwrap();
            writeln!(file_output, "{name} params ({fit_residue_and_evals_msg}):", name=self_.get_name()).unwrap();
            let (amplitude, shift, height, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4], params[5]);
            writeln!(file_output, "amplitude={amplitude}").unwrap();
            writeln!(file_output, "shift={shift}").unwrap();
            writeln!(file_output, "height={height}").unwrap();
            writeln!(file_output, "tau_a={tau_a}").unwrap();
            writeln!(file_output, "tau_b={tau_b}").unwrap();
            writeln!(file_output, "tau_c={tau_c}").unwrap();
            if let Ok(desmos_function_str) = desmos_function_str {
                writeln!(file_output, "{desmos_function_str}").unwrap();
            }
        }
        self_ @ Deconvolution::SatExp_TwoDecExp_SeparateConsts { .. } => {
            let mut file_output = File::create(filepathstr_output).unwrap();
            writeln!(file_output, "{name} params ({fit_residue_and_evals_msg}):", name=self_.get_name()).unwrap();
            let (amplitude_b, amplitude_c, shift, tau_a, tau_b, tau_c) = (params[0], params[1], params[2], params[3], params[4], params[5]);
            writeln!(file_output, "amplitude_b={amplitude_b}").unwrap();
            writeln!(file_output, "amplitude_c={amplitude_c}").unwrap();
            writeln!(file_output, "shift={shift}").unwrap();
            writeln!(file_output, "tau_a={tau_a}").unwrap();
            writeln!(file_output, "tau_b={tau_b}").unwrap();
            writeln!(file_output, "tau_c={tau_c}").unwrap();
            if let Ok(desmos_function_str) = desmos_function_str {
                writeln!(file_output, "{desmos_function_str}").unwrap();
            }
        }
        Deconvolution::Fourier {} => unimplemented!(),
    }

    let convolved_points: Vec<float> = deconvolution_data.convolve_from_params(&deconvolution_results.params);
    let convolved = Spectrum {
        points: convolved_points,
        x_start: deconvolution_data.measured.x_start,
        step: deconvolution_data.measured.step,
    };
    convolved.write_to_file(filepathstr_output_convolved);
}


pub fn load_data_y(filename: &str) -> Vec<float> {
    let file = File::open(filename).expect(&format!("Unable to open file: `{}`", filename));
    let lines = BufReader::new(file).lines();
    let mut points = Vec::<float>::with_capacity(20);
    for line in lines {
        let line = line.unwrap();
        let line = line.trim();
        if line == "" { continue }
        let (_, y) = line.split_once(&[' ', '\t']).unwrap();
        let y = y.trim();
        let y = y.replace(",", ".");
        let y = y.parse().unwrap();
        points.push(y);
    }
    points
}

