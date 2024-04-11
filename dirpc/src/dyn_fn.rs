use dirpc_proc_macro::dyn_fn_impl;

use crate::{
    description::{FunctionDescription, GetTypeDescription},
    inject::{self, GetTupleForExport, ToArgsDescription},
    request::Request,
    rpc_serde::{RpcDeserializer, RpcSerializer},
};

pub type DynFunction<Context, RequestState> = Box<
    dyn Fn(
            &Context,
            Request<RequestState>,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = anyhow::Result<bytes::Bytes>> + Send>,
        > + Send
        + Sync,
>;

pub trait IntoDynFunction<Context, RequestState, PhantomGeneric> {
    type NameTuple;
    fn into_dyn_fn<Serializer: RpcSerializer, Deserializer: RpcDeserializer>(
        self,
    ) -> DynFunction<Context, RequestState>;
    fn get_type_description(names: Self::NameTuple) -> FunctionDescription;
}

impl<Context, RequestState, Fut, R, E, F> IntoDynFunction<Context, RequestState, ((R, E),)> for F
where
    Fut: std::future::Future<Output = Result<R, E>> + Send + 'static,
    R: serde::Serialize + GetTypeDescription,
    E: ToString,
    F: FnOnce() -> Fut + Clone + Send + Sync + 'static,
{
    type NameTuple = ();
    fn into_dyn_fn<Serializer: RpcSerializer, Deserializer: RpcDeserializer>(
        self,
    ) -> DynFunction<Context, RequestState> {
        Box::new(move |_ctx, _req| {
            let function = self.clone();
            Box::pin(async move {
                match function().await {
                    Ok(v) => Serializer::serialize((v, ())),
                    Err(e) => {
                        let e = e.to_string();
                        eprintln!("ERROR: {e}");
                        Serializer::serialize(((), e))
                    }
                }
            })
        })
    }
    fn get_type_description(_names: Self::NameTuple) -> FunctionDescription {
        FunctionDescription {
            args_types: vec![],
            return_type: R::get_type_description(),
        }
    }
}

dyn_fn_impl!(16);
