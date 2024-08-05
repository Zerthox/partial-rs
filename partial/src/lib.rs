//! Partial representations of types.
//!
//! # Usage
//! ```no_run
//! use partial::{Partial, IntoPartial, PartialOps};
//!
//! #[derive(Debug, Default, Clone, Partial)]
//! struct MyStruct {
//!     valid: bool,
//!     id: u32,
//!     name: String,
//! }
//!
//! let mut first = MyStruct {
//!     valid: true,
//!     id: 123,
//!     name: "foo".into(),
//! };
//! let second = MyStruct {
//!     valid: false,
//!     id: 456,
//!     name: "bar".into(),
//! };
//!
//! let update = second.into_partial().and(Partial::<MyStruct> {
//!     id: Some(456), // only change id
//!     ..Partial::<MyStruct>::empty()
//! });
//! value.set(update);
//! ```

pub use partial_macros::*;

/// Partial representation of the type.
pub type Partial<T> = <T as IntoPartial>::Partial;

/// A trait for converting a type into a partial type.
pub trait IntoPartial {
    /// Partial representation of the type.
    type Partial: PartialOps;

    /// Converts the type into its partial representation.
    fn into_partial(self) -> Self::Partial;

    /// Sets values from partial representation.
    fn set(&mut self, partial: Self::Partial);
}

/// A trait for operations on a partial type.
pub trait PartialOps: Sized {
    /// Creates an empty partial.
    fn empty() -> Self;

    /// Checks whether the partial is empty.
    fn is_empty(&self) -> bool;

    /// Returns a partial with values of `other` if both partials have them and [`None`] otherwise.
    #[inline]
    fn and(mut self, other: Self) -> Self {
        self.set_and(other);
        self
    }

    /// Sets values to `other` if both partials have them and [`None`] otherwise.
    fn set_and(&mut self, other: Self);

    /// Returns a partial with values of either partial or [`None`] if neither has them present.
    #[inline]
    fn or(mut self, other: Self) -> Self {
        self.set_or(other);
        self
    }

    /// Sets values to either partial or [`None`] if neither has them present.
    fn set_or(&mut self, other: Self);
}
