mod serialize_to_bytes;
mod deserialize_from_bytes;
mod get_type_description;

#[proc_macro_derive(SerializeToBytes)]
pub fn serialize_to_bytes_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	serialize_to_bytes::get(input).unwrap_or_else(|e| syn::Error::into_compile_error(e).into()).into()
}
#[proc_macro_derive(DeserializeFromBytes)]
pub fn deserialize_from_bytes_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	deserialize_from_bytes::get(input).unwrap_or_else(|e| syn::Error::into_compile_error(e).into()).into()
}
#[proc_macro_derive(GetTypeDescription)]
pub fn get_type_description_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	get_type_description::get(input).unwrap_or_else(|e| syn::Error::into_compile_error(e).into()).into()
}

