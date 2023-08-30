use crate::hz::Hertz;

pub trait Oscillator {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32;
}

#[derive(Clone, Copy)]
pub struct Sine;
#[derive(Clone, Copy)]
pub struct Square;
#[derive(Clone, Copy)]
pub struct Triangle;
#[derive(Clone, Copy)]
pub struct Sawtooth {
    pub num_sinewaves: usize,
}
#[derive(Clone, Copy)]
pub struct SawtoothFast;

impl Oscillator for Sine {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        (frequency.angular_velocity() * time).sin() as f32
    }
}

impl Oscillator for Square {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        Sine.value(frequency, time).signum()
    }
}

impl Oscillator for Triangle {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        Sine.value(frequency, time).asin()
    }
}

impl Oscillator for Sawtooth {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        (2. / ::std::f32::consts::PI)
            * (1..(1 + self.num_sinewaves))
                .into_iter()
                .map(|i| Sine.value(frequency, i as f64 * time))
                .sum::<f32>()
    }
}

impl Oscillator for SawtoothFast {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        ((2. / ::std::f64::consts::PI)
            * (*frequency * ::std::f64::consts::PI * (time % (1. / *frequency))
                - (::std::f64::consts::PI / 2.))) as f32
    }
}
