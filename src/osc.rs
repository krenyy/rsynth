use crate::hz::Hertz;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Oscillator trait.
/// Is implemented on each oscillator and also on `Vec<Box<dyn Oscillator>>`.
#[typetag::serde(tag = "type")]
pub trait Oscillator: Debug + Send + Sync {
    /// Returns a value of an oscillator with a specific frequency at a specific time.
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32;
}

#[typetag::serde]
impl Oscillator for Vec<Box<dyn Oscillator>> {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        self.into_iter().map(|osc| osc.value(frequency, time)).sum()
    }
}

/// Sine wave oscillator.
#[derive(Debug, Deserialize, Serialize)]
pub struct Sine;

/// Square wave oscillator.
#[derive(Debug, Deserialize, Serialize)]
pub struct Square;

/// Triangle wave oscillator.
#[derive(Debug, Deserialize, Serialize)]
pub struct Triangle;

/// Sawtooth wave oscillator.
/// Takes a number of sinewaves to compose the sawtooth wave from.
/// Slower, but more detailed than `SawtoothFast`.
#[derive(Debug, Deserialize, Serialize)]
pub struct Sawtooth {
    pub num_sinewaves: usize,
}

/// Fast sawtooth wave oscillator.
/// Computes a sawtooth wave directly.
/// Faster, but less detailed than `Sawtooth`.
#[derive(Debug, Deserialize, Serialize)]
pub struct SawtoothFast;

/// Amplitude oscillator.
/// Adjust the amplitude of an existing oscillator.
#[derive(Debug, Deserialize, Serialize)]
pub struct Amplitude {
    pub amplitude: f32,
    pub oscillator: Box<dyn Oscillator>,
}

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
        (2. / ::std::f32::consts::PI)
            * (1..(1 + self.num_sinewaves))
                .into_iter()
                .map(|i| Sine.value(frequency, i as f64 * time) / -(i as f32))
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

#[typetag::serde]
impl Oscillator for Amplitude {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        self.amplitude * self.oscillator.value(frequency, time)
    }
}
