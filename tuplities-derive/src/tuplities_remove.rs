//! Submodule providing the derive macro for the `TupleRemove` trait.

use quote::quote;

use crate::tuple_size::{generate_non_empty, type_params};

/// Generate `TupleRemove<Idx>` trait implementations for all tuple sizes and indices.
pub fn impl_remove() -> proc_macro2::TokenStream {
    generate_non_empty(|size| {
        let type_params = type_params(size);
        (0..size)
            .map(|u_idx| {
                let removed_type = &type_params[u_idx];
                let others = type_params
                    .iter()
                    .enumerate()
                    .filter_map(|(i, t)| if i == u_idx { None } else { Some(t) })
                    .collect::<Vec<_>>();
                let removed_index = syn::Index::from(u_idx);
                let other_indices = (0..size)
                    .filter_map(|i| {
                        if i == u_idx {
                            None
                        } else {
                            Some(syn::Index::from(i))
                        }
                    })
                    .collect::<Vec<_>>();
                let typenum_ident =
                    syn::Ident::new(&format!("U{u_idx}"), proc_macro2::Span::call_site());
                let typenum: syn::Path = syn::parse_quote!(typenum::#typenum_ident);
                quote! {
                    impl<#(#type_params,)*> TupleRemove<#typenum> for (#(#type_params,)*)
                    {
                        type Type = #removed_type;
                        type Remainder = (#(#others,)*);

                        #[inline]
                        fn remove(self) -> (Self::Type, Self::Remainder) {
                            (self.#removed_index, (#(self.#other_indices,)*))
                        }
                    }
                }
            })
            .collect()
    })
}
