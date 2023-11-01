//! Per Point

use toml::Value as TomlValue;

use crate::{
    antispikes::Antispikes,
    config::Load,
    diff_function::DiffFunction,
    float_type::float,
};

use super::{InitialValuesGeneric, InitialValuesF, InitialValuesVAD, ValueAndDomain, DeconvolutionType, i_to_x};


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

    fn load_from_self_toml_value(toml_value: &TomlValue) -> Self {
        let diff_function_type = DiffFunction::load_from_self_toml_value(
            toml_value
                .get("diff_function_type")
                .expect("deconvolution_function -> PerPoint: `diff_function_type` not found")
        );
        let antispikes = toml_value
            .get("antispikes")
            .map(Antispikes::load_from_self_toml_value);
        // let initial_vad = toml_value
        //     .get("initial_value")
        //     .expect("deconvolution_function -> PerPoint: `initial_value` not found")
        //     .as_float()
        //     .expect("deconvolution_function -> PerPoint -> initial_value: can't parse as float");
        let initial_vad = InitialValues_PerPoint::load_from_parent_toml_value(toml_value);
        PerPoint {
            diff_function_type,
            antispikes,
            initial_vad,
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InitialValues_PerPoint<T> {
    pub len: usize,
    pub vad: T,
}

impl<T> InitialValues_PerPoint<T> {
    pub const fn new(len: usize, vad: T) -> Self {
        Self { len, vad }
    }
}

impl<T: Copy + std::fmt::Debug> InitialValuesGeneric<T> for InitialValues_PerPoint<T> {
    fn len_stat() -> usize {
        unreachable!()
    }

    fn len_dyn(&self) -> usize {
        self.len
    }

    fn from_vec(params: &Vec<T>) -> Self {
        dbg!(params);
        unreachable!()
    }

    fn to_vec(&self) -> Vec<T> {
        vec![self.vad; self.len_dyn()]
    }

    fn params_to_points(&self, params: &Vec<float>, _points_len: usize, _x_start_end: (float, float)) -> Vec<float> {
        params.to_vec()
    }
}

impl InitialValuesVAD for InitialValues_PerPoint<ValueAndDomain> {}

// impl From<InitialValues_PerPoint<ValueAndDomain>> for InitialValues_PerPoint<float> {
//     fn from(value: InitialValues_PerPoint<ValueAndDomain>) -> Self {
//         Self::from_vec(&value.to_vec().iter().map(|v| v.value).collect())
//     }
// }


impl Load for InitialValues_PerPoint<ValueAndDomain> {
    const TOML_NAME: &'static str = "initial_values";
    fn load_from_self_toml_value(toml_value: &TomlValue) -> Self {
        todo!()
    }
}

