//! Per Point

use std::fmt::Debug;

use toml::Value as TomlValue;

use crate::{
    antispikes::Antispikes,
    diff_function::DiffFunction,
    load::Load,
    stacktrace::Stacktrace,
    types::{float::float, named_wrappers::{Deconvolved, DeconvolvedV, Params, ParamsG, ParamsV}},
};

use super::{DeconvolutionType, InitialValuesGeneric, InitialValuesVAD, ValueAndDomain};


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

    fn to_plottable_function(&self, params: &Params, significant_digits: u8, format: &'static str) -> String {
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

impl<T: Copy + Debug> InitialValuesGeneric<T> for InitialValues_PerPoint<T> {
    const LEN: usize = unreachable!();

    fn len(&self) -> usize {
        self.len
    }

    fn from_vec(params: &ParamsG<T>) -> Self {
        dbg!(params);
        unreachable!()
    }

    fn to_vec(&self) -> ParamsG<T> {
        ParamsG::<T>(vec![self.vad; self.len()])
    }

    fn params_to_points(&self, params: &Params, _points_len: usize, _x_start_end: (float, float)) -> Deconvolved {
        Deconvolved(params.0.to_vec())
    }

    fn params_to_points_v(&self, params: &ParamsV, _points_len: usize, _x_start_end: (float, float)) -> DeconvolvedV {
        DeconvolvedV(params.0.clone())
    }
}

impl InitialValuesVAD for InitialValues_PerPoint<ValueAndDomain> {}


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

