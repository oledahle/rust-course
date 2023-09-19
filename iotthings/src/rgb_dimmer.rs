use crate::traits::Dimmable;

pub struct RgbDimmer {
    red: f32,
    green: f32,
    blue: f32,
}

impl RgbDimmer {
    pub fn new(red: f32, green: f32, blue: f32) -> RgbDimmer {
        let mut my_red = red;
        if my_red > 1.0 {
            my_red = 1.0;
        }
        let mut my_green = green;
        if my_green > 1.0 {
            my_green = 1.0;
        }
        let mut my_blue = blue;
        if my_blue > 1.0 {
            my_blue = 1.0;
        }
        RgbDimmer{red: my_red, green: my_green, blue: my_blue,}
    }
}
impl Dimmable for RgbDimmer {
    fn brightness(&self) -> f32 {
        (self.red + self.green + self.blue) / 3f32
    }

    fn set_brightness(&mut self, setting: f32) -> f32 {
        self.red = setting;
        self.green = setting;
        self.blue = setting;
        self.brightness()
    }
}