use crate::hz::Hertz;
use std::{any::Any, collections::HashMap};

pub trait Oscillator: Send + Sync {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32;

    fn name(&self) -> &'static str;

    fn get_fields(&self) -> Option<HashMap<&'static str, &dyn Any>> {
        None
    }

    fn get_fields_mut(&mut self) -> Option<HashMap<&'static str, &mut dyn Any>> {
        None
    }
}

impl<I> Oscillator for I
where
    I: ?Sized + Sync + Send,
    for<'a> &'a I: IntoIterator<Item = &'a Box<dyn Oscillator>>,
{
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        self.into_iter().map(|osc| osc.value(frequency, time)).sum()
    }

    fn name(&self) -> &'static str {
        "Iterable"
    }
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

    fn name(&self) -> &'static str {
        "Sine"
    }
}

impl Oscillator for Square {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        Sine.value(frequency, time).signum()
    }

    fn name(&self) -> &'static str {
        "Square"
    }
}

impl Oscillator for Triangle {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        Sine.value(frequency, time).asin()
    }

    fn name(&self) -> &'static str {
        "Triangle"
    }
}

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

    fn get_fields(&self) -> Option<HashMap<&'static str, &dyn Any>> {
        let mut map = HashMap::<&str, &dyn Any>::new();
        map.insert("num_sinewaves", &self.num_sinewaves);
        Some(map)
    }

    fn get_fields_mut(&mut self) -> Option<HashMap<&'static str, &mut dyn Any>> {
        let mut map = HashMap::<&str, &mut dyn Any>::new();
        map.insert("num_sinewaves", &mut self.num_sinewaves);
        Some(map)
    }
}

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

#[derive(Clone)]
pub struct Amplitude<T: Oscillator> {
    pub amplitude: f32,
    pub oscillator: T,
}

impl<T: Oscillator + 'static> Oscillator for Amplitude<T> {
    fn value(&self, frequency: Hertz<f64>, time: f64) -> f32 {
        self.amplitude * self.oscillator.value(frequency, time)
    }

    fn name(&self) -> &'static str {
        "Amplitude"
    }

    fn get_fields(&self) -> Option<HashMap<&'static str, &dyn Any>> {
        let mut map = HashMap::<&str, &dyn Any>::new();
        map.insert("amplitude", &self.amplitude);
        map.insert("oscillator", &self.oscillator);
        Some(map)
    }

    fn get_fields_mut(&mut self) -> Option<HashMap<&'static str, &mut dyn Any>> {
        let mut map = HashMap::<&str, &mut dyn Any>::new();
        map.insert("amplitude", &mut self.amplitude);
        map.insert("oscillator", &mut self.oscillator);
        Some(map)
    }
}

#[derive(Clone, Copy)]
pub struct Zero;

impl Oscillator for Zero {
    fn value(&self, _frequency: Hertz<f64>, _time: f64) -> f32 {
        0.
    }

    fn name(&self) -> &'static str {
        "Zero"
    }
}
