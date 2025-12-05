//! Submodule providing the derive macro for the `TupleDefault` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, type_params};

/// Generate `TupleDefault` trait implementations for all tuple sizes.
pub fn impl_tuple_default() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);

        quote! {
            impl<#(#type_params: Default,)*> TupleDefault for (#(#type_params,)*)
            {
                #[inline]
                fn tuple_default() -> Self {
                    ( #(<#type_params as Default>::default(),)* )
                }
            }
        }
    })
}
