//! Per Point

use toml::Value as TomlValue;

use crate::{
    antispikes::Antispikes,
    diff_function::DiffFunction,
    float_type::float,
    load::Load,
    stacktrace::Stacktrace,
};

use super::{DeconvolutionType, InitialValuesGeneric, InitialValuesVAD, ValueAndDomain, i_to_x};


/// [y0, y1, y2, …]
#[derive(Debug, Clone, PartialEq)]
pub struct PerPoint {
    pub diff_function_type: DiffFunction,
    pub antispikes: Option<Antispikes>,
    pub initial_vad: InitialValues_PerPoint<ValueAndDomain>,
}

impl DeconvolutionType for PerPoint {
    const NAME: &'static str = "per point";

    const FORMAT_FOR_DESMOS: &'static str = unreachable!();
    const FORMAT_FOR_ORIGIN: &'static str = unreachable!();

    fn to_plottable_function(&self, params: &Vec<float>, significant_digits: u8, format: &'static str) -> String {
        unreachable!()
    }
}

impl Load for PerPoint {
    const TOML_NAME: &'static str = stringify!(PerPoint);
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        Self {
            diff_function_type: DiffFunction::load_from_parent_handle_stacktrace(toml_value, stacktrace),
            antispikes: Some(Antispikes::load_from_parent_handle_stacktrace(toml_value, stacktrace)),
            initial_vad: InitialValues_PerPoint::load_from_parent_handle_stacktrace(toml_value, stacktrace),
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InitialValues_PerPoint<T> {
    // TODO: remove `pub`?
    pub len: usize,
    pub vad: T,
}

impl<T> InitialValues_PerPoint<T> {
    pub const fn new(len: usize, vad: T) -> Self {
        Self { len, vad }
    }
}

impl<T: Copy + std::fmt::Debug> InitialValuesGeneric<T> for InitialValues_PerPoint<T> {
    const LEN: usize = unreachable!();

    fn len(&self) -> usize {
        self.len
    }

    fn from_vec(params: &Vec<T>) -> Self {
        dbg!(params);
        unreachable!()
    }

    fn to_vec(&self) -> Vec<T> {
        vec![self.vad; self.len()]
    }

    fn params_to_points(&self, params: &Vec<float>, _points_len: usize, _x_start_end: (float, float)) -> Vec<float> {
        params.to_vec()
    }
}

impl InitialValuesVAD for InitialValues_PerPoint<ValueAndDomain> {
    fn get_randomized(&self, initial_values_random_scale: float) -> Vec<float> {
        unimplemented!()
    }
}


impl Load for InitialValues_PerPoint<ValueAndDomain> {
    const TOML_NAME: &'static str = "initial_values";
    fn load_from_self(toml_value: &TomlValue, stacktrace: &Stacktrace) -> Self {
        // let initial_vad = toml_value
        //     .get("initial_value")
        //     .expect("deconvolution_function -> PerPoint: `initial_value` not found")
        //     .as_float()
        //     .expect("deconvolution_function -> PerPoint -> initial_value: can't parse as float");
        todo!()
    }
}

