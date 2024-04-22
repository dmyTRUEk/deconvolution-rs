//! Named wrapper types.
//!
//! There are no `.into()` to prevent from accidental casting to wrong type.

use crate::types::{
    float::float,
    linalg::DVect,
};



pub struct Convolved(pub Vec<float>);
pub struct Deconvolved(pub Vec<float>);
pub struct Instrument(pub Vec<float>);
pub struct InstrumentRev(pub Vec<float>);
pub struct Measured(pub Vec<float>);
/// Generic Params
#[derive(Debug, Clone)]
pub struct ParamsG<T>(pub Vec<T>);
pub type Params = ParamsG<float>;

pub struct ConvolvedV(pub DVect);
pub struct DeconvolvedV(pub DVect);
pub struct InstrumentV(pub DVect);
pub struct InstrumentRevV(pub DVect);
pub struct MeasuredV(pub DVect);
#[derive(Debug, Clone)]
pub struct ParamsV(pub DVect);


impl From<Convolved> for ConvolvedV {
    fn from(value: Convolved) -> Self {
        Self(DVect::from_vec(value.0))
    }
}
impl From<Deconvolved> for DeconvolvedV {
    fn from(value: Deconvolved) -> Self {
        Self(DVect::from_vec(value.0))
    }
}
impl From<Instrument> for InstrumentV {
    fn from(value: Instrument) -> Self {
        Self(DVect::from_vec(value.0))
    }
}
impl From<Instrument> for InstrumentRevV {
    fn from(value: Instrument) -> Self {
        let instrument_rev = InstrumentRev::from(value);
        Self(DVect::from_vec(instrument_rev.0))
    }
}
impl From<Instrument> for InstrumentRev {
    fn from(value: Instrument) -> Self {
        let mut vec = value.0;
        vec.reverse();
        Self(vec)
    }
}
// impl From<InstrumentV> for InstrumentRevV {
//     fn from(value: InstrumentV) -> Self {
//         let mut vec: Vec<float> = value.0.data.as_vec().to_vec();
//         vec.reverse(); // using this reverse bc it's probably the fastest
//         Self(DVect::from_vec(vec))
//     }
// }
impl From<InstrumentRev> for InstrumentRevV {
    fn from(value: InstrumentRev) -> Self {
        Self(DVect::from_vec(value.0))
    }
}
impl From<Measured> for MeasuredV {
    fn from(value: Measured) -> Self {
        Self(DVect::from_vec(value.0))
    }
}
impl From<Params> for ParamsV {
    fn from(value: Params) -> Self {
        Self(DVect::from_vec(value.0))
    }
}


