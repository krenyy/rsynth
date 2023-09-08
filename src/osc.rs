use crate::hz::Hertz;
use serde::{Deserialize, Serialize};
use std::fmt;

#[typetag::serde(tag = "type")]
pub trait Oscillator: fmt::Debug + Send + Sync {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32;
}

#[typetag::serde]
impl Oscillator for Vec<Box<dyn Oscillator>> {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        self.into_iter().map(|osc| osc.value(frequency, time)).sum()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sine;
#[derive(Debug, Deserialize, Serialize)]
pub struct Square;
#[derive(Debug, Deserialize, Serialize)]
pub struct Triangle;
#[derive(Debug, Deserialize, Serialize)]
pub struct Sawtooth {
    pub num_sinewaves: usize,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SawtoothFast;

#[typetag::serde]
impl Oscillator for Sine {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        (frequency.angular_velocity() * time).sin() as f32
    }
}

#[typetag::serde]
impl Oscillator for Square {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        Sine.value(frequency, time).signum()
    }
}

#[typetag::serde]
impl Oscillator for Triangle {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        Sine.value(frequency, time).asin()
    }
}

#[typetag::serde]
impl Oscillator for Sawtooth {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        -(2. / ::std::f32::consts::PI)
            * (1..(1 + self.num_sinewaves))
                .into_iter()
                .map(|i| Sine.value(frequency, i as f64 * time) / i as f32)
                .sum::<f32>()
    }
}

#[typetag::serde]
impl Oscillator for SawtoothFast {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        ((2. / ::std::f64::consts::PI)
            * (*frequency * ::std::f64::consts::PI * (time % (1. / *frequency))
                - (::std::f64::consts::PI / 2.))) as f32
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Amplitude {
    pub amplitude: f32,
    pub oscillator: Box<dyn Oscillator>,
}

#[typetag::serde]
impl Oscillator for Amplitude {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        self.amplitude * self.oscillator.value(frequency, time)
    }
}
