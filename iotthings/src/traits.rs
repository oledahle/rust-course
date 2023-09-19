pub trait Dimmable {
    fn brightness(&self) -> f32;

    fn set_brightness(&mut self, setting: f32) -> f32;
}