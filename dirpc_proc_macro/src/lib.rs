use quote::{quote, quote_spanned};

mod get_type_description;

#[proc_macro_derive(GetTypeDescription)]
pub fn get_type_description_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	get_type_description::get(input).unwrap_or_else(|e| syn::Error::into_compile_error(e).into()).into()
}

#[proc_macro]
pub fn dyn_fn_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let number: syn::LitInt = syn::parse(input).unwrap();
	let out = (0..number.base10_parse().unwrap()).map(|n| {
		let n = n + 1;
		let t = (0..n).map(|i| syn::Ident::new(&format!("T{i}"), number.span())).collect::<Vec<_>>();
		let t_idx = (0..n).map(|i| syn::Ident::new(&format!("t{i}"), number.span())).collect::<Vec<_>>();
		let mut cl_build = Vec::with_capacity(n);
		let span = number.span();
		cl_build.push(quote_spanned! {span => ((), T0::TExportDefinition)});
		for i in 1..n {
			let prev = &cl_build[i - 1];
			let t = syn::Ident::new(&format!("T{i}"), number.span());
			cl_build.push(quote_spanned! {span => (<#prev as GetTupleForExport<StrType>>::Result, #t::TExportDefinition)});
		}
		let name_tuple = cl_build.last().unwrap();
		quote_spanned! {
			span =>
			impl<Context, RequestState, #(#t,)* Fut, R, E, F, StrType> IntoDynFunction<Context, RequestState, ((#(#t,)* R, E), StrType)> for F
			where
				#(#t: inject::Inject<Context, RequestState> + Send + 'static,)*
				Fut: std::future::Future<Output = Result<R, E>> + Send + 'static,
				R: serde::Serialize + GetTypeDescription,
				E: ToString,
				F: FnOnce(#(#t,)*) -> Fut + Clone + Send + Sync + 'static,
				StrType: Into<String>,
				#(
					#cl_build: GetTupleForExport<StrType>,
				)*
			{
				type NameTuple = <#name_tuple as GetTupleForExport<StrType>>::Result;
				fn into_dyn_fn<Serializer: RpcSerializer, Deserializer: RpcDeserializer>(self) -> DynFunction<Context, RequestState> {
					Box::new(move |ctx, mut req| {
						#(let #t_idx = #t::inject::<Deserializer>(ctx, &mut req);)*
						let function = self.clone();
						Box::pin(async move {
							match function(#(#t_idx?,)*).await {
								Ok(v) => Serializer::serialize((v, ())),
								Err(e) => {
									let e = e.to_string();
									eprintln!("ERROR: {e}");
									Serializer::serialize(((), e))
								},
							}
						})
					})
				}
				fn get_type_description(names: Self::NameTuple) -> FunctionDescription {
					FunctionDescription {
						args_types: <Self::NameTuple as ToArgsDescription>::to_args_description(
							names,
							[#(<#t as inject::Inject<_, _>>::get_type_description(),)*]
						),
						return_type: R::get_type_description(),
					}
				}
			}
		}
	});
	let result = quote!(#(#out)*);
	result.into()
}