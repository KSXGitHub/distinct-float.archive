use crate::{Float, Sign, ToHashable};
use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};
use derive_more::{AsMut, AsRef, Constructor, Deref, DerefMut, Display};
use pipe_trait::Pipe;

/// Enforce distinction on a float.
///
/// If the underlying type has [partial equality](PartialEq), this type will have [total equality](Eq):
/// * -0.0 ≠ +0.0
/// * NaN = NaN
///
/// If the underlying type has [partial order](PartialOrd), this type will have [total order](Ord):
/// * -∞ < NegNonZero < -0.0 < NaN < +0.0 < PosNonZero < +∞
#[derive(Debug, Display, Default, Constructor, Clone, Copy, AsRef, AsMut, Deref, DerefMut)]
pub struct Distinct<Value>(pub Value);

impl<Value> Distinct<Value> {
    /// Access the inner value.
    pub fn into_inner(self) -> Value {
        let Distinct(value) = self;
        value
    }
}

impl<Value: Float + PartialEq> PartialEq for Distinct<Value> {
    fn eq(&self, other: &Self) -> bool {
        if self.is_nan() && other.is_nan() {
            return true;
        }

        match (self.zero_sign(), other.zero_sign()) {
            (None, None) => {}
            (Some(a), Some(b)) => return a == b,
            (Some(_), None) | (None, Some(_)) => return false,
        }

        match (self.inf_sign(), other.inf_sign()) {
            (None, None) => {}
            (Some(a), Some(b)) => return a == b,
            (Some(_), None) | (None, Some(_)) => return false,
        }

        self.0 == other.0
    }
}

impl<Value: Float + PartialEq> Eq for Distinct<Value> {}

/// Helper type to use in the implementation of [`Hash`] for [`Distinct`].
#[derive(Hash)]
enum FloatHashable<Hashable: Hash> {
    Nan,
    Zero(Sign),
    Infinite(Sign),
    Finite(Hashable),
}

impl<Value: Float + PartialEq + ToHashable> Hash for Distinct<Value> {
    fn hash<State: Hasher>(&self, state: &mut State) {
        let hashable = {
            use FloatHashable::*;
            if self.is_nan() {
                Nan
            } else if let Some(sign) = self.zero_sign() {
                Zero(sign)
            } else if let Some(sign) = self.inf_sign() {
                Infinite(sign)
            } else {
                self.to_hashable().pipe(Finite)
            }
        };

        hashable.hash(state)
    }
}

impl<Value: Float + PartialOrd> PartialOrd for Distinct<Value> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmp(other).pipe(Some)
    }
}

impl<Value: Float + PartialOrd> Ord for Distinct<Value> {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;
        use Sign::*;

        match (self.inf_sign(), other.inf_sign()) {
            (None, None) => {}
            (None, Some(Negative)) => return Greater,
            (None, Some(Positive)) => return Less,
            (Some(Negative), None) => return Less,
            (Some(Positive), None) => return Greater,
            (Some(a), Some(b)) => return a.cmp(&b),
        }

        match (self.get_sign(), other.get_sign()) {
            (None, None) => return Equal,
            (None, Some(Negative)) => return Greater,
            (None, Some(Positive)) => return Less,
            (Some(Negative), None) => return Less,
            (Some(Positive), None) => return Greater,
            (Some(_), Some(_)) => {}
        }

        if let (Some(a), Some(b)) = (self.zero_sign(), other.zero_sign()) {
            return a.cmp(&b);
        }

        self.0
            .partial_cmp(&other.0)
            .expect("Finite numbers should always be comparable")
    }
}
