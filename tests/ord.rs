use distinct_float::Distinct;
use pretty_assertions::assert_eq;
use std::fmt::Debug;

fn assert_dbg_eq(a: impl Debug, b: impl Debug) {
    assert_eq!(format!("{a:#?}"), format!("{b:#?}"));
}

#[test]
fn sort() {
    let mut array = [0.0, -0.0, f64::INFINITY, f64::NEG_INFINITY, f64::NAN].map(Distinct::new);
    array.sort();
    let received = array.map(Distinct::into_inner);
    dbg!(received);
    let expected = [f64::NEG_INFINITY, -0.0, f64::NAN, 0.0, f64::INFINITY];
    assert_dbg_eq(received, expected);
}
