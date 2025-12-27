//use dirpc_proc_macro::dyn_fn_impl;

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
    type IntoStringType: Into<String>;
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
    type IntoStringType = &'static str;
    type NameTuple = ();
    fn into_dyn_fn<Serializer: RpcSerializer, Deserializer: RpcDeserializer>(
        self,
    ) -> DynFunction<Context, RequestState> {
        Box::new(move |_ctx, _req| {
            let function = self.clone();
            Box::pin(async move {
                match function().await {
                    Ok(v) => Serializer::serialize_ok::<R>(v),
                    Err(e) => {
                        let e = e.to_string();
                        eprintln!("ERROR: {e}");
                        Serializer::serialize_error::<R>(e)
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

//dyn_fn_impl!(16);
// Recursive expansion of dyn_fn_impl! macro
// ==========================================

impl <Context,RequestState,T0,Fut,R,E,F,StrType>IntoDynFunction<Context,RequestState,((T0,R,E),StrType)>for F where T0:inject::Inject<Context,RequestState> +Send+'static,Fut:std::future::Future<Output = Result<R,E>> +Send+'static,R:serde::Serialize+GetTypeDescription,E:ToString,F:FnOnce(T0,) -> Fut+Clone+Send+Sync+'static,StrType:Into<String> ,((),T0::TExportDefinition):GetTupleForExport<StrType> ,{
    type IntoStringType = StrType;
    type NameTuple =  <((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result;
    fn into_dyn_fn<Serializer:RpcSerializer,Deserializer:RpcDeserializer>(self) -> DynFunction<Context,RequestState>{
        Box::new(move|ctx,mut req|{
            let t0 = T0::inject:: <Deserializer>(ctx, &mut req);
            let function = self.clone();
            Box::pin(async move {
                match function(t0? ,).await {
                    Ok(v) => Serializer::serialize_ok:: <R>(v),
                    Err(e) => {
                        let e = e.to_string();
                        {
                            eprintln!("ERROR: {e}");
                        };
                        Serializer::serialize_error:: <R>(e)
                    },
                
                    }
            })
        })
    }
    fn get_type_description(names:Self::NameTuple) -> FunctionDescription {
        FunctionDescription {
            args_types: <Self::NameTuple as ToArgsDescription> ::to_args_description(names,[(<<T0 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T0 as inject::Inject<_,_>> ::get_type_description()),]),return_type:R::get_type_description(),
        }
    }

    }
impl <Context,RequestState,T0,T1,Fut,R,E,F,StrType>IntoDynFunction<Context,RequestState,((T0,T1,R,E),StrType)>for F where T0:inject::Inject<Context,RequestState> +Send+'static,T1:inject::Inject<Context,RequestState> +Send+'static,Fut:std::future::Future<Output = Result<R,E>> +Send+'static,R:serde::Serialize+GetTypeDescription,E:ToString,F:FnOnce(T0,T1,) -> Fut+Clone+Send+Sync+'static,StrType:Into<String> ,((),T0::TExportDefinition):GetTupleForExport<StrType> ,(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition):GetTupleForExport<StrType> ,{
    type IntoStringType = StrType;
    type NameTuple =  <(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result;
    fn into_dyn_fn<Serializer:RpcSerializer,Deserializer:RpcDeserializer>(self) -> DynFunction<Context,RequestState>{
        Box::new(move|ctx,mut req|{
            let t0 = T0::inject:: <Deserializer>(ctx, &mut req);
            let t1 = T1::inject:: <Deserializer>(ctx, &mut req);
            let function = self.clone();
            Box::pin(async move {
                match function(t0? ,t1? ,).await {
                    Ok(v) => Serializer::serialize_ok:: <R>(v),
                    Err(e) => {
                        let e = e.to_string();
                        {
                            eprintln!("ERROR: {e}");
                        };
                        Serializer::serialize_error:: <R>(e)
                    },
                
                    }
            })
        })
    }
    fn get_type_description(names:Self::NameTuple) -> FunctionDescription {
        FunctionDescription {
            args_types: <Self::NameTuple as ToArgsDescription> ::to_args_description(names,[(<<T0 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T0 as inject::Inject<_,_>> ::get_type_description()),(<<T1 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T1 as inject::Inject<_,_>> ::get_type_description()),]),return_type:R::get_type_description(),
        }
    }

    }
impl <Context,RequestState,T0,T1,T2,Fut,R,E,F,StrType>IntoDynFunction<Context,RequestState,((T0,T1,T2,R,E),StrType)>for F where T0:inject::Inject<Context,RequestState> +Send+'static,T1:inject::Inject<Context,RequestState> +Send+'static,T2:inject::Inject<Context,RequestState> +Send+'static,Fut:std::future::Future<Output = Result<R,E>> +Send+'static,R:serde::Serialize+GetTypeDescription,E:ToString,F:FnOnce(T0,T1,T2,) -> Fut+Clone+Send+Sync+'static,StrType:Into<String> ,((),T0::TExportDefinition):GetTupleForExport<StrType> ,(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition):GetTupleForExport<StrType> ,(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition):GetTupleForExport<StrType> ,{
    type IntoStringType = StrType;
    type NameTuple =  <(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result;
    fn into_dyn_fn<Serializer:RpcSerializer,Deserializer:RpcDeserializer>(self) -> DynFunction<Context,RequestState>{
        Box::new(move|ctx,mut req|{
            let t0 = T0::inject:: <Deserializer>(ctx, &mut req);
            let t1 = T1::inject:: <Deserializer>(ctx, &mut req);
            let t2 = T2::inject:: <Deserializer>(ctx, &mut req);
            let function = self.clone();
            Box::pin(async move {
                match function(t0? ,t1? ,t2? ,).await {
                    Ok(v) => Serializer::serialize_ok:: <R>(v),
                    Err(e) => {
                        let e = e.to_string();
                        {
                            eprintln!("ERROR: {e}");
                        };
                        Serializer::serialize_error:: <R>(e)
                    },
                }
            })
        })
    }
    fn get_type_description(names:Self::NameTuple) -> FunctionDescription {
        FunctionDescription {
            args_types: <Self::NameTuple as ToArgsDescription> ::to_args_description(names,[(<<T0 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T0 as inject::Inject<_,_>> ::get_type_description()),(<<T1 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T1 as inject::Inject<_,_>> ::get_type_description()),(<<T2 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T2 as inject::Inject<_,_>> ::get_type_description()),]),return_type:R::get_type_description(),
        }
    }

    }
impl <Context,RequestState,T0,T1,T2,T3,Fut,R,E,F,StrType>IntoDynFunction<Context,RequestState,((T0,T1,T2,T3,R,E),StrType)>for F where T0:inject::Inject<Context,RequestState> +Send+'static,T1:inject::Inject<Context,RequestState> +Send+'static,T2:inject::Inject<Context,RequestState> +Send+'static,T3:inject::Inject<Context,RequestState> +Send+'static,Fut:std::future::Future<Output = Result<R,E>> +Send+'static,R:serde::Serialize+GetTypeDescription,E:ToString,F:FnOnce(T0,T1,T2,T3,) -> Fut+Clone+Send+Sync+'static,StrType:Into<String> ,((),T0::TExportDefinition):GetTupleForExport<StrType> ,(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition):GetTupleForExport<StrType> ,(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition):GetTupleForExport<StrType> ,{
    type IntoStringType = StrType;
    type NameTuple =  <(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result;
    fn into_dyn_fn<Serializer:RpcSerializer,Deserializer:RpcDeserializer>(self) -> DynFunction<Context,RequestState>{
        Box::new(move|ctx,mut req|{
            let t0 = T0::inject:: <Deserializer>(ctx, &mut req);
            let t1 = T1::inject:: <Deserializer>(ctx, &mut req);
            let t2 = T2::inject:: <Deserializer>(ctx, &mut req);
            let t3 = T3::inject:: <Deserializer>(ctx, &mut req);
            let function = self.clone();
            Box::pin(async move {
                match function(t0? ,t1? ,t2? ,t3? ,).await {
                    Ok(v) => Serializer::serialize_ok:: <R>(v),
                    Err(e) => {
                        let e = e.to_string();
                        {
                            eprintln!("ERROR: {e}");
                        };
                        Serializer::serialize_error:: <R>(e)
                    },
                
                    }
            })
        })
    }
    fn get_type_description(names:Self::NameTuple) -> FunctionDescription {
        FunctionDescription {
            args_types: <Self::NameTuple as ToArgsDescription> ::to_args_description(names,[(<<T0 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T0 as inject::Inject<_,_>> ::get_type_description()),(<<T1 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T1 as inject::Inject<_,_>> ::get_type_description()),(<<T2 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T2 as inject::Inject<_,_>> ::get_type_description()),(<<T3 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T3 as inject::Inject<_,_>> ::get_type_description()),]),return_type:R::get_type_description(),
        }
    }

    }
impl <Context,RequestState,T0,T1,T2,T3,T4,Fut,R,E,F,StrType>IntoDynFunction<Context,RequestState,((T0,T1,T2,T3,T4,R,E),StrType)>for F where T0:inject::Inject<Context,RequestState> +Send+'static,T1:inject::Inject<Context,RequestState> +Send+'static,T2:inject::Inject<Context,RequestState> +Send+'static,T3:inject::Inject<Context,RequestState> +Send+'static,T4:inject::Inject<Context,RequestState> +Send+'static,Fut:std::future::Future<Output = Result<R,E>> +Send+'static,R:serde::Serialize+GetTypeDescription,E:ToString,F:FnOnce(T0,T1,T2,T3,T4,) -> Fut+Clone+Send+Sync+'static,StrType:Into<String> ,((),T0::TExportDefinition):GetTupleForExport<StrType> ,(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition):GetTupleForExport<StrType> ,(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition):GetTupleForExport<StrType> ,{
    type IntoStringType = StrType;
    type NameTuple =  <(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result;
    fn into_dyn_fn<Serializer:RpcSerializer,Deserializer:RpcDeserializer>(self) -> DynFunction<Context,RequestState>{
        Box::new(move|ctx,mut req|{
            let t0 = T0::inject:: <Deserializer>(ctx, &mut req);
            let t1 = T1::inject:: <Deserializer>(ctx, &mut req);
            let t2 = T2::inject:: <Deserializer>(ctx, &mut req);
            let t3 = T3::inject:: <Deserializer>(ctx, &mut req);
            let t4 = T4::inject:: <Deserializer>(ctx, &mut req);
            let function = self.clone();
            Box::pin(async move {
                match function(t0? ,t1? ,t2? ,t3? ,t4? ,).await {
                    Ok(v) => Serializer::serialize_ok:: <R>(v),
                    Err(e) => {
                        let e = e.to_string();
                        {
                            eprintln!("ERROR: {e}");
                        };
                        Serializer::serialize_error:: <R>(e)
                    },
                
                    }
            })
        })
    }
    fn get_type_description(names:Self::NameTuple) -> FunctionDescription {
        FunctionDescription {
            args_types: <Self::NameTuple as ToArgsDescription> ::to_args_description(names,[(<<T0 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T0 as inject::Inject<_,_>> ::get_type_description()),(<<T1 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T1 as inject::Inject<_,_>> ::get_type_description()),(<<T2 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T2 as inject::Inject<_,_>> ::get_type_description()),(<<T3 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T3 as inject::Inject<_,_>> ::get_type_description()),(<<T4 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T4 as inject::Inject<_,_>> ::get_type_description()),]),return_type:R::get_type_description(),
        }
    }

    }
impl <Context,RequestState,T0,T1,T2,T3,T4,T5,Fut,R,E,F,StrType>IntoDynFunction<Context,RequestState,((T0,T1,T2,T3,T4,T5,R,E),StrType)>for F where T0:inject::Inject<Context,RequestState> +Send+'static,T1:inject::Inject<Context,RequestState> +Send+'static,T2:inject::Inject<Context,RequestState> +Send+'static,T3:inject::Inject<Context,RequestState> +Send+'static,T4:inject::Inject<Context,RequestState> +Send+'static,T5:inject::Inject<Context,RequestState> +Send+'static,Fut:std::future::Future<Output = Result<R,E>> +Send+'static,R:serde::Serialize+GetTypeDescription,E:ToString,F:FnOnce(T0,T1,T2,T3,T4,T5,) -> Fut+Clone+Send+Sync+'static,StrType:Into<String> ,((),T0::TExportDefinition):GetTupleForExport<StrType> ,(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition):GetTupleForExport<StrType> ,(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition):GetTupleForExport<StrType> ,{
    type IntoStringType = StrType;
    type NameTuple =  <(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result;
    fn into_dyn_fn<Serializer:RpcSerializer,Deserializer:RpcDeserializer>(self) -> DynFunction<Context,RequestState>{
        Box::new(move|ctx,mut req|{
            let t0 = T0::inject:: <Deserializer>(ctx, &mut req);
            let t1 = T1::inject:: <Deserializer>(ctx, &mut req);
            let t2 = T2::inject:: <Deserializer>(ctx, &mut req);
            let t3 = T3::inject:: <Deserializer>(ctx, &mut req);
            let t4 = T4::inject:: <Deserializer>(ctx, &mut req);
            let t5 = T5::inject:: <Deserializer>(ctx, &mut req);
            let function = self.clone();
            Box::pin(async move {
                match function(t0? ,t1? ,t2? ,t3? ,t4? ,t5? ,).await {
                    Ok(v) => Serializer::serialize_ok:: <R>(v),
                    Err(e) => {
                        let e = e.to_string();
                        {
                            eprintln!("ERROR: {e}");
                        };
                        Serializer::serialize_error:: <R>(e)
                    },
                
                    }
            })
        })
    }
    fn get_type_description(names:Self::NameTuple) -> FunctionDescription {
        FunctionDescription {
            args_types: <Self::NameTuple as ToArgsDescription> ::to_args_description(names,[(<<T0 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T0 as inject::Inject<_,_>> ::get_type_description()),(<<T1 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T1 as inject::Inject<_,_>> ::get_type_description()),(<<T2 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T2 as inject::Inject<_,_>> ::get_type_description()),(<<T3 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T3 as inject::Inject<_,_>> ::get_type_description()),(<<T4 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T4 as inject::Inject<_,_>> ::get_type_description()),(<<T5 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T5 as inject::Inject<_,_>> ::get_type_description()),]),return_type:R::get_type_description(),
        }
    }

    }
impl <Context,RequestState,T0,T1,T2,T3,T4,T5,T6,Fut,R,E,F,StrType>IntoDynFunction<Context,RequestState,((T0,T1,T2,T3,T4,T5,T6,R,E),StrType)>for F where T0:inject::Inject<Context,RequestState> +Send+'static,T1:inject::Inject<Context,RequestState> +Send+'static,T2:inject::Inject<Context,RequestState> +Send+'static,T3:inject::Inject<Context,RequestState> +Send+'static,T4:inject::Inject<Context,RequestState> +Send+'static,T5:inject::Inject<Context,RequestState> +Send+'static,T6:inject::Inject<Context,RequestState> +Send+'static,Fut:std::future::Future<Output = Result<R,E>> +Send+'static,R:serde::Serialize+GetTypeDescription,E:ToString,F:FnOnce(T0,T1,T2,T3,T4,T5,T6,) -> Fut+Clone+Send+Sync+'static,StrType:Into<String> ,((),T0::TExportDefinition):GetTupleForExport<StrType> ,(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition):GetTupleForExport<StrType> ,(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition):GetTupleForExport<StrType> ,{
    type IntoStringType = StrType;
    type NameTuple =  <(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result;
    fn into_dyn_fn<Serializer:RpcSerializer,Deserializer:RpcDeserializer>(self) -> DynFunction<Context,RequestState>{
        Box::new(move|ctx,mut req|{
            let t0 = T0::inject:: <Deserializer>(ctx, &mut req);
            let t1 = T1::inject:: <Deserializer>(ctx, &mut req);
            let t2 = T2::inject:: <Deserializer>(ctx, &mut req);
            let t3 = T3::inject:: <Deserializer>(ctx, &mut req);
            let t4 = T4::inject:: <Deserializer>(ctx, &mut req);
            let t5 = T5::inject:: <Deserializer>(ctx, &mut req);
            let t6 = T6::inject:: <Deserializer>(ctx, &mut req);
            let function = self.clone();
            Box::pin(async move {
                match function(t0? ,t1? ,t2? ,t3? ,t4? ,t5? ,t6? ,).await {
                    Ok(v) => Serializer::serialize_ok:: <R>(v),
                    Err(e) => {
                        let e = e.to_string();
                        {
                            eprintln!("ERROR: {e}");
                        };
                        Serializer::serialize_error:: <R>(e)
                    },
                
                    }
            })
        })
    }
    fn get_type_description(names:Self::NameTuple) -> FunctionDescription {
        FunctionDescription {
            args_types: <Self::NameTuple as ToArgsDescription> ::to_args_description(names,[(<<T0 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T0 as inject::Inject<_,_>> ::get_type_description()),(<<T1 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T1 as inject::Inject<_,_>> ::get_type_description()),(<<T2 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T2 as inject::Inject<_,_>> ::get_type_description()),(<<T3 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T3 as inject::Inject<_,_>> ::get_type_description()),(<<T4 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T4 as inject::Inject<_,_>> ::get_type_description()),(<<T5 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T5 as inject::Inject<_,_>> ::get_type_description()),(<<T6 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T6 as inject::Inject<_,_>> ::get_type_description()),]),return_type:R::get_type_description(),
        }
    }

    }
impl <Context,RequestState,T0,T1,T2,T3,T4,T5,T6,T7,Fut,R,E,F,StrType>IntoDynFunction<Context,RequestState,((T0,T1,T2,T3,T4,T5,T6,T7,R,E),StrType)>for F where T0:inject::Inject<Context,RequestState> +Send+'static,T1:inject::Inject<Context,RequestState> +Send+'static,T2:inject::Inject<Context,RequestState> +Send+'static,T3:inject::Inject<Context,RequestState> +Send+'static,T4:inject::Inject<Context,RequestState> +Send+'static,T5:inject::Inject<Context,RequestState> +Send+'static,T6:inject::Inject<Context,RequestState> +Send+'static,T7:inject::Inject<Context,RequestState> +Send+'static,Fut:std::future::Future<Output = Result<R,E>> +Send+'static,R:serde::Serialize+GetTypeDescription,E:ToString,F:FnOnce(T0,T1,T2,T3,T4,T5,T6,T7,) -> Fut+Clone+Send+Sync+'static,StrType:Into<String> ,((),T0::TExportDefinition):GetTupleForExport<StrType> ,(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition):GetTupleForExport<StrType> ,(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition):GetTupleForExport<StrType> ,{
    type IntoStringType = StrType;
    type NameTuple =  <(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result;
    fn into_dyn_fn<Serializer:RpcSerializer,Deserializer:RpcDeserializer>(self) -> DynFunction<Context,RequestState>{
        Box::new(move|ctx,mut req|{
            let t0 = T0::inject:: <Deserializer>(ctx, &mut req);
            let t1 = T1::inject:: <Deserializer>(ctx, &mut req);
            let t2 = T2::inject:: <Deserializer>(ctx, &mut req);
            let t3 = T3::inject:: <Deserializer>(ctx, &mut req);
            let t4 = T4::inject:: <Deserializer>(ctx, &mut req);
            let t5 = T5::inject:: <Deserializer>(ctx, &mut req);
            let t6 = T6::inject:: <Deserializer>(ctx, &mut req);
            let t7 = T7::inject:: <Deserializer>(ctx, &mut req);
            let function = self.clone();
            Box::pin(async move {
                match function(t0? ,t1? ,t2? ,t3? ,t4? ,t5? ,t6? ,t7? ,).await {
                    Ok(v) => Serializer::serialize_ok:: <R>(v),
                    Err(e) => {
                        let e = e.to_string();
                        {
                            eprintln!("ERROR: {e}");
                        };
                        Serializer::serialize_error:: <R>(e)
                    },
                
                    }
            })
        })
    }
    fn get_type_description(names:Self::NameTuple) -> FunctionDescription {
        FunctionDescription {
            args_types: <Self::NameTuple as ToArgsDescription> ::to_args_description(names,[(<<T0 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T0 as inject::Inject<_,_>> ::get_type_description()),(<<T1 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T1 as inject::Inject<_,_>> ::get_type_description()),(<<T2 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T2 as inject::Inject<_,_>> ::get_type_description()),(<<T3 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T3 as inject::Inject<_,_>> ::get_type_description()),(<<T4 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T4 as inject::Inject<_,_>> ::get_type_description()),(<<T5 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T5 as inject::Inject<_,_>> ::get_type_description()),(<<T6 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T6 as inject::Inject<_,_>> ::get_type_description()),(<<T7 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T7 as inject::Inject<_,_>> ::get_type_description()),]),return_type:R::get_type_description(),
        }
    }

    }
impl <Context,RequestState,T0,T1,T2,T3,T4,T5,T6,T7,T8,Fut,R,E,F,StrType>IntoDynFunction<Context,RequestState,((T0,T1,T2,T3,T4,T5,T6,T7,T8,R,E),StrType)>for F where T0:inject::Inject<Context,RequestState> +Send+'static,T1:inject::Inject<Context,RequestState> +Send+'static,T2:inject::Inject<Context,RequestState> +Send+'static,T3:inject::Inject<Context,RequestState> +Send+'static,T4:inject::Inject<Context,RequestState> +Send+'static,T5:inject::Inject<Context,RequestState> +Send+'static,T6:inject::Inject<Context,RequestState> +Send+'static,T7:inject::Inject<Context,RequestState> +Send+'static,T8:inject::Inject<Context,RequestState> +Send+'static,Fut:std::future::Future<Output = Result<R,E>> +Send+'static,R:serde::Serialize+GetTypeDescription,E:ToString,F:FnOnce(T0,T1,T2,T3,T4,T5,T6,T7,T8,) -> Fut+Clone+Send+Sync+'static,StrType:Into<String> ,((),T0::TExportDefinition):GetTupleForExport<StrType> ,(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition):GetTupleForExport<StrType> ,(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition):GetTupleForExport<StrType> ,{
    type IntoStringType = StrType;
    type NameTuple =  <(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result;
    fn into_dyn_fn<Serializer:RpcSerializer,Deserializer:RpcDeserializer>(self) -> DynFunction<Context,RequestState>{
        Box::new(move|ctx,mut req|{
            let t0 = T0::inject:: <Deserializer>(ctx, &mut req);
            let t1 = T1::inject:: <Deserializer>(ctx, &mut req);
            let t2 = T2::inject:: <Deserializer>(ctx, &mut req);
            let t3 = T3::inject:: <Deserializer>(ctx, &mut req);
            let t4 = T4::inject:: <Deserializer>(ctx, &mut req);
            let t5 = T5::inject:: <Deserializer>(ctx, &mut req);
            let t6 = T6::inject:: <Deserializer>(ctx, &mut req);
            let t7 = T7::inject:: <Deserializer>(ctx, &mut req);
            let t8 = T8::inject:: <Deserializer>(ctx, &mut req);
            let function = self.clone();
            Box::pin(async move {
                match function(t0? ,t1? ,t2? ,t3? ,t4? ,t5? ,t6? ,t7? ,t8? ,).await {
                    Ok(v) => Serializer::serialize_ok:: <R>(v),
                    Err(e) => {
                        let e = e.to_string();
                        {
                            eprintln!("ERROR: {e}");
                        };
                        Serializer::serialize_error:: <R>(e)
                    },
                
                    }
            })
        })
    }
    fn get_type_description(names:Self::NameTuple) -> FunctionDescription {
        FunctionDescription {
            args_types: <Self::NameTuple as ToArgsDescription> ::to_args_description(names,[(<<T0 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T0 as inject::Inject<_,_>> ::get_type_description()),(<<T1 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T1 as inject::Inject<_,_>> ::get_type_description()),(<<T2 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T2 as inject::Inject<_,_>> ::get_type_description()),(<<T3 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T3 as inject::Inject<_,_>> ::get_type_description()),(<<T4 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T4 as inject::Inject<_,_>> ::get_type_description()),(<<T5 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T5 as inject::Inject<_,_>> ::get_type_description()),(<<T6 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T6 as inject::Inject<_,_>> ::get_type_description()),(<<T7 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T7 as inject::Inject<_,_>> ::get_type_description()),(<<T8 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T8 as inject::Inject<_,_>> ::get_type_description()),]),return_type:R::get_type_description(),
        }
    }

    }
impl <Context,RequestState,T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,Fut,R,E,F,StrType>IntoDynFunction<Context,RequestState,((T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,R,E),StrType)>for F where T0:inject::Inject<Context,RequestState> +Send+'static,T1:inject::Inject<Context,RequestState> +Send+'static,T2:inject::Inject<Context,RequestState> +Send+'static,T3:inject::Inject<Context,RequestState> +Send+'static,T4:inject::Inject<Context,RequestState> +Send+'static,T5:inject::Inject<Context,RequestState> +Send+'static,T6:inject::Inject<Context,RequestState> +Send+'static,T7:inject::Inject<Context,RequestState> +Send+'static,T8:inject::Inject<Context,RequestState> +Send+'static,T9:inject::Inject<Context,RequestState> +Send+'static,Fut:std::future::Future<Output = Result<R,E>> +Send+'static,R:serde::Serialize+GetTypeDescription,E:ToString,F:FnOnce(T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,) -> Fut+Clone+Send+Sync+'static,StrType:Into<String> ,((),T0::TExportDefinition):GetTupleForExport<StrType> ,(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition):GetTupleForExport<StrType> ,(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition):GetTupleForExport<StrType> ,{
    type IntoStringType = StrType;
    type NameTuple =  <(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result;
    fn into_dyn_fn<Serializer:RpcSerializer,Deserializer:RpcDeserializer>(self) -> DynFunction<Context,RequestState>{
        Box::new(move|ctx,mut req|{
            let t0 = T0::inject:: <Deserializer>(ctx, &mut req);
            let t1 = T1::inject:: <Deserializer>(ctx, &mut req);
            let t2 = T2::inject:: <Deserializer>(ctx, &mut req);
            let t3 = T3::inject:: <Deserializer>(ctx, &mut req);
            let t4 = T4::inject:: <Deserializer>(ctx, &mut req);
            let t5 = T5::inject:: <Deserializer>(ctx, &mut req);
            let t6 = T6::inject:: <Deserializer>(ctx, &mut req);
            let t7 = T7::inject:: <Deserializer>(ctx, &mut req);
            let t8 = T8::inject:: <Deserializer>(ctx, &mut req);
            let t9 = T9::inject:: <Deserializer>(ctx, &mut req);
            let function = self.clone();
            Box::pin(async move {
                match function(t0? ,t1? ,t2? ,t3? ,t4? ,t5? ,t6? ,t7? ,t8? ,t9? ,).await {
                    Ok(v) => Serializer::serialize_ok:: <R>(v),
                    Err(e) => {
                        let e = e.to_string();
                        {
                            eprintln!("ERROR: {e}");
                        };
                        Serializer::serialize_error:: <R>(e)
                    },
                
                    }
            })
        })
    }
    fn get_type_description(names:Self::NameTuple) -> FunctionDescription {
        FunctionDescription {
            args_types: <Self::NameTuple as ToArgsDescription> ::to_args_description(names,[(<<T0 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T0 as inject::Inject<_,_>> ::get_type_description()),(<<T1 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T1 as inject::Inject<_,_>> ::get_type_description()),(<<T2 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T2 as inject::Inject<_,_>> ::get_type_description()),(<<T3 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T3 as inject::Inject<_,_>> ::get_type_description()),(<<T4 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T4 as inject::Inject<_,_>> ::get_type_description()),(<<T5 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T5 as inject::Inject<_,_>> ::get_type_description()),(<<T6 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T6 as inject::Inject<_,_>> ::get_type_description()),(<<T7 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T7 as inject::Inject<_,_>> ::get_type_description()),(<<T8 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T8 as inject::Inject<_,_>> ::get_type_description()),(<<T9 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T9 as inject::Inject<_,_>> ::get_type_description()),]),return_type:R::get_type_description(),
        }
    }

    }
impl <Context,RequestState,T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,Fut,R,E,F,StrType>IntoDynFunction<Context,RequestState,((T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,R,E),StrType)>for F where T0:inject::Inject<Context,RequestState> +Send+'static,T1:inject::Inject<Context,RequestState> +Send+'static,T2:inject::Inject<Context,RequestState> +Send+'static,T3:inject::Inject<Context,RequestState> +Send+'static,T4:inject::Inject<Context,RequestState> +Send+'static,T5:inject::Inject<Context,RequestState> +Send+'static,T6:inject::Inject<Context,RequestState> +Send+'static,T7:inject::Inject<Context,RequestState> +Send+'static,T8:inject::Inject<Context,RequestState> +Send+'static,T9:inject::Inject<Context,RequestState> +Send+'static,T10:inject::Inject<Context,RequestState> +Send+'static,Fut:std::future::Future<Output = Result<R,E>> +Send+'static,R:serde::Serialize+GetTypeDescription,E:ToString,F:FnOnce(T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,) -> Fut+Clone+Send+Sync+'static,StrType:Into<String> ,((),T0::TExportDefinition):GetTupleForExport<StrType> ,(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition):GetTupleForExport<StrType> ,(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition):GetTupleForExport<StrType> ,{
    type IntoStringType = StrType;
    type NameTuple =  <(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result;
    fn into_dyn_fn<Serializer:RpcSerializer,Deserializer:RpcDeserializer>(self) -> DynFunction<Context,RequestState>{
        Box::new(move|ctx,mut req|{
            let t0 = T0::inject:: <Deserializer>(ctx, &mut req);
            let t1 = T1::inject:: <Deserializer>(ctx, &mut req);
            let t2 = T2::inject:: <Deserializer>(ctx, &mut req);
            let t3 = T3::inject:: <Deserializer>(ctx, &mut req);
            let t4 = T4::inject:: <Deserializer>(ctx, &mut req);
            let t5 = T5::inject:: <Deserializer>(ctx, &mut req);
            let t6 = T6::inject:: <Deserializer>(ctx, &mut req);
            let t7 = T7::inject:: <Deserializer>(ctx, &mut req);
            let t8 = T8::inject:: <Deserializer>(ctx, &mut req);
            let t9 = T9::inject:: <Deserializer>(ctx, &mut req);
            let t10 = T10::inject:: <Deserializer>(ctx, &mut req);
            let function = self.clone();
            Box::pin(async move {
                match function(t0? ,t1? ,t2? ,t3? ,t4? ,t5? ,t6? ,t7? ,t8? ,t9? ,t10? ,).await {
                    Ok(v) => Serializer::serialize_ok:: <R>(v),
                    Err(e) => {
                        let e = e.to_string();
                        {
                            eprintln!("ERROR: {e}");
                        };
                        Serializer::serialize_error:: <R>(e)
                    },
                
                    }
            })
        })
    }
    fn get_type_description(names:Self::NameTuple) -> FunctionDescription {
        FunctionDescription {
            args_types: <Self::NameTuple as ToArgsDescription> ::to_args_description(names,[(<<T0 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T0 as inject::Inject<_,_>> ::get_type_description()),(<<T1 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T1 as inject::Inject<_,_>> ::get_type_description()),(<<T2 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T2 as inject::Inject<_,_>> ::get_type_description()),(<<T3 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T3 as inject::Inject<_,_>> ::get_type_description()),(<<T4 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T4 as inject::Inject<_,_>> ::get_type_description()),(<<T5 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T5 as inject::Inject<_,_>> ::get_type_description()),(<<T6 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T6 as inject::Inject<_,_>> ::get_type_description()),(<<T7 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T7 as inject::Inject<_,_>> ::get_type_description()),(<<T8 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T8 as inject::Inject<_,_>> ::get_type_description()),(<<T9 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T9 as inject::Inject<_,_>> ::get_type_description()),(<<T10 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T10 as inject::Inject<_,_>> ::get_type_description()),]),return_type:R::get_type_description(),
        }
    }

    }
impl <Context,RequestState,T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,Fut,R,E,F,StrType>IntoDynFunction<Context,RequestState,((T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,R,E),StrType)>for F where T0:inject::Inject<Context,RequestState> +Send+'static,T1:inject::Inject<Context,RequestState> +Send+'static,T2:inject::Inject<Context,RequestState> +Send+'static,T3:inject::Inject<Context,RequestState> +Send+'static,T4:inject::Inject<Context,RequestState> +Send+'static,T5:inject::Inject<Context,RequestState> +Send+'static,T6:inject::Inject<Context,RequestState> +Send+'static,T7:inject::Inject<Context,RequestState> +Send+'static,T8:inject::Inject<Context,RequestState> +Send+'static,T9:inject::Inject<Context,RequestState> +Send+'static,T10:inject::Inject<Context,RequestState> +Send+'static,T11:inject::Inject<Context,RequestState> +Send+'static,Fut:std::future::Future<Output = Result<R,E>> +Send+'static,R:serde::Serialize+GetTypeDescription,E:ToString,F:FnOnce(T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,) -> Fut+Clone+Send+Sync+'static,StrType:Into<String> ,((),T0::TExportDefinition):GetTupleForExport<StrType> ,(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition):GetTupleForExport<StrType> ,(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition):GetTupleForExport<StrType> ,{
    type IntoStringType = StrType;
    type NameTuple =  <(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition)as GetTupleForExport<StrType>> ::Result;
    fn into_dyn_fn<Serializer:RpcSerializer,Deserializer:RpcDeserializer>(self) -> DynFunction<Context,RequestState>{
        Box::new(move|ctx,mut req|{
            let t0 = T0::inject:: <Deserializer>(ctx, &mut req);
            let t1 = T1::inject:: <Deserializer>(ctx, &mut req);
            let t2 = T2::inject:: <Deserializer>(ctx, &mut req);
            let t3 = T3::inject:: <Deserializer>(ctx, &mut req);
            let t4 = T4::inject:: <Deserializer>(ctx, &mut req);
            let t5 = T5::inject:: <Deserializer>(ctx, &mut req);
            let t6 = T6::inject:: <Deserializer>(ctx, &mut req);
            let t7 = T7::inject:: <Deserializer>(ctx, &mut req);
            let t8 = T8::inject:: <Deserializer>(ctx, &mut req);
            let t9 = T9::inject:: <Deserializer>(ctx, &mut req);
            let t10 = T10::inject:: <Deserializer>(ctx, &mut req);
            let t11 = T11::inject:: <Deserializer>(ctx, &mut req);
            let function = self.clone();
            Box::pin(async move {
                match function(t0? ,t1? ,t2? ,t3? ,t4? ,t5? ,t6? ,t7? ,t8? ,t9? ,t10? ,t11? ,).await {
                    Ok(v) => Serializer::serialize_ok:: <R>(v),
                    Err(e) => {
                        let e = e.to_string();
                        {
                            eprintln!("ERROR: {e}");
                        };
                        Serializer::serialize_error:: <R>(e)
                    },
                
                    }
            })
        })
    }
    fn get_type_description(names:Self::NameTuple) -> FunctionDescription {
        FunctionDescription {
            args_types: <Self::NameTuple as ToArgsDescription> ::to_args_description(names,[(<<T0 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T0 as inject::Inject<_,_>> ::get_type_description()),(<<T1 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T1 as inject::Inject<_,_>> ::get_type_description()),(<<T2 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T2 as inject::Inject<_,_>> ::get_type_description()),(<<T3 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T3 as inject::Inject<_,_>> ::get_type_description()),(<<T4 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T4 as inject::Inject<_,_>> ::get_type_description()),(<<T5 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T5 as inject::Inject<_,_>> ::get_type_description()),(<<T6 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T6 as inject::Inject<_,_>> ::get_type_description()),(<<T7 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T7 as inject::Inject<_,_>> ::get_type_description()),(<<T8 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T8 as inject::Inject<_,_>> ::get_type_description()),(<<T9 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T9 as inject::Inject<_,_>> ::get_type_description()),(<<T10 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T10 as inject::Inject<_,_>> ::get_type_description()),(<<T11 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T11 as inject::Inject<_,_>> ::get_type_description()),]),return_type:R::get_type_description(),
        }
    }

    }
impl <Context,RequestState,T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,Fut,R,E,F,StrType>IntoDynFunction<Context,RequestState,((T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,R,E),StrType)>for F where T0:inject::Inject<Context,RequestState> +Send+'static,T1:inject::Inject<Context,RequestState> +Send+'static,T2:inject::Inject<Context,RequestState> +Send+'static,T3:inject::Inject<Context,RequestState> +Send+'static,T4:inject::Inject<Context,RequestState> +Send+'static,T5:inject::Inject<Context,RequestState> +Send+'static,T6:inject::Inject<Context,RequestState> +Send+'static,T7:inject::Inject<Context,RequestState> +Send+'static,T8:inject::Inject<Context,RequestState> +Send+'static,T9:inject::Inject<Context,RequestState> +Send+'static,T10:inject::Inject<Context,RequestState> +Send+'static,T11:inject::Inject<Context,RequestState> +Send+'static,T12:inject::Inject<Context,RequestState> +Send+'static,Fut:std::future::Future<Output = Result<R,E>> +Send+'static,R:serde::Serialize+GetTypeDescription,E:ToString,F:FnOnce(T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,) -> Fut+Clone+Send+Sync+'static,StrType:Into<String> ,((),T0::TExportDefinition):GetTupleForExport<StrType> ,(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition):GetTupleForExport<StrType> ,(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T12::TExportDefinition):GetTupleForExport<StrType> ,{
    type IntoStringType = StrType;
    type NameTuple =  <(<(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T12::TExportDefinition)as GetTupleForExport<StrType>> ::Result;
    fn into_dyn_fn<Serializer:RpcSerializer,Deserializer:RpcDeserializer>(self) -> DynFunction<Context,RequestState>{
        Box::new(move|ctx,mut req|{
            let t0 = T0::inject:: <Deserializer>(ctx, &mut req);
            let t1 = T1::inject:: <Deserializer>(ctx, &mut req);
            let t2 = T2::inject:: <Deserializer>(ctx, &mut req);
            let t3 = T3::inject:: <Deserializer>(ctx, &mut req);
            let t4 = T4::inject:: <Deserializer>(ctx, &mut req);
            let t5 = T5::inject:: <Deserializer>(ctx, &mut req);
            let t6 = T6::inject:: <Deserializer>(ctx, &mut req);
            let t7 = T7::inject:: <Deserializer>(ctx, &mut req);
            let t8 = T8::inject:: <Deserializer>(ctx, &mut req);
            let t9 = T9::inject:: <Deserializer>(ctx, &mut req);
            let t10 = T10::inject:: <Deserializer>(ctx, &mut req);
            let t11 = T11::inject:: <Deserializer>(ctx, &mut req);
            let t12 = T12::inject:: <Deserializer>(ctx, &mut req);
            let function = self.clone();
            Box::pin(async move {
                match function(t0? ,t1? ,t2? ,t3? ,t4? ,t5? ,t6? ,t7? ,t8? ,t9? ,t10? ,t11? ,t12? ,).await {
                    Ok(v) => Serializer::serialize_ok:: <R>(v),
                    Err(e) => {
                        let e = e.to_string();
                        {
                            eprintln!("ERROR: {e}");
                        };
                        Serializer::serialize_error:: <R>(e)
                    },
                
                    }
            })
        })
    }
    fn get_type_description(names:Self::NameTuple) -> FunctionDescription {
        FunctionDescription {
            args_types: <Self::NameTuple as ToArgsDescription> ::to_args_description(names,[(<<T0 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T0 as inject::Inject<_,_>> ::get_type_description()),(<<T1 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T1 as inject::Inject<_,_>> ::get_type_description()),(<<T2 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T2 as inject::Inject<_,_>> ::get_type_description()),(<<T3 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T3 as inject::Inject<_,_>> ::get_type_description()),(<<T4 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T4 as inject::Inject<_,_>> ::get_type_description()),(<<T5 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T5 as inject::Inject<_,_>> ::get_type_description()),(<<T6 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T6 as inject::Inject<_,_>> ::get_type_description()),(<<T7 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T7 as inject::Inject<_,_>> ::get_type_description()),(<<T8 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T8 as inject::Inject<_,_>> ::get_type_description()),(<<T9 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T9 as inject::Inject<_,_>> ::get_type_description()),(<<T10 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T10 as inject::Inject<_,_>> ::get_type_description()),(<<T11 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T11 as inject::Inject<_,_>> ::get_type_description()),(<<T12 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T12 as inject::Inject<_,_>> ::get_type_description()),]),return_type:R::get_type_description(),
        }
    }

    }
impl <Context,RequestState,T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,Fut,R,E,F,StrType>IntoDynFunction<Context,RequestState,((T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,R,E),StrType)>for F where T0:inject::Inject<Context,RequestState> +Send+'static,T1:inject::Inject<Context,RequestState> +Send+'static,T2:inject::Inject<Context,RequestState> +Send+'static,T3:inject::Inject<Context,RequestState> +Send+'static,T4:inject::Inject<Context,RequestState> +Send+'static,T5:inject::Inject<Context,RequestState> +Send+'static,T6:inject::Inject<Context,RequestState> +Send+'static,T7:inject::Inject<Context,RequestState> +Send+'static,T8:inject::Inject<Context,RequestState> +Send+'static,T9:inject::Inject<Context,RequestState> +Send+'static,T10:inject::Inject<Context,RequestState> +Send+'static,T11:inject::Inject<Context,RequestState> +Send+'static,T12:inject::Inject<Context,RequestState> +Send+'static,T13:inject::Inject<Context,RequestState> +Send+'static,Fut:std::future::Future<Output = Result<R,E>> +Send+'static,R:serde::Serialize+GetTypeDescription,E:ToString,F:FnOnce(T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,) -> Fut+Clone+Send+Sync+'static,StrType:Into<String> ,((),T0::TExportDefinition):GetTupleForExport<StrType> ,(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition):GetTupleForExport<StrType> ,(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T12::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T12::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T13::TExportDefinition):GetTupleForExport<StrType> ,{
    type IntoStringType = StrType;
    type NameTuple =  <(<(<(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T12::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T13::TExportDefinition)as GetTupleForExport<StrType>> ::Result;
    fn into_dyn_fn<Serializer:RpcSerializer,Deserializer:RpcDeserializer>(self) -> DynFunction<Context,RequestState>{
        Box::new(move|ctx,mut req|{
            let t0 = T0::inject:: <Deserializer>(ctx, &mut req);
            let t1 = T1::inject:: <Deserializer>(ctx, &mut req);
            let t2 = T2::inject:: <Deserializer>(ctx, &mut req);
            let t3 = T3::inject:: <Deserializer>(ctx, &mut req);
            let t4 = T4::inject:: <Deserializer>(ctx, &mut req);
            let t5 = T5::inject:: <Deserializer>(ctx, &mut req);
            let t6 = T6::inject:: <Deserializer>(ctx, &mut req);
            let t7 = T7::inject:: <Deserializer>(ctx, &mut req);
            let t8 = T8::inject:: <Deserializer>(ctx, &mut req);
            let t9 = T9::inject:: <Deserializer>(ctx, &mut req);
            let t10 = T10::inject:: <Deserializer>(ctx, &mut req);
            let t11 = T11::inject:: <Deserializer>(ctx, &mut req);
            let t12 = T12::inject:: <Deserializer>(ctx, &mut req);
            let t13 = T13::inject:: <Deserializer>(ctx, &mut req);
            let function = self.clone();
            Box::pin(async move {
                match function(t0? ,t1? ,t2? ,t3? ,t4? ,t5? ,t6? ,t7? ,t8? ,t9? ,t10? ,t11? ,t12? ,t13? ,).await {
                    Ok(v) => Serializer::serialize_ok:: <R>(v),
                    Err(e) => {
                        let e = e.to_string();
                        {
                            eprintln!("ERROR: {e}");
                        };
                        Serializer::serialize_error:: <R>(e)
                    },
                
                    }
            })
        })
    }
    fn get_type_description(names:Self::NameTuple) -> FunctionDescription {
        FunctionDescription {
            args_types: <Self::NameTuple as ToArgsDescription> ::to_args_description(names,[(<<T0 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T0 as inject::Inject<_,_>> ::get_type_description()),(<<T1 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T1 as inject::Inject<_,_>> ::get_type_description()),(<<T2 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T2 as inject::Inject<_,_>> ::get_type_description()),(<<T3 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T3 as inject::Inject<_,_>> ::get_type_description()),(<<T4 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T4 as inject::Inject<_,_>> ::get_type_description()),(<<T5 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T5 as inject::Inject<_,_>> ::get_type_description()),(<<T6 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T6 as inject::Inject<_,_>> ::get_type_description()),(<<T7 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T7 as inject::Inject<_,_>> ::get_type_description()),(<<T8 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T8 as inject::Inject<_,_>> ::get_type_description()),(<<T9 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T9 as inject::Inject<_,_>> ::get_type_description()),(<<T10 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T10 as inject::Inject<_,_>> ::get_type_description()),(<<T11 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T11 as inject::Inject<_,_>> ::get_type_description()),(<<T12 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T12 as inject::Inject<_,_>> ::get_type_description()),(<<T13 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T13 as inject::Inject<_,_>> ::get_type_description()),]),return_type:R::get_type_description(),
        }
    }

    }
impl <Context,RequestState,T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,Fut,R,E,F,StrType>IntoDynFunction<Context,RequestState,((T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,R,E),StrType)>for F where T0:inject::Inject<Context,RequestState> +Send+'static,T1:inject::Inject<Context,RequestState> +Send+'static,T2:inject::Inject<Context,RequestState> +Send+'static,T3:inject::Inject<Context,RequestState> +Send+'static,T4:inject::Inject<Context,RequestState> +Send+'static,T5:inject::Inject<Context,RequestState> +Send+'static,T6:inject::Inject<Context,RequestState> +Send+'static,T7:inject::Inject<Context,RequestState> +Send+'static,T8:inject::Inject<Context,RequestState> +Send+'static,T9:inject::Inject<Context,RequestState> +Send+'static,T10:inject::Inject<Context,RequestState> +Send+'static,T11:inject::Inject<Context,RequestState> +Send+'static,T12:inject::Inject<Context,RequestState> +Send+'static,T13:inject::Inject<Context,RequestState> +Send+'static,T14:inject::Inject<Context,RequestState> +Send+'static,Fut:std::future::Future<Output = Result<R,E>> +Send+'static,R:serde::Serialize+GetTypeDescription,E:ToString,F:FnOnce(T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,) -> Fut+Clone+Send+Sync+'static,StrType:Into<String> ,((),T0::TExportDefinition):GetTupleForExport<StrType> ,(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition):GetTupleForExport<StrType> ,(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T12::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T12::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T13::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T12::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T13::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T14::TExportDefinition):GetTupleForExport<StrType> ,{
    type IntoStringType = StrType;
    type NameTuple =  <(<(<(<(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T12::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T13::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T14::TExportDefinition)as GetTupleForExport<StrType>> ::Result;
    fn into_dyn_fn<Serializer:RpcSerializer,Deserializer:RpcDeserializer>(self) -> DynFunction<Context,RequestState>{
        Box::new(move|ctx,mut req|{
            let t0 = T0::inject:: <Deserializer>(ctx, &mut req);
            let t1 = T1::inject:: <Deserializer>(ctx, &mut req);
            let t2 = T2::inject:: <Deserializer>(ctx, &mut req);
            let t3 = T3::inject:: <Deserializer>(ctx, &mut req);
            let t4 = T4::inject:: <Deserializer>(ctx, &mut req);
            let t5 = T5::inject:: <Deserializer>(ctx, &mut req);
            let t6 = T6::inject:: <Deserializer>(ctx, &mut req);
            let t7 = T7::inject:: <Deserializer>(ctx, &mut req);
            let t8 = T8::inject:: <Deserializer>(ctx, &mut req);
            let t9 = T9::inject:: <Deserializer>(ctx, &mut req);
            let t10 = T10::inject:: <Deserializer>(ctx, &mut req);
            let t11 = T11::inject:: <Deserializer>(ctx, &mut req);
            let t12 = T12::inject:: <Deserializer>(ctx, &mut req);
            let t13 = T13::inject:: <Deserializer>(ctx, &mut req);
            let t14 = T14::inject:: <Deserializer>(ctx, &mut req);
            let function = self.clone();
            Box::pin(async move {
                match function(t0? ,t1? ,t2? ,t3? ,t4? ,t5? ,t6? ,t7? ,t8? ,t9? ,t10? ,t11? ,t12? ,t13? ,t14? ,).await {
                    Ok(v) => Serializer::serialize_ok:: <R>(v),
                    Err(e) => {
                        let e = e.to_string();
                        {
                            eprintln!("ERROR: {e}");
                        };
                        Serializer::serialize_error:: <R>(e)
                    },
                
                    }
            })
        })
    }
    fn get_type_description(names:Self::NameTuple) -> FunctionDescription {
        FunctionDescription {
            args_types: <Self::NameTuple as ToArgsDescription> ::to_args_description(names,[(<<T0 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T0 as inject::Inject<_,_>> ::get_type_description()),(<<T1 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T1 as inject::Inject<_,_>> ::get_type_description()),(<<T2 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T2 as inject::Inject<_,_>> ::get_type_description()),(<<T3 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T3 as inject::Inject<_,_>> ::get_type_description()),(<<T4 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T4 as inject::Inject<_,_>> ::get_type_description()),(<<T5 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T5 as inject::Inject<_,_>> ::get_type_description()),(<<T6 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T6 as inject::Inject<_,_>> ::get_type_description()),(<<T7 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T7 as inject::Inject<_,_>> ::get_type_description()),(<<T8 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T8 as inject::Inject<_,_>> ::get_type_description()),(<<T9 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T9 as inject::Inject<_,_>> ::get_type_description()),(<<T10 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T10 as inject::Inject<_,_>> ::get_type_description()),(<<T11 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T11 as inject::Inject<_,_>> ::get_type_description()),(<<T12 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T12 as inject::Inject<_,_>> ::get_type_description()),(<<T13 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T13 as inject::Inject<_,_>> ::get_type_description()),(<<T14 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T14 as inject::Inject<_,_>> ::get_type_description()),]),return_type:R::get_type_description(),
        }
    }

    }
impl <Context,RequestState,T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15,Fut,R,E,F,StrType>IntoDynFunction<Context,RequestState,((T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15,R,E),StrType)>for F where T0:inject::Inject<Context,RequestState> +Send+'static,T1:inject::Inject<Context,RequestState> +Send+'static,T2:inject::Inject<Context,RequestState> +Send+'static,T3:inject::Inject<Context,RequestState> +Send+'static,T4:inject::Inject<Context,RequestState> +Send+'static,T5:inject::Inject<Context,RequestState> +Send+'static,T6:inject::Inject<Context,RequestState> +Send+'static,T7:inject::Inject<Context,RequestState> +Send+'static,T8:inject::Inject<Context,RequestState> +Send+'static,T9:inject::Inject<Context,RequestState> +Send+'static,T10:inject::Inject<Context,RequestState> +Send+'static,T11:inject::Inject<Context,RequestState> +Send+'static,T12:inject::Inject<Context,RequestState> +Send+'static,T13:inject::Inject<Context,RequestState> +Send+'static,T14:inject::Inject<Context,RequestState> +Send+'static,T15:inject::Inject<Context,RequestState> +Send+'static,Fut:std::future::Future<Output = Result<R,E>> +Send+'static,R:serde::Serialize+GetTypeDescription,E:ToString,F:FnOnce(T0,T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,T15,) -> Fut+Clone+Send+Sync+'static,StrType:Into<String> ,((),T0::TExportDefinition):GetTupleForExport<StrType> ,(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition):GetTupleForExport<StrType> ,(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T12::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T12::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T13::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T12::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T13::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T14::TExportDefinition):GetTupleForExport<StrType> ,(<(<(<(<(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T12::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T13::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T14::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T15::TExportDefinition):GetTupleForExport<StrType> ,{
    type IntoStringType = StrType;
    type NameTuple =  <(<(<(<(<(<(<(<(<(<(<(<(<(<(<(<((),T0::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T1::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T2::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T3::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T4::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T5::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T6::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T7::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T8::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T9::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T10::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T11::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T12::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T13::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T14::TExportDefinition)as GetTupleForExport<StrType>> ::Result,T15::TExportDefinition)as GetTupleForExport<StrType>> ::Result;
    fn into_dyn_fn<Serializer:RpcSerializer,Deserializer:RpcDeserializer>(self) -> DynFunction<Context,RequestState>{
        Box::new(move|ctx,mut req|{
            let t0 = T0::inject:: <Deserializer>(ctx, &mut req);
            let t1 = T1::inject:: <Deserializer>(ctx, &mut req);
            let t2 = T2::inject:: <Deserializer>(ctx, &mut req);
            let t3 = T3::inject:: <Deserializer>(ctx, &mut req);
            let t4 = T4::inject:: <Deserializer>(ctx, &mut req);
            let t5 = T5::inject:: <Deserializer>(ctx, &mut req);
            let t6 = T6::inject:: <Deserializer>(ctx, &mut req);
            let t7 = T7::inject:: <Deserializer>(ctx, &mut req);
            let t8 = T8::inject:: <Deserializer>(ctx, &mut req);
            let t9 = T9::inject:: <Deserializer>(ctx, &mut req);
            let t10 = T10::inject:: <Deserializer>(ctx, &mut req);
            let t11 = T11::inject:: <Deserializer>(ctx, &mut req);
            let t12 = T12::inject:: <Deserializer>(ctx, &mut req);
            let t13 = T13::inject:: <Deserializer>(ctx, &mut req);
            let t14 = T14::inject:: <Deserializer>(ctx, &mut req);
            let t15 = T15::inject:: <Deserializer>(ctx, &mut req);
            let function = self.clone();
            Box::pin(async move {
                match function(t0? ,t1? ,t2? ,t3? ,t4? ,t5? ,t6? ,t7? ,t8? ,t9? ,t10? ,t11? ,t12? ,t13? ,t14? ,t15? ,).await {
                    Ok(v) => Serializer::serialize_ok:: <R>(v),
                    Err(e) => {
                        let e = e.to_string();
                        {
                            eprintln!("ERROR: {e}");
                        };
                        Serializer::serialize_error:: <R>(e)
                    },
                
                    }
            })
        })
    }
    fn get_type_description(names:Self::NameTuple) -> FunctionDescription {
        FunctionDescription {
            args_types: <Self::NameTuple as ToArgsDescription> ::to_args_description(names,[(<<T0 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T0 as inject::Inject<_,_>> ::get_type_description()),(<<T1 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T1 as inject::Inject<_,_>> ::get_type_description()),(<<T2 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T2 as inject::Inject<_,_>> ::get_type_description()),(<<T3 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T3 as inject::Inject<_,_>> ::get_type_description()),(<<T4 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T4 as inject::Inject<_,_>> ::get_type_description()),(<<T5 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T5 as inject::Inject<_,_>> ::get_type_description()),(<<T6 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T6 as inject::Inject<_,_>> ::get_type_description()),(<<T7 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T7 as inject::Inject<_,_>> ::get_type_description()),(<<T8 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T8 as inject::Inject<_,_>> ::get_type_description()),(<<T9 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T9 as inject::Inject<_,_>> ::get_type_description()),(<<T10 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T10 as inject::Inject<_,_>> ::get_type_description()),(<<T11 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T11 as inject::Inject<_,_>> ::get_type_description()),(<<T12 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T12 as inject::Inject<_,_>> ::get_type_description()),(<<T13 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T13 as inject::Inject<_,_>> ::get_type_description()),(<<T14 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T14 as inject::Inject<_,_>> ::get_type_description()),(<<T15 as inject::Inject<_,_>> ::TExportDefinition as inject::ExportDefinition> ::VALUE, <T15 as inject::Inject<_,_>> ::get_type_description()),]),return_type:R::get_type_description(),
        }
    }

    }