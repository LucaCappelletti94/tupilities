//! Submodule providing the derive macro for the `TupleCopy` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `TupleCopy` trait implementations for all tuple sizes.
pub fn impl_tuple_copy() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        quote! {
            impl<#(#type_params: Copy,)*> TupleCopy for (#(#type_params,)*)
            {
                #[inline]
                fn tuple_copy(&self) -> Self {
                    ( #(self.#indices,)* )
                }
            }
        }
    })
}
