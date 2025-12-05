//! Submodule providing the derive macro for the `TupleInsert` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `TupleInsert<Idx, T>` trait implementations for all tuple sizes and indices.
pub fn impl_insert() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);
        (0..=size)
            .map(|u_idx| {
                let before = &type_params[..u_idx];
                let after = &type_params[u_idx..];
                let before_indices = &indices[..u_idx];
                let after_indices = &indices[u_idx..];
                let typenum_ident =
                    syn::Ident::new(&format!("U{u_idx}"), proc_macro2::Span::call_site());
                let typenum: syn::Path = syn::parse_quote!(typenum::#typenum_ident);
                quote! {
                    impl<T, #(#type_params,)*> TupleInsert<#typenum, T> for (#(#type_params,)*)
                    {
                        type Output = (#(#before,)* T, #(#after,)*);

                        #[inline]
                        fn insert(self, value: T) -> Self::Output {
                            (#(self.#before_indices,)* value, #(self.#after_indices,)*)
                        }
                    }
                }
            })
            .collect()
    })
}
