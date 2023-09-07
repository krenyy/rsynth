use crate::{envelope::Envelope, hz::Hertz, osc::Oscillator};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};

#[derive(Deserialize, Serialize)]
pub struct Instrument {
    pub volume: f64,
    pub envelope: Envelope,
    pub oscillator: Box<dyn Oscillator>,
}

#[derive(Debug)]
pub enum InstrumentReadError {
    IoError(io::Error),
    Deserialize(serde_yaml::Error),
}

impl Instrument {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self, InstrumentReadError> {
        match fs::read_to_string(path) {
            Ok(s) => serde_yaml::from_str(&s).map_err(|e| InstrumentReadError::Deserialize(e)),
            Err(e) => return Err(InstrumentReadError::IoError(e)),
        }
    }

    pub fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        self.volume as f32 * self.oscillator.value(frequency, time)
    }
}
