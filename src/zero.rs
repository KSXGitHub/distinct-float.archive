use crate::Sign;

/// Get the sign if the float is equal to zero.
pub trait ZeroSign {
    /// Get the sign if the float is equal to zero.
    fn zero_sign(&self) -> Option<Sign>;
}

impl ZeroSign for f32 {
    fn zero_sign(&self) -> Option<Sign> {
        if *self != 0.0 {
            None
        } else if self.is_sign_negative() {
            Some(Sign::Negative)
        } else {
            Some(Sign::Positive)
        }
    }
}

impl ZeroSign for f64 {
    fn zero_sign(&self) -> Option<Sign> {
        if *self != 0.0 {
            None
        } else if self.is_sign_negative() {
            Some(Sign::Negative)
        } else {
            Some(Sign::Positive)
        }
    }
}

/// Check if the float is zero.
pub trait IsZero: ZeroSign {
    /// Check if the float is zero.
    fn is_zero(&self) -> bool {
        self.zero_sign().is_some()
    }
}

impl<X: ZeroSign> IsZero for X {}
