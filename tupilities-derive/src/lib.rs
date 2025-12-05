//! Procedural macros for the Tupilities suite.

use proc_macro::TokenStream;

mod tupilities_clone;
mod tupilities_copy;
mod tupilities_debug;
mod tuple_size;

/// Generate `TableIndex` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_clone(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tupilities_clone::impl_tuple_clone());
    item.into()
}

/// Generate `TupleCopy` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_copy(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tupilities_copy::impl_tuple_copy());
    item.into()
}

/// Generate `TupleDebug` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_debug(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tupilities_debug::impl_tuple_debug());
    item.into()
}
