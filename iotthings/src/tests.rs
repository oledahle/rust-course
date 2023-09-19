use crate::rgb_dimmer::RgbDimmer;
use crate::binary_switch::BinarySwitch;
use crate::traits::Dimmable;


#[test]
fn test_rgb() {
    assert_eq!(RgbDimmer::new(1., 1., 1.).brightness(), 1.0);
}

#[test]
fn test_switch_on() {
    assert_eq!(BinarySwitch::new(true).brightness(), 1.0);
}

#[test]
fn test_switch_off() {
    assert_eq!(BinarySwitch::new(false).brightness(), 0.0);
}

#[test]
fn illegal_values_rgb() {
    assert_eq!(RgbDimmer::new(5.5, 15.1, 1.3).brightness(), 1.0);
}