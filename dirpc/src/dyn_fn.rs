use crate::{rpc_serde::{RpcDeserializer, RpcSerializer}, description::{FunctionDescription, GetTypeDescription}, for_all_functions, request::Request};

pub type DynFunction<Context, RequestState> = Box<dyn Fn(&Context, Request<RequestState>) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<bytes::Bytes>> + Send>> + Send + Sync>;

pub trait IntoDynFunction<Context, RequestState, PhantomGeneric> {
    type NameTuple;
    fn into_dyn_fn<Serializer: RpcSerializer, Deserializer: RpcDeserializer>(self) -> DynFunction<Context, RequestState>;
    fn get_type_description(names: Self::NameTuple) -> FunctionDescription;
}
/*
impl<Context, RequestState, Fut, R, F> IntoDynFunction<Context, RequestState, ((R, ), )> for F
where
    Fut: std::future::Future<Output = R> + Send + Sync + 'static,
    R: serde::Serialize + GetTypeDescription,
    F: FnOnce() -> Fut + Clone + Send + Sync + 'static,
{
    type NameTuple = ();
    fn into_dyn_fn<Serializer: RpcSerializer, Deserializer: RpcDeserializer>(self) -> DynFunction<Context, RequestState> {
        Box::new(move |_ctx, _req| {
            let function = self.clone();
            Box::pin(async move {
                Serializer::serialize(function().await)
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
*/
impl<Context, RequestState, Fut, R, E, F> IntoDynFunction<Context, RequestState, ((R, E), )> for F
where
    Fut: std::future::Future<Output = Result<R, E>> + Send + 'static,
    R: serde::Serialize + GetTypeDescription,
    E: ToString,
    F: FnOnce() -> Fut + Clone + Send + Sync + 'static,
{
    type NameTuple = ();
    fn into_dyn_fn<Serializer: RpcSerializer, Deserializer: RpcDeserializer>(self) -> DynFunction<Context, RequestState> {
        Box::new(move |_ctx, _req| {
            let function = self.clone();
            Box::pin(async move {
                match function().await {
                    Ok(v) => Serializer::serialize(Ok::<R, String>(v)),
                    Err(e) => {
                        let e = e.to_string();
                        eprintln!("ERROR: {e}");
                        Serializer::serialize(Err::<R, String>(e))
                    },
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

macro_rules! type_as_string {
    ( $t: ty ) => { StrType };
}

macro_rules! dyn_fn_impl {
    ( $( $t:ident $t_idx:ident; )* ) => {
        /*impl<Context, RequestState, $($t,)* Fut, R, F, StrType> IntoDynFunction<Context, RequestState, ($($t,)* R, StrType)> for F
        where
            $($t: $crate::inject::Inject<Context, RequestState> + GetTypeDescription + Send + Sync + 'static,)*
            Fut: std::future::Future<Output = R> + Send + Sync + 'static,
            R: serde::Serialize + GetTypeDescription,
            F: FnOnce($($t,)*) -> Fut + Clone + Send + Sync + 'static,
            StrType: Into<String>,
        {
            type NameTuple = ($(type_as_string!($t),)*);
            fn into_dyn_fn<Serializer: RpcSerializer, Deserializer: RpcDeserializer>(self) -> DynFunction<Context, RequestState> {
                Box::new(move |ctx, mut req| {
                    $(let $t_idx = $t::inject::<Deserializer>(ctx, &mut req);)*
                    let function = self.clone();
                    Box::pin(async move {
                        Serializer::serialize(function($($t_idx?,)*).await)
                    })
                })
            }
            fn get_type_description(names: Self::NameTuple) -> FunctionDescription {
                #[allow(non_snake_case)]
                let ($($t,)*) = names;
                FunctionDescription {
                    args_types: vec![$(($t::EXPORT_DEFINITION, $t.into(), $t::get_type_description()),)*],
                    return_type: R::get_type_description(),
                }
            }
        }*/
        impl<Context, RequestState, $($t,)* Fut, R, E, F, StrType> IntoDynFunction<Context, RequestState, (($($t,)* R, E), StrType)> for F
        where
            $($t: $crate::inject::Inject<Context, RequestState> + GetTypeDescription + Send + 'static,)*
            Fut: std::future::Future<Output = Result<R, E>> + Send + 'static,
            R: serde::Serialize + GetTypeDescription,
            E: ToString,
            F: FnOnce($($t,)*) -> Fut + Clone + Send + Sync + 'static,
            StrType: Into<String>,
        {
            type NameTuple = ($(type_as_string!($t),)*);
            fn into_dyn_fn<Serializer: RpcSerializer, Deserializer: RpcDeserializer>(self) -> DynFunction<Context, RequestState> {
                Box::new(move |ctx, mut req| {
                    $(let $t_idx = $t::inject::<Deserializer>(ctx, &mut req);)*
                    let function = self.clone();
                    Box::pin(async move {
                        match function($($t_idx?,)*).await {
                            Ok(v) => Serializer::serialize(Ok::<R, String>(v)),
                            Err(e) => {
                                let e = e.to_string();
                                eprintln!("ERROR: {e}");
                                Serializer::serialize(Err::<R, String>(e))
                            },
                        }
                    })
                })
            }
            fn get_type_description(names: Self::NameTuple) -> FunctionDescription {
                #[allow(non_snake_case)]
                let ($($t,)*) = names;
                FunctionDescription {
                    args_types: vec![$(($t::EXPORT_DEFINITION, $t.into(), $t::get_type_description()),)*],
                    return_type: R::get_type_description(),
                }
            }
        }
    };
}

for_all_functions!(dyn_fn_impl);
