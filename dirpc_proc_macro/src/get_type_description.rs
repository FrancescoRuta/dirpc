use quote::quote;
use syn::{spanned::Spanned, DeriveInput};

pub fn get(input: proc_macro::TokenStream) -> syn::Result<proc_macro::TokenStream> {
    let input: DeriveInput = syn::parse(input)?;
    if let syn::Data::Union(_) = &input.data {
        return Err(syn::Error::new(input.span(), "Unions are not supported"));
    }
    let name = input.ident;
    match input.data {
        syn::Data::Struct(input) => Ok(match input.fields {
            syn::Fields::Named(fields) => {
                let fields1 = fields.named.iter().map(|f| {
                        let ty = &f.ty;
                        let field_name = f.ident.as_ref().unwrap().to_string();
                        quote! { fields.insert(String::from(#field_name), <#ty as dirpc::GetTypeDescription>::get_type_description()); }
                    });
                let fields2 = fields1.clone();
                let len = fields.named.len();
                quote! {
                        impl dirpc::GetTypeDescription for #name {
                            fn get_type_description() -> dirpc::TypeDescription {
                                let mut fields = ::std::collections::hash_map::HashMap::with_capacity(#len);
                                #(#fields1)*
                                dirpc::TypeDescription {
                                    module_path: ::std::string::String::from(module_path!()),
                                    name: ::std::string::String::from((stringify!(#name))),
                                    typeinfo: dirpc::TypeInfo::Object(fields),
                                }
                            }
                        }
                        impl dirpc::GetTypeDescription for &#name {
                            fn get_type_description() -> dirpc::TypeDescription {
                                let mut fields = ::std::collections::hash_map::HashMap::with_capacity(#len);
                                #(#fields2)*
                                dirpc::TypeDescription {
                                    module_path: ::std::string::String::from(module_path!()),
                                    name: ::std::string::String::from((stringify!(#name))),
                                    typeinfo: dirpc::TypeInfo::Object(fields),
                                }
                            }
                        }
                    }.into()
            }
            syn::Fields::Unnamed(fields) => {
                let fields1 = fields.unnamed.iter().map(|f| {
                    let ty = &f.ty;
                    quote! { <#ty as dirpc::GetTypeDescription>::get_type_description() }
                });
                let fields2 = fields1.clone();
                quote! {
                    impl dirpc::GetTypeDescription for #name {
                        fn get_type_description() -> dirpc::TypeDescription {
                            dirpc::TypeDescription {
                                module_path: ::std::string::String::from(module_path!()),
                                name: ::std::string::String::from((stringify!(#name))),
                                typeinfo: dirpc::TypeInfo::Tuple(vec![#(#fields1),*]),
                            }
                        }
                    }
                    impl dirpc::GetTypeDescription for &#name {
                        fn get_type_description() -> dirpc::TypeDescription {
                            dirpc::TypeDescription {
                                module_path: ::std::string::String::from(module_path!()),
                                name: ::std::string::String::from((stringify!(#name))),
                                typeinfo: dirpc::TypeInfo::Tuple(vec![#(#fields2),*]),
                            }
                        }
                    }
                }
                .into()
            }
            syn::Fields::Unit => quote! {
                impl dirpc::GetTypeDescription for #name {
                    fn get_type_description() -> dirpc::TypeDescription {
                        dirpc::TypeDescription {
                            module_path: ::std::string::String::from(module_path!()),
                            name: ::std::string::String::from(stringify!(#name)),
                            typeinfo: dirpc::TypeInfo::BaseType(dirpc::BaseTypeDescription::Void),
                        }
                    }
                }
                impl dirpc::GetTypeDescription for &#name {
                    fn get_type_description() -> dirpc::TypeDescription {
                        dirpc::TypeDescription {
                            module_path: ::std::string::String::from(module_path!()),
                            name: ::std::string::String::from(stringify!(#name)),
                            typeinfo: dirpc::TypeInfo::BaseType(dirpc::BaseTypeDescription::Void),
                        }
                    }
                }
            }
            .into(),
        }),
        syn::Data::Enum(input) => Ok({
            let variants = input
                .variants
                .into_iter()
                .map(|v| {
                    let ident = v.ident.to_string();
                    v.discriminant.as_ref()
                        .map(|(_, d)| {
                            if let syn::Expr::Lit(d) = &d {
                                if let syn::Lit::Int(d) = &d.lit {
                                    d.base10_parse::<u32>().map(|d| quote! { (::std::string::String::from(#ident), #d) } )
                                } else {
                                    unreachable!();
                                }
                            } else {
                                unreachable!();
                            }
                        })
                        .ok_or_else(|| {
                            syn::Error::new(
                                v.span(),
                                "All variants must contain an explicit discriminant.",
                            )
                        })
                })
                .collect::<Result<Result<Vec<_>, _>, _>>()??;
            quote! {
                impl dirpc::GetTypeDescription for #name {
                    fn get_type_description() -> dirpc::TypeDescription {
                        dirpc::TypeDescription {
                            module_path: ::std::string::String::from(module_path!()),
                            name: ::std::string::String::from((stringify!(#name))),
                            typeinfo: dirpc::TypeInfo::Enum(vec![#(#variants),*]),
                        }
                    }
                }
                impl dirpc::GetTypeDescription for &#name {
                    fn get_type_description() -> dirpc::TypeDescription {
                        dirpc::TypeDescription {
                            module_path: ::std::string::String::from(module_path!()),
                            name: ::std::string::String::from((stringify!(#name))),
                            typeinfo: dirpc::TypeInfo::Enum(vec![#(#variants),*]),
                        }
                    }
                }
            }.into()
        }),
        syn::Data::Union(_) => unreachable!(),
    }
}