//! Main file of deconvolution-rs.

#![feature(
    array_chunks,
    array_windows,
    trait_alias,
)]

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

mod aliases_method_to_function;
mod antispikes;
mod config;
mod deconvolution;
mod diff_function;
mod exponent_function;
mod extensions;
mod fit_algorithms;
mod float_type;
mod load;
mod macros;
mod spectrum;
mod stacktrace;
mod utils_io;

use config::Config;
use deconvolution::deconvolution_data::DeconvolutionData;
use extensions::ToStringUnderscoreSeparated; // TODO: use
use fit_algorithms::Fit;
use float_type::float;
use spectrum::Spectrum;
use utils_io::flush;


fn main() {
    let config = Config::load_from_default_file();

    let cli_args: Vec<_> = env::args().collect();
    match cli_args.as_slice() {
        [_, _] => panic!("Expected at least two filenames (instrumental & measured), provided only one."),
        [_] => panic!("Expected at least two filenames (instrumental & measured), provided zero."),
        [] => unreachable!("Unexpected CLI args number."),
        _ => {}
    }
    let filepathstr_instrument: &str = &cli_args[1];

    print!("Loading instrumental spectrum  from `{}`...", filepathstr_instrument); flush();
    let instrument = Spectrum::load_from_file_as_instrumental(filepathstr_instrument);
    let filepathstr_instrument_stem = Path::new(filepathstr_instrument)
        .file_stem().unwrap().to_str().unwrap();
    println!(" done");

    for filepathstr_measured in cli_args[2..].iter() {
        process_measured_file(
            &config,
            instrument.clone(),
            filepathstr_instrument_stem,
            filepathstr_measured,
        );
    }
}


fn process_measured_file(
    config: &Config,
    instrument: Spectrum,
    filepathstr_instrument_stem: &str,
    filepathstr_measured: &str,
) {
    print!("Loading spectrum to deconvolve from `{}`...", filepathstr_measured); flush();
    let measured = Spectrum::load_from_file(filepathstr_measured);
    println!(" done");

    // TODO: warning if points in instr more than in spectrum.
    // assert!(measured.points.len() > instrument.points.len());

    println!("Fit Algorithm = {:#?}", config.fit_algorithm);
    // TODO: fit_algorithm.max_evals.to_string_underscore_separated

    let file_spectrum = Path::new(filepathstr_measured);
    let filepathstr_spectrum_stem = file_spectrum.file_stem().unwrap().to_str().unwrap();

    // TODO: fix: dont work on win7?
    // assert_eq!(
    //     file_instrument.parent().unwrap().canonicalize().unwrap().to_str().unwrap(),
    //     file_spectrum  .parent().unwrap().canonicalize().unwrap().to_str().unwrap()
    // );

    const FILENAME_PREFIX: &str = "result";

    let build_filepathstr_output = |randomized_initial_values_i: u64| -> String {
        let filepath_output = file_spectrum.with_file_name(format!(
            "{FILENAME_PREFIX}_{}_{}{}.dat",
            filepathstr_instrument_stem,
            filepathstr_spectrum_stem,
            if randomized_initial_values_i == 0 { "".to_string() } else { format!("_riv{}", randomized_initial_values_i) },
        ));
        let filepathstr_output: String = filepath_output.to_str().unwrap().to_string();
        filepathstr_output
    };

    let build_filepathstr_output_convolved = |randomized_initial_values_i: u64| -> String {
        let filepath_output_convolved = file_spectrum.with_file_name(format!(
            "{FILENAME_PREFIX}_{}_{}{}_convolved.dat",
            filepathstr_instrument_stem,
            filepathstr_spectrum_stem,
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
    }.aligned_steps_to(config.input_params.align_step_to);

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

    let mut best_fit_residue: float = if deconvolve_results.is_ok() { deconvolve_results.unwrap().fit_residue } else { float::MAX };
    for randomized_initial_values_i in 1..=config.deconvolution_params.try_randomized_initial_values {
        let mut deconvolution_data = deconvolution_data.clone();
        deconvolution_data.deconvolution.randomize(config.deconvolution_params.initial_values_random_scale);
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
    deconvolution_results: &Fit,
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
    deconvolution_data.write_result_to_file(
        deconvolution_results,
        filepathstr_output,
        desmos_function_str,
        &fit_residue_and_evals_msg,
        params,
    );

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

