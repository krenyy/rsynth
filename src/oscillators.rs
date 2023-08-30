use crate::hz::Hertz;

pub trait Oscillator {
    fn value(frequency: Hertz<f64>, time: f64) -> f32;
}

pub struct Sine;
pub struct Square;

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
