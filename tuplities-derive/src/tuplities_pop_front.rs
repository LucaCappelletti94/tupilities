//! Submodule providing the derive macro for the `PopFront` trait.

use quote::quote;

use crate::tuple_size::{generate_non_empty, indices, type_params};

/// Generate `PopFront` trait implementations for all tuple sizes starting from 1.
pub fn impl_pop_front() -> proc_macro2::TokenStream {
    generate_non_empty(|size| {
        let type_params = type_params(size);
        let indices = indices(size);
        let (first, others) = type_params.split_first().unwrap();
        let (first_index, other_indices) = indices.split_first().unwrap();

        quote! {
            impl<#(#type_params,)*> PopFront for (#(#type_params,)*)
            {
                type Front = #first;
                type Tail = (#(#others,)*);

                #[inline]
                fn pop_front(self) -> (Self::Front, Self::Tail) {
                    (self.#first_index, (#(self.#other_indices,)*))
                }
            }
        }
    })
}
