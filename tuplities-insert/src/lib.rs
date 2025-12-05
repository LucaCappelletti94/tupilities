//! A trait for inserting elements at specific indices into tuples.
//!
//! This crate provides the `TupleInsert<Idx, T>` trait, which allows inserting an element
//! at a compile-time known index into a tuple, returning the tuple with the element inserted.

#![no_std]

/// A trait for inserting an element at a specific index into a tuple.
///
/// This trait allows inserting an element at compile-time known index `Idx`
/// into a tuple, returning the tuple with the element inserted.
///
/// # Examples
///
/// ```
/// use tuplities_insert::TupleInsert;
/// use typenum::U1;
///
/// let tuple = (1, 3.14);
/// let inserted = TupleInsert::<U1, _>::insert(tuple, "hello");
/// assert_eq!(inserted, (1, "hello", 3.14));
/// ```
#[tuplities_derive::impl_insert]
pub trait TupleInsert<Idx: typenum::Unsigned, T> {
    /// The type of the tuple after inserting the element.
    type Output;

    /// Inserts the element at index `Idx` into the tuple.
    ///
    /// Returns the tuple with the element inserted at the specified index.
    fn insert(self, value: T) -> Self::Output;
}
