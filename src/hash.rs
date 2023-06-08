use core::hash::Hash;
use pipe_trait::Pipe;

/// Convert a float to something that can be hashed.
pub trait ToHashable {
    /// The underlying bits.
    type Hashable: Hash;
    /// Convert a float to something that can be hashed.
    fn to_hashable(&self) -> Self::Hashable;
}

/// Return type of [`ToHashable`] implementation of [`f32`].
#[derive(Hash)]
pub struct HashableFloat32(u32);

impl ToHashable for f32 {
    type Hashable = HashableFloat32;
    fn to_hashable(&self) -> Self::Hashable {
        self.to_bits().pipe(HashableFloat32)
    }
}

/// Return type of [`ToHashable`] implementation of [`f64`].
#[derive(Hash)]
pub struct HashableFloat64(u64);

impl ToHashable for f64 {
    type Hashable = HashableFloat64;
    fn to_hashable(&self) -> Self::Hashable {
        self.to_bits().pipe(HashableFloat64)
    }
}
