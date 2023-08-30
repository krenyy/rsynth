use crate::hz::Hertz;

pub trait Oscillator {
    fn value(frequency: Hertz<f64>, time: f64) -> f32;
}

pub struct Sine;
pub struct Square;
pub struct Triangle;
pub struct Sawtooth;
pub struct SawtoothFast;

impl Oscillator for Sine {
    fn value(frequency: Hertz<f64>, time: f64) -> f32 {
        (frequency.angular_velocity() * time).sin() as f32
    }
}

impl Oscillator for Square {
    fn value(frequency: Hertz<f64>, time: f64) -> f32 {
        Sine::value(frequency, time).signum()
    }
}

impl Oscillator for Triangle {
    fn value(frequency: Hertz<f64>, time: f64) -> f32 {
        Sine::value(frequency, time).asin()
    }
}

impl Oscillator for Sawtooth {
    fn value(frequency: Hertz<f64>, time: f64) -> f32 {
        (2. / ::std::f32::consts::PI)
            * (1..10)
                .into_iter()
                .map(|i| Sine::value(frequency, i as f64 * time))
                .sum::<f32>()
    }
}

impl Oscillator for SawtoothFast {
    fn value(frequency: Hertz<f64>, time: f64) -> f32 {
        ((2. / ::std::f64::consts::PI)
            * (*frequency * ::std::f64::consts::PI * (time % (1. / *frequency))
                - (::std::f64::consts::PI / 2.))) as f32
    }
}
