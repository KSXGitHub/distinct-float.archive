use distinct_float::{GetSign, Sign};
use pretty_assertions::assert_eq;

#[test]
fn sign_ord() {
    use Sign::*;
    assert!(Negative < Positive);
    assert!(Positive > Negative);
}

#[test]
fn get_sign_f32() {
    use Sign::*;
    assert_eq!(0.1f32.get_sign(), Some(Positive));
    assert_eq!((-0.1f32).get_sign(), Some(Negative));
    assert_eq!(0.0f32.get_sign(), Some(Positive));
    assert_eq!((-0.0f32).get_sign(), Some(Negative));
    assert_eq!(f32::INFINITY.get_sign(), Some(Positive));
    assert_eq!(f32::NEG_INFINITY.get_sign(), Some(Negative));
    assert_eq!(f32::NAN.get_sign(), None);
}

#[test]
fn get_sign_f64() {
    use Sign::*;
    assert_eq!(0.1f64.get_sign(), Some(Positive));
    assert_eq!((-0.1f64).get_sign(), Some(Negative));
    assert_eq!(0.0f64.get_sign(), Some(Positive));
    assert_eq!((-0.0f64).get_sign(), Some(Negative));
    assert_eq!(f64::INFINITY.get_sign(), Some(Positive));
    assert_eq!(f64::NEG_INFINITY.get_sign(), Some(Negative));
    assert_eq!(f64::NAN.get_sign(), None);
}
