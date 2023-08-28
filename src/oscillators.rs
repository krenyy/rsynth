pub trait Oscillator {
    fn value(frequency: f64, time: f64) -> f64;
}

pub struct Sine;
pub struct Square;

impl Oscillator for Sine {
    fn value(frequency: f64, time: f64) -> f64 {
        (2. * frequency * time * ::std::f64::consts::PI).sin()
    }
}

impl Oscillator for Square {
    fn value(frequency: f64, time: f64) -> f64 {
        (2. * frequency * time * ::std::f64::consts::PI)
            .sin()
            .signum()
    }
}
