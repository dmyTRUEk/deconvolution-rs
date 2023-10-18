//! Config

use std::fs::read_to_string as read_file_to_string;

use toml::{
    Table as TomlTable,
    Value as TomlValue,
};

use crate::{
    deconvolution::Deconvolution,
    deconvolution_data::AlignStepsTo,
    fit_algorithms::fit_algorithm::FitAlgorithm,
    float_type::float,
};


pub trait Load {
    // TODO: return `Option/Result<Self>`
    fn load_from_toml_value(toml_value: &TomlValue) -> Self;
}


#[derive(Debug, PartialEq)]
pub struct Config {
    pub deconvolution_function: ConfigDeconvolutionFunc,
    pub deconvolution_params: ConfigDeconvolutionParams,
    pub input_params: ConfigInputParams,
    pub output_params: ConfigOutputParams,
    pub fit_algorithm: ConfigFitAlgorithmParams,
}
impl Config {
    pub fn load_from_default_file() -> Self {
        Self::load_from_file("config.toml")
    }
    pub fn load_from_file(filename: &str) -> Self {
        let text = read_file_to_string(filename)
            .expect("can't read config file");
        Self::load_from_text(&text)
    }
    fn load_from_text(text: &str) -> Self {
        let toml_table = text.parse::<TomlTable>()
            .expect("can't parse text as toml table");
        Self::load_from_toml_table(toml_table)
    }
    fn load_from_toml_table(toml_table: TomlTable) -> Self {
        let deconvolution_function = ConfigDeconvolutionFunc::load_from_toml_value(
            toml_table
                .get("deconvolution_function")
                .expect("load config: `deconvolution_function` not found")
        );
        let deconvolution_params = ConfigDeconvolutionParams::load_from_toml_value(
            toml_table
                .get("deconvolution_params")
                .expect("load config: `deconvolution_params` not found")
        );
        let input_params = ConfigInputParams::load_from_toml_value(
            toml_table
                .get("input_params")
                .expect("load config: `input_params` not found")
        );
        let output_params = ConfigOutputParams::load_from_toml_value(
            toml_table
                .get("output_params")
                .expect("load config: `output_params` not found")
        );
        let fit_algorithm_params = ConfigFitAlgorithmParams::load_from_toml_value(
            toml_table
                .get("fit_algorithm")
                .expect("load config: `fit_algorithm` not found")
        );
        Self {
            deconvolution_function,
            deconvolution_params,
            input_params,
            output_params,
            fit_algorithm: fit_algorithm_params,
        }
    }
}


pub type ConfigDeconvolutionFunc = Deconvolution;

#[derive(Debug, PartialEq)]
pub struct ConfigDeconvolutionParams {
    pub try_randomized_initial_values: u64,
    pub initial_values_random_scale: float,
    pub change_sing_probability: float,
    pub print_only_better_deconvolution: bool,
}
impl Load for ConfigDeconvolutionParams {
    fn load_from_toml_value(toml_value: &TomlValue) -> Self {
        let try_randomized_initial_values = toml_value
            .get("try_randomized_initial_values")
            .expect("deconvolution_params: `try_randomized_initial_values` not found")
            .as_integer()
            .expect("deconvolution_params -> try_randomized_initial_values: can't parse as integer");
        assert!(try_randomized_initial_values >= 0);
        let try_randomized_initial_values: u64 = try_randomized_initial_values as u64;
        let initial_values_random_scale = toml_value
            .get("initial_values_random_scale")
            .expect("deconvolution_params: `initial_values_random_scale` not found")
            .as_float()
            .expect("deconvolution_params -> initial_values_random_scale: can't parse as float");
        let change_sing_probability = toml_value
            .get("change_sing_probability")
            .expect("deconvolution_params: `change_sing_probability` not found")
            .as_float()
            .expect("deconvolution_params -> change_sing_probability: can't parse as float");
        let print_only_better_deconvolution = toml_value
            .get("print_only_better_deconvolution")
            .expect("deconvolution_params: `print_only_better_deconvolution` not found")
            .as_bool()
            .expect("deconvolution_params -> print_only_better_deconvolution: can't parse as bool");
        Self {
            try_randomized_initial_values,
            initial_values_random_scale,
            change_sing_probability,
            print_only_better_deconvolution,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ConfigInputParams {
    pub align_step_to: AlignStepsTo,
}
impl Load for ConfigInputParams {
    fn load_from_toml_value(toml_value: &TomlValue) -> Self {
        let align_step_to = AlignStepsTo::load_from_toml_value(
            toml_value
                .get("align_step_to")
                .expect("input_params: `align_step_to` not found")
        );
        Self {
            align_step_to
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ConfigOutputParams {
    pub significant_digits: u8,
}
impl Load for ConfigOutputParams {
    fn load_from_toml_value(toml_value: &TomlValue) -> Self {
        let significant_digits = toml_value
            .get("significant_digits")
            .expect("output_params: `significant_digits` not found")
            .as_integer()
            .expect("output_params -> significant_digits: can't parse as integer");
        assert!(significant_digits < 20);
        let significant_digits = significant_digits as u8;
        Self {
            significant_digits
        }
    }
}

type ConfigFitAlgorithmParams = FitAlgorithm;




#[test]
fn load_from_text() {
    use crate::{
        diff_function::DiffFunction,
        fit_algorithms::pattern_search::PatternSearch,
    };
    let config_expected = Config {
        deconvolution_function: ConfigDeconvolutionFunc::Two_SatExp_DecExp {
            diff_function_type: DiffFunction::DySqr,
            initial_values: [0.12, 296., 3.96, 6.7, 1.16, 310., 23.2, 1.79],
        },
        deconvolution_params: ConfigDeconvolutionParams {
            try_randomized_initial_values: 42,
            initial_values_random_scale: 10.,
            change_sing_probability: 0.05,
            print_only_better_deconvolution: true,
        },
        input_params: ConfigInputParams {
            align_step_to: AlignStepsTo::Smaller,
        },
        output_params: ConfigOutputParams {
            significant_digits: 4,
        },
        fit_algorithm: ConfigFitAlgorithmParams::PatternSearch(PatternSearch {
            fit_algorithm_min_step: 1e-4,
            fit_residue_evals_max: 1_000_000,
            fit_residue_max_value: 1e6,
            initial_step: 1.,
            alpha: 1.1,
            beta: None,
        }),
    };
    let config_actual = Config::load_from_text(
r#"
[deconvolution_function.Two_SatExp_DecExp]
diff_function_type = "DySqr"
initial_values = [ 0.12, 296.0, 3.96, 6.7, 1.16, 310.0, 23.2, 1.79 ]

[deconvolution_params]
try_randomized_initial_values = 42
initial_values_random_scale = 10.0
change_sing_probability = 0.05
print_only_better_deconvolution = true

[input_params]
align_step_to = "smaller"

[output_params]
significant_digits = 4

[fit_algorithm.pattern_search]
fit_algorithm_min_step = 1e-4
fit_residue_evals_max = 1_000_000
fit_residue_max_value = 1e6
initial_step = 1.0
alpha = 1.1     # step increase coefficient
# beta = 0.9    # step decrease coefficient, default = 1/alpha
"#);
    dbg!(&config_actual);
    assert_eq!(config_expected, config_actual);
}

// #[test]
// fn deconvolution_function() {
//     todo!()
// }

// #[test]
// fn deconvolution_params() {
//     todo!()
// }

// #[test]
// fn output_params() {
//     todo!()
//     // assert_eq!(
//     //     todo!(),
//     //     ConfigOutputParams::load_from_text
//     // )
// }

// #[test]
// fn fit_params() {
//     todo!()
// }

// #[test]
// fn fit_method_params() {
//     todo!()
// }

