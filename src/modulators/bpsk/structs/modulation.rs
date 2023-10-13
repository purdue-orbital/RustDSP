use crate::common::constellation::Constellation;

pub struct Modulation {
    // Calculate the number of samples per a symbol
    pub(crate) samples_per_symbol: usize,

    // The rate the radio will sample at
    pub(crate) sample_rate: f32,
    
    pub(crate) constellation: Constellation
}