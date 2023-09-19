use crate::traits::Dimmable;

pub struct BinarySwitch {
   on: bool,
}

impl BinarySwitch {
    pub fn new(on: bool) -> BinarySwitch {
        BinarySwitch{on}
    }
}
impl Dimmable for BinarySwitch {
    fn brightness(&self) -> f32 {
       if self.on {
           return 1.0
       } else {
           0.0
       }
    }

    fn set_brightness(&mut self, setting: f32) -> f32 {
        if setting >= 1.0 {
            self.on = true;
        } else {
            self.on = false;
        }
        self.brightness()
    }
}