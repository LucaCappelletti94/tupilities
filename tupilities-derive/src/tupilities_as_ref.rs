//! Submodule providing the derive macro for the `TupleAsRef` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params, type_params_with_prefix};

/// Generate `TupleAsRef` trait implementations for all tuple sizes.
pub fn impl_tuple_as_ref() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let ref_target_params = type_params_with_prefix(size, "R");
        let indices = indices(size);

        quote! {
            impl<'a, #(#ref_target_params: 'a + ?Sized,)* #(#type_params: AsRef<#ref_target_params>,)*> TupleAsRef<'a, (#(&'a #ref_target_params,)*)> for (#(#type_params,)*)
            where Self: 'a
            {
                #[inline]
                fn as_tuple_ref(&'a self) -> (#(&'a #ref_target_params,)*) {
                    (#(self.#indices.as_ref(),)*)
                }
            }
        }
    })
}
