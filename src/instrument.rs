use std::fs;

use serde::{Deserialize, Serialize};

use crate::{envelope::Envelope, osc::Oscillator};

#[derive(Deserialize, Serialize)]
pub struct Instrument {
    pub envelope: Envelope,
    pub oscillator: Box<dyn Oscillator>,
}

impl Instrument {
    pub fn read(path: &str) -> Self {
        serde_yaml::from_str(&fs::read_to_string(path).expect("io error during read_to_string!"))
            .unwrap_or(Instrument {
                envelope: Envelope::ADSR {
                    attack_time: 0.,
                    decay_time: 0.,
                    sustain_amplitude: 0.,
                    release_time: 0.,
                },
                oscillator: Box::new(vec![]),
            })
    }
}
