#[derive(Clone, Copy, Debug)]
pub struct Key {
    pub active: bool,
    pub time_pressed: f64,
    pub time_released: f64,
}

impl Key {
    pub fn is_pressed(&self) -> bool {
        self.time_pressed > self.time_released
    }
}
