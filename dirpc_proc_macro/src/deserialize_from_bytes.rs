use quote::quote;
use syn::{spanned::Spanned, DeriveInput};

pub fn get(input: proc_macro::TokenStream) -> syn::Result<proc_macro::TokenStream> {
    let input: DeriveInput = syn::parse(input)?;
    if let syn::Data::Union(_) = &input.data {
        return Err(syn::Error::new(input.span(), "Unions are not supported"));
    }
    let name = input.ident;
    match input.data {
        syn::Data::Struct(input) => {
            Ok(match input.fields {
                syn::Fields::Named(fields) => {
                    let mut fields = fields.named.into_iter().map(|f| f.ident.unwrap()).map(|f| (f.to_string(), f)).collect::<Vec<_>>();
                    fields.sort_by(|(a, _), (b, _)| a.cmp(b));
                    let self_fields = fields.iter().map(|(_, f)| f);
                    let fields = fields.iter().map(|(_, f)| quote! { let #f = dirpc::DeserializeFromBytes::deserialize_from_bytes(data)?; });
                    quote! {
                        impl dirpc::DeserializeFromBytes for #name {
                            #[inline]
                            fn deserialize_from_bytes(data: &mut bytes::Bytes) -> anyhow::Result<Self> {
                                #(#fields)*
                                Ok(Self {#(#self_fields),*})
                            }
                        }
                    }.into()
                }
                syn::Fields::Unnamed(fields) => {
                    let fields = fields.unnamed.iter().map(|_| {
                        quote! { dirpc::DeserializeFromBytes::deserialize_from_bytes(data)? }
                    });
                    quote! {
                        impl dirpc::DeserializeFromBytes for #name {
                            #[inline]
                            fn deserialize_from_bytes(data: &mut bytes::Bytes) -> anyhow::Result<Self> {
                                Ok(Self(#(#fields),*))
                            }
                        }
                    }.into()
                }
                syn::Fields::Unit => {
                    quote! {
                        impl dirpc::DeserializeFromBytes for #name {
                            #[inline]
                            fn deserialize_from_bytes(data: &mut bytes::Bytes) -> anyhow::Result<Self> {
                                Ok(Self)
                            }
                        }
                    }.into()
                }
            })
        }
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => unreachable!(),
    }
}