use crate::traits::Dimmable;

mod rgb_dimmer;
mod traits;
mod binary_switch;
mod tests;

fn main() {
    use crate::rgb_dimmer::RgbDimmer;
    use crate::traits::Dimmable;
    let dimmer = RgbDimmer::new(1., 0.5, 1.);
    println!("Hello, Dimmers: {}", dimmer.brightness());
}
