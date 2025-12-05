#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TupleClone` trait.

#[tuplities_derive::impl_tuple_clone]
/// A trait for cloning tuples.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait TupleClone {
    #[must_use]
    /// Clones `self` into a new instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_clone::TupleClone;
    ///
    /// let tuple = (1, "hello", vec![1, 2, 3]);
    /// let cloned_tuple = tuple.tuple_clone();
    ///
    /// assert_eq!(tuple, cloned_tuple);
    /// ```
    fn tuple_clone(&self) -> Self;
}
