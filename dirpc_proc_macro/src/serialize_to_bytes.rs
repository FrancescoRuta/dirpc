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
                    let fields1 = fields.iter().map(|(_, f)| quote! { dirpc::SerializeToBytes::serialize_to_bytes(self.#f, serialization_helper)?; });
                    let fields2 = fields.iter().map(|(_, f)| quote! { dirpc::SerializeToBytes::serialize_to_bytes(&self.#f, serialization_helper)?; });
                    quote! {
                        impl dirpc::SerializeToBytes for #name {
                            #[inline]
                            fn serialize_to_bytes(self, serialization_helper: &mut dirpc::SerializationHelper) -> dirpc::anyhow::Result<()> {
                                #(#fields1)*
                                Ok(())
                            }
                        }
                        impl dirpc::SerializeToBytes for &#name {
                            #[inline]
                            fn serialize_to_bytes(self, serialization_helper: &mut dirpc::SerializationHelper) -> dirpc::anyhow::Result<()> {
                                #(#fields2)*
                                Ok(())
                            }
                        }
                    }.into()
                }
                syn::Fields::Unnamed(fields) => {
                    let fields1 = fields.unnamed.iter().enumerate().map(|(i, _)| {
                        let i = syn::Index::from(i);
                        quote! { dirpc::SerializeToBytes::serialize_to_bytes(self.#i, serialization_helper)?; }
                    });
                    let fields2 = fields.unnamed.iter().enumerate().map(|(i, _)| {
                        let i = syn::Index::from(i);
                        quote! { dirpc::SerializeToBytes::serialize_to_bytes(&self.#i, serialization_helper)?; }
                    });
                    quote! {
                        impl dirpc::SerializeToBytes for #name {
                            #[inline]
                            fn serialize_to_bytes(self, serialization_helper: &mut dirpc::SerializationHelper) -> dirpc::anyhow::Result<()> {
                                #(#fields1)*
                                Ok(())
                            }
                        }
                        impl dirpc::SerializeToBytes for &#name {
                            #[inline]
                            fn serialize_to_bytes(self, serialization_helper: &mut dirpc::SerializationHelper) -> dirpc::anyhow::Result<()> {
                                #(#fields2)*
                                Ok(())
                            }
                        }
                    }.into()
                }
                syn::Fields::Unit => {
                    quote! {
                        impl dirpc::SerializeToBytes for #name {
                            #[inline]
                            fn serialize_to_bytes(self, _serialization_helper: &mut dirpc::SerializationHelper) -> dirpc::anyhow::Result<()> {
                                Ok(())
                            }
                        }
                        impl dirpc::SerializeToBytes for &#name {
                            #[inline]
                            fn serialize_to_bytes(self, _serialization_helper: &mut dirpc::SerializationHelper) -> dirpc::anyhow::Result<()> {
                                Ok(())
                            }
                        }
                    }.into()
                }
            })
        }
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => unreachable!()
    }
}