use crate::{envelope::Envelope, hz::Hz, osc::Oscillator};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};

// why can't I call powf in a const context?
// const STEP_BASE: f64 = 2f64.powf(1. / 12.);
const STEP_BASE: f64 = 1.0594630944;
const A4_FREQUENCY: f64 = 440.;

#[derive(Clone, Copy, Debug)]
pub struct Key {
    pub active: bool,
    pub state: KeyState,
}

#[derive(Clone, Copy, Debug)]
pub enum KeyState {
    Pressed { time_pressed: f64 },
    Released { time_released: f64 },
}

#[derive(Deserialize, Serialize)]
pub struct Instrument {
    pub volume: f32,
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

    pub fn value(&self, keys: &[Key; 256], time: f64) -> f32 {
        keys.into_iter()
            .enumerate()
            .filter(|(_i, key)| key.active)
            .map(|(i, key)| {
                self.envelope.amplitude(key, time) as f32
                    * self
                        .oscillator
                        .value((A4_FREQUENCY * STEP_BASE.powi(i as i32 - 57)).hz(), time)
            })
            .sum::<f32>()
            * self.volume
    }
}
