//! Submodule providing the derive macro for the `TupleDebug` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `TupleDebug` trait implementations for all tuple sizes.
pub fn impl_tuple_debug() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        quote! {
            impl<#(#type_params: std::fmt::Debug,)*> TupleDebug for (#(#type_params,)*)
            {
                #[inline]
                fn tuple_debug(&self) -> String {
                    let parts: Vec<String> = vec![
                        #(
                            format!("{:?}", self.#indices),
                        )*
                    ];
                    format!(
                        "({})",
                        parts.join(", ")
                    )
                }
            }
        }
    })
}
