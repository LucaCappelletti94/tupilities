//! A trait for removing elements at specific indices from tuples.
//!
//! This crate provides the `TupleRemove<Idx>` trait, which allows removing an element
//! at a compile-time known index from a tuple, returning the element and the
//! remaining tuple.

#![no_std]

/// A trait for removing an element at a specific index from a tuple.
///
/// This trait allows removing an element at compile-time known index `Idx`
/// from a tuple, returning the element and the remaining tuple.
///
/// # Examples
///
/// ```
/// use tuplities_remove::TupleRemove;
/// use typenum::U1;
///
/// let tuple = (1, "hello", 3.14);
/// let (removed, remainder) = TupleRemove::<U1>::remove(tuple);
/// assert_eq!(removed, "hello");
/// assert_eq!(remainder, (1, 3.14));
/// ```
#[tuplities_derive::impl_remove]
pub trait TupleRemove<Idx: typenum::Unsigned> {
    /// The type of the element being removed.
    type Type;

    /// The type of the remaining tuple after removing.
    type Remainder;

    /// Removes the element at index `Idx` from the tuple.
    ///
    /// Returns a tuple containing the removed element and the remaining tuple.
    fn remove(self) -> (Self::Type, Self::Remainder);
}
