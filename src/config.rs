//! Config

use std::fs::read_to_string as read_file_to_string;

use toml::{
    Table as TomlTable,
    Value as TomlValue,
};

use crate::{
    fit_algorithms::FitAlgorithmVariant,
    load::Load,
    stacktrace::Stacktrace,
    types::float::float,
};

use super::deconvolution::{DeconvolutionVariant, deconvolution_data::AlignStepsTo};



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
        let toml_value: TomlValue = toml_table.into();
        Self::load_from_toml_value(&toml_value)
    }
    fn load_from_toml_value(toml_value: &TomlValue) -> Self {
        Self {
            deconvolution_function: ConfigDeconvolutionFunc::load_from_parent_as_root(toml_value),
            deconvolution_params: ConfigDeconvolutionParams::load_from_parent_as_root(toml_value),
            input_params: ConfigInputParams::load_from_parent_as_root(toml_value),
            output_params: ConfigOutputParams::load_from_parent_as_root(toml_value),
            fit_algorithm: ConfigFitAlgorithmParams::load_from_parent_as_root(toml_value),
        }
    }
}


pub type ConfigDeconvolutionFunc = DeconvolutionVariant;

#[derive(Debug, PartialEq)]
pub struct ConfigDeconvolutionParams {
    pub try_randomized_initial_values: u64,
    pub initial_values_random_scale: float,
    pub change_sing_probability: float,
    pub print_only_better_deconvolution: bool,
}
impl Load for ConfigDeconvolutionParams {
    const TOML_NAME: &'static str = "deconvolution_params";
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        Self {
            try_randomized_initial_values: toml_value.load_u64("try_randomized_initial_values", stacktrace),
            initial_values_random_scale: toml_value.load_float("initial_values_random_scale", stacktrace),
            change_sing_probability: toml_value.load_float("change_sing_probability", stacktrace),
            print_only_better_deconvolution: toml_value.load_bool("print_only_better_deconvolution", stacktrace),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ConfigInputParams {
    pub align_step_to: AlignStepsTo,
    pub max_step_relative_diff: float,
}
impl Load for ConfigInputParams {
    const TOML_NAME: &'static str = "input_params";
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        Self {
            align_step_to: AlignStepsTo::load_from_parent_handle_stacktrace(toml_value, stacktrace),
            max_step_relative_diff: toml_value.load_float("max_step_relative_diff", stacktrace),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ConfigOutputParams {
    pub significant_digits: u8,
}
impl Load for ConfigOutputParams {
    const TOML_NAME: &'static str = "output_params";
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        let significant_digits = toml_value.load_u8("significant_digits", stacktrace);
        assert!(significant_digits < 20);
        Self {
            significant_digits,
        }
    }
}

type ConfigFitAlgorithmParams = FitAlgorithmVariant;



trait ExtTomlValueLoadPrimitives {
    fn load_float(&self, name: &'static str, stacktrace: &Stacktrace) -> float;
    fn load_bool(&self, name: &'static str, stacktrace: &Stacktrace) -> bool;
    fn load_u64(&self, name: &'static str, stacktrace: &Stacktrace) -> u64;
    fn load_u8(&self, name: &'static str, stacktrace: &Stacktrace) -> u8;
}
impl ExtTomlValueLoadPrimitives for TomlValue {
    fn load_float(&self, name: &'static str, stacktrace: &Stacktrace) -> float {
        let stacktrace = stacktrace.pushed(name);
        self
            .get(name)
            .unwrap_or_else(|| stacktrace.panic_not_found())
            .as_float()
            .unwrap_or_else(|| stacktrace.panic_cant_parse_as("float"))
    }

    fn load_bool(&self, name: &'static str, stacktrace: &Stacktrace) -> bool {
        let stacktrace = stacktrace.pushed(name);
        self
            .get(name)
            .unwrap_or_else(|| stacktrace.panic_not_found())
            .as_bool()
            .unwrap_or_else(|| stacktrace.panic_cant_parse_as("bool"))
    }

    fn load_u64(&self, name: &'static str, stacktrace: &Stacktrace) -> u64 {
        let stacktrace = stacktrace.pushed(name);
        self
            .get(name)
            .unwrap_or_else(|| stacktrace.panic_not_found())
            .as_integer()
            .unwrap_or_else(|| stacktrace.panic_cant_parse_as("integer"))
            .try_into/* ::<u64> */()
            .unwrap_or_else(|_| stacktrace.panic_cant_parse_as("u64"))
    }

    fn load_u8(&self, name: &'static str, stacktrace: &Stacktrace) -> u8 {
        let stacktrace = stacktrace.pushed(name);
        self
            .get(name)
            .unwrap_or_else(|| stacktrace.panic_not_found())
            .as_integer()
            .unwrap_or_else(|| stacktrace.panic_cant_parse_as("integer"))
            .try_into/* ::<u64> */()
            .unwrap_or_else(|_| stacktrace.panic_cant_parse_as("u64"))
    }
}



#[test]
fn load_from_text_ok() {
    use crate::{
        deconvolution::types::{
            InitialValuesGeneric,
            ValueAndDomain,
            sat_exp__two_dec_exp__separate_consts::{SatExp_TwoDecExp_SeparateConsts, InitialValues_SatExp_TwoDecExp_SeparateConsts},
        },
        diff_function::DiffFunction,
        fit_algorithms::pattern_search::PatternSearch,
        types::named_wrappers::ParamsG,
    };
    let config_expected = Config {
        deconvolution_function: ConfigDeconvolutionFunc::SatExp_TwoDecExp_SeparateConsts(SatExp_TwoDecExp_SeparateConsts {
            diff_function_type: DiffFunction::DySqr,
            initial_vads: InitialValues_SatExp_TwoDecExp_SeparateConsts::from_vec(&ParamsG(vec![
                ValueAndDomain::free(0.12),
                ValueAndDomain::fixed(296.),
                ValueAndDomain::range_with_max(3.96, 10.),
                ValueAndDomain::range_with_min(6.71, 0.),
                ValueAndDomain::range_closed(1.16, (0., 2.)),
                ValueAndDomain::range_with_min(310., 0.),
            ])),
        }),
        deconvolution_params: ConfigDeconvolutionParams {
            try_randomized_initial_values: 42,
            initial_values_random_scale: 10.,
            change_sing_probability: 0.05,
            print_only_better_deconvolution: true,
        },
        input_params: ConfigInputParams {
            align_step_to: AlignStepsTo::Smaller,
            max_step_relative_diff: 0.02,
        },
        output_params: ConfigOutputParams {
            significant_digits: 4,
        },
        fit_algorithm: ConfigFitAlgorithmParams::PatternSearch(PatternSearch {
            fit_algorithm_min_step: 1e-4,
            fit_residue_evals_max: 1_000_000,
            initial_step: 1.,
            alpha: 1.1,
            beta: None,
        }),
    };
    let config_actual = Config::load_from_text(
r#"
[deconvolution_function.SatExp_TwoDecExp_SeparateConsts]
diff_function_type = "DySqr"
initial_values = "b=0.12, c==296, s=3.96<10, ta=6.71>0, 0<tb=1.16<2, tc=310>0"

[deconvolution_params]
try_randomized_initial_values = 42
initial_values_random_scale = 10.0
change_sing_probability = 0.05
print_only_better_deconvolution = true

[input_params]
align_steps_to = "smaller"
max_step_relative_diff = 0.02

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
    dbg!(&config_expected, &config_actual);
    assert_eq!(config_expected, config_actual);
}

#[should_panic(expected = "`deconvolution_function` -> `Two_SatExp_DecExp` -> `diff_function_type`: not found")]
#[test]
fn load_from_text_panic() {
    Config::load_from_text(
r#"
[deconvolution_function.Two_SatExp_DecExp]
# diff_function_type = "DySqr"
initial_values = [ 0.12, 296.0, 3.96, 6.7, 1.16, 310.0, 23.2, 1.79 ]

[deconvolution_params]
try_randomized_initial_values = 42
initial_values_random_scale = 10.0
change_sing_probability = 0.05
print_only_better_deconvolution = true

[input_params]
align_steps_to = "smaller"

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
}
