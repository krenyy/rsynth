use crate::key::Key;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Envelope {
    ADSR {
        attack_time: f64,
        decay_time: f64,
        sustain_amplitude: f64,
        release_time: f64,
    },
    AD {
        attack_time: f64,
        decay_time: f64,
    },
}

impl Envelope {
    pub fn amplitude(&self, key: &Key, time: f64) -> f64 {
        match *self {
            Envelope::ADSR {
                attack_time,
                decay_time,
                sustain_amplitude,
                release_time,
            } => {
                let key_released_before_attack_decay_finished =
                    (key.time_released - key.time_pressed) < (attack_time + decay_time);
                let during_attack_decay = (time - key.time_pressed) <= (attack_time + decay_time);
                if key.is_pressed()
                    || (key_released_before_attack_decay_finished && during_attack_decay)
                {
                    let pressed_for_time = time - key.time_pressed;
                    if pressed_for_time < attack_time {
                        return pressed_for_time / attack_time;
                    }
                    if pressed_for_time < attack_time + decay_time {
                        return 1.
                            - (1. - sustain_amplitude)
                                * ((pressed_for_time - attack_time) / decay_time);
                    }
                    return sustain_amplitude;
                } else {
                    // let released_for_time = time - key.time_released;
                    let released_for_time = if key_released_before_attack_decay_finished {
                        time - (key.time_pressed + attack_time + decay_time)
                    } else {
                        time - key.time_released
                    };
                    if released_for_time < release_time {
                        return sustain_amplitude
                            - sustain_amplitude * (released_for_time / release_time);
                    }
                    return 0.;
                }
            }
            Envelope::AD {
                attack_time,
                decay_time,
            } => {
                let pressed_for_time = time - key.time_pressed;
                if pressed_for_time < attack_time {
                    return pressed_for_time / attack_time;
                }
                if pressed_for_time < attack_time + decay_time {
                    return 1. - ((pressed_for_time - attack_time) / decay_time);
                }
                return 0.;
            }
        }
    }
}
