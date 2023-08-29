#[derive(Debug, Clone, Copy)]
pub struct Hertz<T>(T);

impl Hertz<f32> {
    pub fn angular_velocity(self) -> f32 {
        self.0 * 2. * ::std::f32::consts::PI
    }
}

impl Hertz<f64> {
    pub fn angular_velocity(self) -> f64 {
        self.0 * 2. * ::std::f64::consts::PI
    }
}

impl<T> std::ops::Deref for Hertz<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait Hz<T> {
    fn hz(self) -> Hertz<T>;
}

impl Hz<f32> for f32 {
    fn hz(self) -> Hertz<f32> {
        Hertz(self)
    }
}

impl Hz<f32> for &f32 {
    fn hz(self) -> Hertz<f32> {
        Hertz(*self)
    }
}

impl Hz<f32> for &mut f32 {
    fn hz(self) -> Hertz<f32> {
        Hertz(*self)
    }
}

impl Hz<f64> for f64 {
    fn hz(self) -> Hertz<f64> {
        Hertz(self)
    }
}

impl Hz<f64> for &f64 {
    fn hz(self) -> Hertz<f64> {
        Hertz(*self)
    }
}

impl Hz<f64> for &mut f64 {
    fn hz(self) -> Hertz<f64> {
        Hertz(*self)
    }
}
