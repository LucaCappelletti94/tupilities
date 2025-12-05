//! Submodule providing the derive macro for the `TupleOrd` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `TupleOrd` trait implementations for all tuple sizes.
pub fn impl_tuple_ord() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        quote! {
            impl<#(#type_params: Ord,)*> TupleOrd for (#(#type_params,)*)
            {
                #[inline]
                fn tuple_cmp(&self, other: &Self) -> core::cmp::Ordering {
                    #(
                        match self.#indices.cmp(&other.#indices) {
                            core::cmp::Ordering::Equal => {},
                            non_eq => return non_eq,
                        }
                    )*
                    core::cmp::Ordering::Equal
                }
            }
        }
    })
}
