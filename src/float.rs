use crate::{IsInf, IsNan, IsZero};

/// Signify that the type is a float.
pub trait Float: IsNan + IsZero + IsInf {}
impl<X: IsNan + IsZero + IsInf> Float for X {}
