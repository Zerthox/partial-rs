mod args;

use self::args::*;
use attribute_derive::FromAttr;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Member, Type};

/// Derive macro generating an associated partial struct and implementations.
#[proc_macro_derive(Partial, attributes(partial))]
pub fn partial(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs,
        vis,
        ident: parent,
        generics,
        data,
    } = parse_macro_input!(input as DeriveInput);

    let fields = match data {
        Data::Struct(data) => data.fields,
        Data::Enum(data) => {
            return syn::Error::new_spanned(
                &data.enum_token,
                "enum not supported by partial derive",
            )
            .into_compile_error()
            .into()
        }
        Data::Union(data) => {
            return syn::Error::new_spanned(
                &data.union_token,
                "union not supported by partial derive",
            )
            .into_compile_error()
            .into()
        }
    };

    let args = match Args::from_attributes(&attrs) {
        Ok(args) => args,
        Err(err) => return err.into_compile_error().into(),
    };

    let partial_fields = fields.iter().map(|field| {
        let ty = &field.ty;
        Field {
            ty: Type::Verbatim(quote! { ::core::option::Option<#ty> }),
            ..field.clone()
        }
    });

    let members = fields
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, field)| match field.ident {
            Some(ident) => Member::Named(ident),
            None => Member::Unnamed(i.into()),
        })
        .collect::<Vec<_>>();

    let partial = args.name(&parent);
    let attributes = args.attributes(&parent);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let partial_struct = match fields {
        Fields::Named(_) => quote! {
            #vis struct #partial #ty_generics #where_clause {
                #(#partial_fields),*
            }
        },
        Fields::Unnamed(_) => quote! {
            #vis struct #partial #ty_generics (
                #(#partial_fields),*
            ) #where_clause;
        },
        Fields::Unit => quote! {
            #vis struct #partial #ty_generics #where_clause;
        },
    };

    quote! {
        #[automatically_derived]
        #attributes
        #partial_struct

        #[automatically_derived]
        impl #impl_generics #partial #ty_generics #where_clause {
            /// Creates an empty partial.
            #[inline]
            const fn empty() -> Self {
                Self {
                    #( #members: ::core::option::Option::None ),*
                }
            }

            /// Checks if the partial is empty.
            #[inline]
            const fn is_empty(&self) -> bool {
                #( self.#members.is_none() )&&*
            }
        }

        #[automatically_derived]
        impl #impl_generics ::core::default::Default for #partial #ty_generics #where_clause {
            #[inline]
            fn default() -> Self {
                Self::empty()
            }
        }

        #[automatically_derived]
        impl #impl_generics ::partial::PartialOps for #partial #ty_generics #where_clause {
            #[inline]
            fn empty() -> Self {
                Self::empty()
            }

            #[inline]
            fn is_empty(&self) -> bool {
                self.is_empty()
            }

            #[inline]
            fn set_and(&mut self, other: Self) {
                #( self.#members = self.#members.as_ref().and(other.#members) );*
            }

            #[inline]
            fn set_or(&mut self, other: Self) {
                #( if self.#members.is_none() {
                    self.#members = other.#members
                } );*
            }
        }

        #[automatically_derived]
        impl #impl_generics ::partial::IntoPartial for #parent #ty_generics #where_clause {
            type Partial = #partial #ty_generics;

            #[inline]
            fn into_partial(self) -> Self::Partial {
                Self::Partial {
                    #( #members: ::core::option::Option::Some(self.#members) ),*
                }
            }

            #[inline]
            fn set(&mut self, partial: Self::Partial) {
                #( if let ::core::option::Option::Some(value) = partial.#members {
                    self.#members = value;
                } )*
            }
        }
    }
    .into()
}
