mod args;

use self::args::*;
use attribute_derive::FromAttr;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Member, Type};

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

    let struct_fields = match data {
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

    let args = match StructArgs::from_attributes(&attrs) {
        Ok(args) => args,
        Err(err) => return err.into_compile_error().into(),
    };

    struct Field {
        args: FieldArgs,
        field: syn::Field,
        member: Member,
    }

    let fields = struct_fields
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, mut field)| {
            let args = FieldArgs::remove_attributes(&mut field.attrs)?;
            let member = match field.ident.clone() {
                Some(ident) => Member::Named(ident),
                None => Member::Unnamed(i.into()),
            };
            Ok(Field {
                args,
                field,
                member,
            })
        })
        .collect::<syn::Result<Vec<_>>>();
    let fields = match fields {
        Ok(fields) => fields,
        Err(err) => return err.into_compile_error().into(),
    };

    let partial_fields = fields.iter().map(|Field { args, field, .. }| {
        let ty = &field.ty;
        syn::Field {
            ty: if args.flatten {
                Type::Verbatim(quote! { ::partial::Partial<#ty> })
            } else {
                Type::Verbatim(quote! { ::core::option::Option<#ty> })
            },
            ..field.clone()
        }
    });

    let partial = args.name(&parent);
    let attributes = args.attributes(&parent);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let partial_struct = match struct_fields {
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

    let members_normal = fields
        .iter()
        .filter(|Field { args, .. }| !args.flatten)
        .map(|Field { member, .. }| member)
        .collect::<Vec<_>>();

    let members_flatten = fields
        .iter()
        .filter(|Field { args, .. }| args.flatten)
        .map(|Field { member, .. }| member)
        .collect::<Vec<_>>();

    let members_flatten_ty = fields
        .iter()
        .filter(|Field { args, .. }| args.flatten)
        .map(|Field { field, .. }| field.ty.clone())
        .collect::<Vec<_>>();

    let qualifiers = fields
        .iter()
        .all(|Field { args, .. }| !args.flatten)
        .then_some(quote! { const });

    quote! {
        #[automatically_derived]
        #attributes
        #partial_struct

        #[automatically_derived]
        impl #impl_generics #partial #ty_generics #where_clause {
            /// Creates an empty partial.
            #[inline]
            pub #qualifiers fn empty() -> Self {
                Self {
                    #( #members_normal: ::core::option::Option::None ),*
                    #( , #members_flatten: <::partial::Partial<#members_flatten_ty> as ::partial::PartialOps>::empty() )*
                }
            }

            /// Checks if the partial is empty.
            #[inline]
            pub #qualifiers fn is_empty(&self) -> bool {
                #( self.#members_normal.is_none() )&&*
                #( && ::partial::PartialOps::is_empty(&self.#members_flatten) )*
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
                #( self.#members_normal = self.#members_normal.as_ref().and(other.#members_normal) );*
                #( ; ::partial::PartialOps::set_and(&mut self.#members_flatten, other.#members_flatten) )*
            }

            #[inline]
            fn set_or(&mut self, other: Self) {
                #( if self.#members_normal.is_none() {
                    self.#members_normal = other.#members_normal
                } )*
                #( ::partial::PartialOps::set_or(&mut self.#members_flatten, other.#members_flatten) );*
            }
        }

        #[automatically_derived]
        impl #impl_generics ::partial::IntoPartial for #parent #ty_generics #where_clause {
            type Partial = #partial #ty_generics;

            #[inline]
            fn into_partial(self) -> Self::Partial {
                Self::Partial {
                    #( #members_normal: ::core::option::Option::Some(self.#members_normal) ),*
                    #( , #members_flatten: ::partial::IntoPartial::into_partial(self.#members_flatten) )*
                }
            }

            #[inline]
            fn set(&mut self, partial: Self::Partial) {
                #( if let ::core::option::Option::Some(value) = partial.#members_normal {
                    self.#members_normal = value;
                } )*
                #( ::partial::IntoPartial::set(&mut self.#members_flatten, partial.#members_flatten) );*
            }
        }
    }
    .into()
}
