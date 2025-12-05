//! Submodule providing the derive macro for the `TuplePartialOrd` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `TuplePartialOrd` trait implementations for all tuple sizes.
pub fn impl_tuple_partial_ord() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        quote! {
            impl<#(#type_params: PartialOrd,)*> TuplePartialOrd for (#(#type_params,)*)
            {
                #[inline]
                fn tuple_partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                    #(
                        match self.#indices.partial_cmp(&other.#indices) {
                            Some(core::cmp::Ordering::Equal) => {},
                            other => return other
                        }
                    )*
                    Some(core::cmp::Ordering::Equal)
                }
            }
        }
    })
}
