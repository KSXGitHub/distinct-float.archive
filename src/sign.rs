/// Sign of a float.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sign {
    /// The float is negative.
    Negative,
    /// The float is positive.
    Positive,
}

/// Get the sign of the float.
pub trait GetSign {
    /// Get the sign of the float.
    ///
    /// The sign for -0.0 will be different from the sign for +0.0.
    ///
    /// `None` means [NaN](https://en.wikipedia.org/wiki/NaN).
    fn get_sign(&self) -> Option<Sign>;
}

impl GetSign for f32 {
    fn get_sign(&self) -> Option<Sign> {
        if f32::is_nan(*self) {
            None
        } else if self.is_sign_negative() {
            Some(Sign::Negative)
        } else if self.is_sign_positive() {
            Some(Sign::Positive)
        } else {
            None
        }
    }
}

impl GetSign for f64 {
    fn get_sign(&self) -> Option<Sign> {
        if f64::is_nan(*self) {
            None
        } else if self.is_sign_negative() {
            Some(Sign::Negative)
        } else if self.is_sign_positive() {
            Some(Sign::Positive)
        } else {
            None
        }
    }
}

/// Check if a number is [NaN](https://en.wikipedia.org/wiki/NaN).
pub trait IsNan: GetSign {
    /// Check if a number is [NaN](https://en.wikipedia.org/wiki/NaN).
    fn is_nan(&self) -> bool {
        self.get_sign().is_none()
    }
}

impl<X: GetSign> IsNan for X {}
