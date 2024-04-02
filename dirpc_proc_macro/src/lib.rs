mod get_type_description;

#[proc_macro_derive(GetTypeDescription)]
pub fn get_type_description_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	get_type_description::get(input).unwrap_or_else(|e| syn::Error::into_compile_error(e).into()).into()
}

