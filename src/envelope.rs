use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Envelope {
    ADSR {
        attack_time: f64,
        decay_time: f64,
        sustain_amplitude: f64,
        release_time: f64,
    },
}
