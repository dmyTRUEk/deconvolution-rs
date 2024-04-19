//! Main file of deconvolution-rs.

#![feature(
    array_chunks,
    array_windows,
    const_mut_refs,
    const_option,
    const_trait_impl,
    effects,
    lint_reasons,
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
mod extensions;
mod fit_algorithms;
mod load;
mod macros;
mod spectrum;
mod stacktrace;
mod types;
mod utils_io;

use config::Config;
use deconvolution::deconvolution_data::DeconvolutionData;
use extensions::{ToStringUnderscoreSeparated, ToStringWithSignificantDigits}; // TODO: use
use fit_algorithms::Fit;
use spectrum::Spectrum;
use types::{float::float, named_wrappers::{Instrument, MeasuredV}};
use utils_io::flush;


// TODO: make `-> Result<(), TODO>`.
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
    let instrument = Spectrum::load_from_file_as_instrumental(filepathstr_instrument, config.input_params.max_step_relative_diff);
    let filepathstr_instrument_stem = Path::new(filepathstr_instrument)
        .file_stem().unwrap().to_str().unwrap();
    println!(" done");

    for filepathstr_measured in cli_args[2..].iter() {
        println!();
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
    let measured = Spectrum::load_from_file(filepathstr_measured, config.input_params.max_step_relative_diff);
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
        let riv = if randomized_initial_values_i == 0 { "".to_string() } else { format!("_riv{}", randomized_initial_values_i) };
        let filepath_output = file_spectrum.with_file_name(format!(
            "{FILENAME_PREFIX}_{filepathstr_instrument_stem}_{filepathstr_spectrum_stem}{riv}.dat"
        ));
        let filepathstr_output: String = filepath_output.to_str().unwrap().to_string();
        filepathstr_output
    };

    let build_filepathstr_output_convolved = |randomized_initial_values_i: u64| -> String {
        let riv = if randomized_initial_values_i == 0 { "".to_string() } else { format!("_riv{}", randomized_initial_values_i) };
        let filepath_output_convolved = file_spectrum.with_file_name(format!(
            "{FILENAME_PREFIX}_{filepathstr_instrument_stem}_{filepathstr_spectrum_stem}{riv}_convolved.dat",
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
    let fit_residue_with_initial_values = deconvolution_data.calc_residue_function_v(
        &deconvolution_data.get_initial_params().into(),
        &Instrument(deconvolution_data.instrument.points.clone()).into(),
        &MeasuredV(deconvolution_data.measured.points.clone().into()).into(),
    );
    println!("fit_residue @ initial_values: {:.4}", fit_residue_with_initial_values);
    println!();

    let deconvolve_results = deconvolution_data.deconvolve(&config.fit_algorithm, None);
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
        let deconvolution_results = deconvolution_data.deconvolve(
            &config.fit_algorithm,
            Some(config.deconvolution_params.initial_values_random_scale)
        );
        match deconvolution_results {
            Ok(deconvolution_results_unwrapped) if deconvolution_results_unwrapped.fit_residue < best_fit_residue => {
                best_fit_residue = deconvolution_results_unwrapped.fit_residue;
                println!("{}", "-".repeat(42));
                println!("initial values tried: {}", randomized_initial_values_i);
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
            _ if !config.deconvolution_params.print_only_better_deconvolution => {
                println!(
                    "fit_residue: {}",
                    deconvolution_results.as_ref()
                        .map(|dr| format!("{:.4}", dr.fit_residue))
                        .unwrap_or_else(|err| format!("Error: {err}"))
                );
            }
            _ => {}
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
    let significant_digits = config.output_params.significant_digits;

    let fit_residue_str = deconvolution_results.fit_residue.to_string_with_significant_digits(significant_digits);
    // let chi_squared = deconvolution_data.calc_chi_squared(deconvolution_results).to_string_with_significant_digits(significant_digits);
    let reduced_chi_square_str = deconvolution_data.calc_reduced_chi_square(deconvolution_results).to_string_with_significant_digits(significant_digits);
    let r_square = deconvolution_data.calc_r_square(deconvolution_results).to_string_with_significant_digits(significant_digits);
    let adjusted_r_square = deconvolution_data.calc_adjusted_r_square(deconvolution_results).to_string_with_significant_digits(significant_digits);

    let desmos_function_str = deconvolution_data.deconvolution.to_desmos_function(params, significant_digits);
    if let Ok(ref desmos_function_str) = desmos_function_str {
        println!("desmos function:");
        println!("{desmos_function_str}");
        println!("\"fit residue: {fit_residue_str}");
        println!("\"reduced chi squared: {reduced_chi_square_str}");
        println!("\"r square: {r_square}");
        println!("\"adjusted r square: {adjusted_r_square}");
        println!();
    }

    let origin_function_str = deconvolution_data.deconvolution.to_origin_function(params, significant_digits);
    if let Ok(ref origin_function_str) = origin_function_str {
        println!("origin function:");
        println!("{origin_function_str}");
    }

    // let mut file_output = File::create(filepath_output).unwrap();
    // assert_eq!((1010..=1089).count(), deconvolve_results.points.len());
    // // TODO(refactor): `zip_exact`.
    // for (x, point) in (1010..=1089).zip(deconvolve_results.points) {
    //     writeln!(file_output, "{x}\t{p}", p=point).unwrap();
    // }
    let fit_goodness_msg: String = [
        format!("fit goodness (achieved after {fre} fit residue function evals):", fre=deconvolution_results.fit_residue_evals),
        format!("- fit residue: {fit_residue_str}"),
        format!("- reduced chi square: {reduced_chi_square_str}"),
        format!("- r square: {r_square}"),
        format!("- adjusted r square: {adjusted_r_square}"),
    ]
        .join("\n");
    deconvolution_data.write_result_to_file(
        filepathstr_output,
        &fit_goodness_msg,
        params,
        desmos_function_str,
        origin_function_str,
        &config.fit_algorithm,
    );

    let convolved_points: Vec<float> = deconvolution_data.convolve_from_params_v(
        &deconvolution_results.params.clone().into(),
        &Instrument(deconvolution_data.instrument.points.clone()).into(),
    ).0.data.as_vec().to_vec();
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

