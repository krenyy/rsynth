use std::fmt;

use serde::{Deserialize, Serialize};

use crate::hz::Hertz;

#[typetag::serde(tag = "type")]
pub trait Oscillator: fmt::Debug + Send + Sync {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32;

    fn name(&self) -> &'static str;

    fn tree(&self, v: &mut Vec<(&'static str, usize, Option<Box<dyn ToString>>)>, level: usize) {
        v.push((self.name(), level, None));
    }
}

#[typetag::serde]
impl Oscillator for Vec<Box<dyn Oscillator>> {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        self.into_iter().map(|osc| osc.value(frequency, time)).sum()
    }

    fn name(&self) -> &'static str {
        "Collection"
    }

    fn tree(&self, v: &mut Vec<(&'static str, usize, Option<Box<dyn ToString>>)>, level: usize) {
        v.push((self.name(), level, None));
        self.into_iter().for_each(|x| x.tree(v, level + 1));
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

    fn name(&self) -> &'static str {
        "Sine"
    }
}

#[typetag::serde]
impl Oscillator for Square {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        Sine.value(frequency, time).signum()
    }

    fn name(&self) -> &'static str {
        "Square"
    }
}

#[typetag::serde]
impl Oscillator for Triangle {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        Sine.value(frequency, time).asin()
    }

    fn name(&self) -> &'static str {
        "Triangle"
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

    fn name(&self) -> &'static str {
        "Sawtooth"
    }

    fn tree(&self, v: &mut Vec<(&'static str, usize, Option<Box<dyn ToString>>)>, level: usize) {
        v.push((self.name(), level, None));
        v.push((
            "num_sinewaves",
            level + 1,
            Some(Box::new(self.num_sinewaves)),
        ))
    }
}

#[typetag::serde]
impl Oscillator for SawtoothFast {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        ((2. / ::std::f64::consts::PI)
            * (*frequency * ::std::f64::consts::PI * (time % (1. / *frequency))
                - (::std::f64::consts::PI / 2.))) as f32
    }

    fn name(&self) -> &'static str {
        "SawtoothFast"
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

    fn name(&self) -> &'static str {
        "Amplitude"
    }

    fn tree(&self, v: &mut Vec<(&'static str, usize, Option<Box<dyn ToString>>)>, level: usize) {
        v.push((self.name(), level, None));
        v.push(("amplitude", level + 1, Some(Box::new(self.amplitude))));
        self.oscillator.tree(v, level + 1);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Zero;

#[typetag::serde]
impl Oscillator for Zero {
    fn value(&self, _frequency: Hertz<f64>, _time: f64) -> f32 {
        0.
    }

    fn name(&self) -> &'static str {
        "Zero"
    }
}
