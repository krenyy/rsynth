use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};

use crate::{envelope::Envelope, osc::Oscillator};

#[derive(Deserialize, Serialize)]
pub struct Instrument {
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
}
