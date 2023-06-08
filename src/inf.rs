use crate::Sign;

/// Get the sign if the float is ±∞.
pub trait InfSign {
    /// Get the sign if the float is ±∞.
    fn inf_sign(&self) -> Option<Sign>;
}

impl InfSign for f32 {
    fn inf_sign(&self) -> Option<Sign> {
        if *self == f32::NEG_INFINITY {
            Some(Sign::Negative)
        } else if *self == f32::INFINITY {
            Some(Sign::Positive)
        } else {
            None
        }
    }
}

impl InfSign for f64 {
    fn inf_sign(&self) -> Option<Sign> {
        if *self == f64::NEG_INFINITY {
            Some(Sign::Negative)
        } else if *self == f64::INFINITY {
            Some(Sign::Positive)
        } else {
            None
        }
    }
}

/// Check if the float is ±∞.
pub trait IsInf: InfSign {
    /// Check if the float is ±∞.
    fn is_inf(&self) -> bool {
        self.inf_sign().is_some()
    }
}

impl<X: InfSign> IsInf for X {}
