pub trait Hz<T: num::Float> {
    fn hz(self) -> Hertz<T>;
}

impl<T: num::Float> Hz<T> for T {
    fn hz(self) -> Hertz<T> {
        Hertz(self)
    }
}

impl<T: num::Float> Hz<T> for &T {
    fn hz(self) -> Hertz<T> {
        Hertz(*self)
    }
}

impl<T: num::Float> Hz<T> for &mut T {
    fn hz(self) -> Hertz<T> {
        Hertz(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hertz<T: num::Float>(T);

impl Hertz<f64> {
    pub fn angular_velocity(self) -> f64 {
        self.0 * 2. * ::std::f64::consts::PI
    }
}

impl<T: num::Float> std::ops::Deref for Hertz<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
