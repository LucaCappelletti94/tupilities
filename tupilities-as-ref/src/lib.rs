#![no_std]

//! Tupilities suite crate providing the `TupleAsRef` trait.

#[tupilities_derive::impl_tuple_as_ref]
/// A trait for converting tuples to references of a common type.
pub trait TupleAsRef<'a, T: 'a> {
    /// Returns a tuple of references to the target type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tupilities_as_ref::TupleAsRef;
    ///
    /// let tuple = ("hello", "world".to_string());
    /// let refs: (&str, &str) = tuple.as_tuple_ref();
    ///
    /// assert_eq!(refs, ("hello", "world"));
    /// ```
    fn as_tuple_ref(&'a self) -> T;
}
