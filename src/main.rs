//! Deconvolution.

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

use rand::{Rng, rngs::ThreadRng, thread_rng};

mod aliases_method_to_function;
mod antispikes;
mod config;
mod convolution;
mod deconvolution;
mod deconvolution_data;
mod diff_function;
mod exponent_function;
mod extensions;
mod fit_algorithms;
mod float_type;
mod macros;
mod spectrum;
mod utils_io;

use config::Config;
use deconvolution::Deconvolution;
use deconvolution_data::DeconvolutionData;
use extensions::ToStringUnderscoreSeparated;
use fit_algorithms::fit_algorithm::FitResult;
use float_type::float;
use spectrum::Spectrum;
use utils_io::flush;


fn main() {
    let config = Config::load_from_default_file();

    let args: Vec<_> = env::args().collect();
    match &args[..] {
        [_, _] => panic!("Expected at least two filenames (instrumental & measured), provided only one."),
        [_] => panic!("Expected at least two filenames (instrumental & measured), provided zero."),
        [] => unreachable!("Unexpected CLI args number."),
        _ => {}
    }
    let filepathstr_instrument: &str = &args[1];

    print!("Loading instrumental spectrum  from `{}`...", filepathstr_instrument); flush();
    let instrument = Spectrum::load_from_file_as_instrumental(filepathstr_instrument);
    println!(" done");

    for filepathstr_measured in args[2..].iter() {
        process_measured_file(&config, instrument.clone(), filepathstr_instrument, filepathstr_measured);
    }
}


fn process_measured_file(
    config: &Config,
    instrument: Spectrum,
    filepathstr_instrument: &str,
    filepathstr_measured: &str,
) {
    print!("Loading spectrum to deconvolve from `{}`...", filepathstr_measured); flush();
    let measured = Spectrum::load_from_file(filepathstr_measured);
    println!(" done");

    // TODO: warning if points in instr more than in spectrum.
    // assert!(measured.points.len() > instrument.points.len());

    println!("Fit Algorithm = {:#?}", config.fit_algorithm);
    // TODO: fit_algorithm.max_evals.to_string_underscore_separated

    let file_instrument = Path::new(filepathstr_instrument);
    let file_spectrum   = Path::new(filepathstr_measured);

    // TODO:
    // assert_eq!(
    //     file_instrument.parent().unwrap().canonicalize().unwrap().to_str().unwrap(),
    //     file_spectrum  .parent().unwrap().canonicalize().unwrap().to_str().unwrap()
    // );

    const FILENAME_PREFIX: &str = "result";

    let build_filepathstr_output = |randomized_initial_values_i: u64| -> String {
        let filepath_output = file_spectrum.with_file_name(format!(
            "{FILENAME_PREFIX}_{}_{}{}.dat",
            file_instrument.file_stem().unwrap().to_str().unwrap(),
            file_spectrum.file_stem().unwrap().to_str().unwrap(),
            if randomized_initial_values_i == 0 { "".to_string() } else { format!("_riv{}", randomized_initial_values_i) },
        ));
        let filepathstr_output: String = filepath_output.to_str().unwrap().to_string();
        filepathstr_output
    };

    let build_filepathstr_output_convolved = |randomized_initial_values_i: u64| -> String {
        let filepath_output_convolved = file_spectrum.with_file_name(format!(
            "{FILENAME_PREFIX}_{}_{}{}_convolved.dat",
            file_instrument.file_stem().unwrap().to_str().unwrap(),
            file_spectrum.file_stem().unwrap().to_str().unwrap(),
            if randomized_initial_values_i == 0 { "".to_string() } else { format!("_riv{}", randomized_initial_values_i) },
        ));
        let filepathstr_output_convolved: String = filepath_output_convolved.to_str().unwrap().to_string();
        filepathstr_output_convolved
    };

    let deconvolution = config.deconvolution_function.clone();

    let deconvolution_data: DeconvolutionData = DeconvolutionData {
        instrument,
        measured,
        deconvolution,
    }.aligned_steps_to_smaller();
    // TODO: option in config to align steps to smaller or bigger

    println!();
    let fit_residue_with_initial_values = deconvolution_data.calc_residue_function(&deconvolution_data.get_initial_params());
    println!("fit_residue @ initial_values: {:.4}", fit_residue_with_initial_values);
    println!();

    let deconvolve_results = deconvolution_data.deconvolve(&config.fit_algorithm);
    match deconvolve_results {
        Err(err) => println!("ERROR: {}", err),
        Ok(ref deconvolution_results_unwrapped) => {
            output_results(
                &config,
                &deconvolution_data,
                deconvolution_results_unwrapped,
                &build_filepathstr_output(0),
                &build_filepathstr_output_convolved(0),
            );
        }
    }
    if config.deconvolution_params.try_randomized_initial_values == 0 { return }

    println!();
    println!("------- NOW TRYING RANDOM INITIAL VALUES -------");
    println!();

    let mut rng = thread_rng();
    let mut best_fit_residue: float = if deconvolve_results.is_ok() { deconvolve_results.unwrap().fit_residue } else { float::MAX };
    for randomized_initial_values_i in 1..=config.deconvolution_params.try_randomized_initial_values {
        let mut deconvolution_data = deconvolution_data.clone();
        fn randomize_array(array: &mut [float], rng: &mut ThreadRng, config: &Config) {
            for i in 0..array.len() {
                let is_change_sign: bool = rng.gen_bool(config.deconvolution_params.change_sing_probability);
                let random_scale: float = rng.gen_range(
                    1./config.deconvolution_params.initial_values_random_scale
                    ..=
                    config.deconvolution_params.initial_values_random_scale
                );
                array[i] *= if is_change_sign { -1. } else { 1. } * random_scale;
            }
        }
        match deconvolution_data.deconvolution {
            Deconvolution::PerPoint { .. } => panic!("there is no need to try different initial params"),
            Deconvolution::Exponents { ref mut initial_values, .. } => randomize_array(&mut initial_values[..], &mut rng, &config),
            Deconvolution::SatExp_DecExp { ref mut initial_values, .. } => randomize_array(initial_values, &mut rng, &config),
            Deconvolution::Two_SatExp_DecExp { ref mut initial_values, .. } => randomize_array(initial_values, &mut rng, &config),
            Deconvolution::SatExp_DecExpPlusConst { ref mut initial_values, .. } => randomize_array(initial_values, &mut rng, &config),
            Deconvolution::SatExp_TwoDecExp { ref mut initial_values, .. } => randomize_array(initial_values, &mut rng, &config),
            Deconvolution::SatExp_TwoDecExpPlusConst { ref mut initial_values, .. } => randomize_array(initial_values, &mut rng, &config),
            Deconvolution::SatExp_TwoDecExp_SeparateConsts { ref mut initial_values, .. } => randomize_array(initial_values, &mut rng, &config),
            Deconvolution::Fourier {} => unimplemented!(),
        }
        let deconvolution_results = deconvolution_data.deconvolve(&config.fit_algorithm);
        match deconvolution_results {
            Ok(deconvolution_results_unwrapped) if deconvolution_results_unwrapped.fit_residue < best_fit_residue => {
                best_fit_residue = deconvolution_results_unwrapped.fit_residue;
                println!("{}", "-".repeat(42));
                println!("initial values tried: {}", randomized_initial_values_i);
                // let Deconvolution::Exponents { initial_values, .. } = deconvolution_data.deconvolution else { unreachable!() };
                // dbg!(initial_values);
                output_results(
                    &config,
                    &deconvolution_data,
                    &deconvolution_results_unwrapped,
                    &build_filepathstr_output(randomized_initial_values_i),
                    &build_filepathstr_output_convolved(randomized_initial_values_i),
                );
                println!("{}", "-".repeat(42));
            }
            _ => {
                if !config.deconvolution_params.print_only_better_deconvolution {
                    println!(
                        "fit_residue: {}",
                        deconvolution_results.as_ref()
                            .map(|dr| format!("{:.4}", dr.fit_residue))
                            .unwrap_or_else(|err| format!("Error: {err}"))
                    );
                }
            },
        }
    }
}


fn output_results(
    config: &Config,
    deconvolution_data: &DeconvolutionData,
    deconvolution_results: &FitResult,
    filepathstr_output: &str,
    filepathstr_output_convolved: &str,
) {
    println!("deconvolution_results = {deconvolution_results:#?}");
    // println!("fit_residue_evals = {}", deconvolution_results.fit_residue_evals.to_string_underscore_separated());

    let params = &deconvolution_results.params;

    let desmos_function_str = deconvolution_data.deconvolution.to_desmos_function(
        &params,
        config.output_params.significant_digits,
    );
    if let Ok(ref desmos_function_str) = desmos_function_str {
        println!("{}", desmos_function_str);
        println!("\"fit residue: {}", deconvolution_results.fit_residue);
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
        self_ @ Deconvolution::PerPoint { .. } => {
            let sd_deconvolved = Spectrum {
                points: deconvolution_results.params.clone(),
                step: deconvolution_data.get_step(),
                x_start: deconvolution_data.measured.x_start,
            };
            let mut file_output = File::create(filepathstr_output).unwrap();
            writeln!(file_output, "{name} params ({fit_residue_and_evals_msg}):", name=self_.get_name()).unwrap();
            drop(file_output);
            sd_deconvolved.write_to_file(filepathstr_output);
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
            // let (amplitude, shift, tau_a, tau_b) = (params[0], params[1], params[2], params[3]);
            let (shift, tau_a, tau_b) = (params[0], params[1], params[2]);
            // writeln!(file_output, "amplitude={amplitude}").unwrap();
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

