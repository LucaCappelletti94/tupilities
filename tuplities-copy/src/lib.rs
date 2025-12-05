#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TupleCopy` trait.

use tuplities_clone::TupleClone;

#[tuplities_derive::impl_tuple_copy]
/// A trait for copying tuples.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait TupleCopy: TupleClone {
    #[must_use]
    /// Copies `self` into a new instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_copy::TupleCopy;
    ///
    /// let tuple = (1, "hello", 3.14);
    /// let copied_tuple = tuple.tuple_copy();
    ///
    /// assert_eq!(tuple, copied_tuple);
    /// ```
    fn tuple_copy(&self) -> Self;
}
