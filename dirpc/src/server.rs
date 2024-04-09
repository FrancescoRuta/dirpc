use std::marker::PhantomData;

use bytes::Buf;

use crate::{rpc_serde::{RpcDeserializer, RpcSerializer}, dyn_fn::{DynFunction, IntoDynFunction}, request::Request, FunctionDescription, ServerDescription};

pub struct Server<Context, RequestState, Serializer: RpcSerializer, Deserializer: RpcDeserializer> {
    ctx: Context,
    functions: Vec<DynFunction<Context, RequestState>>,
    _phantom_data: PhantomData<(Serializer, Deserializer)>,
}

impl<Context, RequestState, Serializer: RpcSerializer, Deserializer: RpcDeserializer> Server<Context, RequestState, Serializer, Deserializer>
where
    Context: Sync,
    RequestState: Clone,
{
    pub fn call<Fut: std::future::Future<Output = ()> + Send>(&self, state: RequestState, mut req_data: bytes::Bytes, send_response: impl FnOnce(Vec<bytes::Bytes>) -> Fut + Send + Sync + 'static) -> impl std::future::Future<Output = ()> + Send + 'static {
        let mut futures = Vec::with_capacity(16);
        while req_data.len() > 0 && futures.len() < 16 {
            if req_data.len() < 8 {
                futures.clear();
                break;
            }
            let index = req_data.get_u32() as usize;
            let size = req_data.get_u32() as usize;
            if req_data.len() < size {
                futures.clear();
                break;
            }
            let data = req_data.slice(..size);
            req_data.advance(size);
            futures.push((self.functions[index])(&self.ctx, Request { state: state.clone(), data }));
        }
        async move {
            let mut results = Vec::with_capacity(futures.len());
            for future in futures {
                match future.await {
                    Ok(r) => results.push(r),
                    Err(error) => {
                        eprintln!("ERROR: {error}");
                        return;
                    },
                }
            }
            send_response(results).await;
        }
    }
}

pub struct ServerBuilder<Context, RequestState, Serializer: RpcSerializer, Deserializer: RpcDeserializer> {
    functions: Vec<(Vec<String>, FunctionDescription, DynFunction<Context, RequestState>)>,
    _phantom_data: PhantomData<(Serializer, Deserializer)>,
}

impl<Context, RequestState, Serializer: RpcSerializer, Deserializer: RpcDeserializer> ServerBuilder<Context, RequestState, Serializer, Deserializer> {
    
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            _phantom_data: PhantomData,
        }
    }
    
    pub fn get_descr(&self) -> ServerDescription {
        ServerDescription {
            functions: self.functions.iter().enumerate().map(|(index, (p, f, _))| (p.join("::"), (index as u32 + 1, f.clone()))).collect(),
        }
    }
    
    pub fn build(self, ctx: Context) -> anyhow::Result<Server<Context, RequestState, Serializer, Deserializer>> {
        let descr = serde_json::to_string(&self.get_descr())?;
        let descr = Serializer::serialize(descr)?;
        let mut functions: Vec<DynFunction<Context, RequestState>> = Vec::with_capacity(self.functions.len() + 1);
        functions.push(Box::new(move |_, _| {
            let descr = descr.clone();
            Box::pin(async move {Ok(descr)})
        }));
        functions.extend(self.functions.into_iter().map(|(_, _, f)| f));
        Ok(Server {
            ctx,
            functions,
            _phantom_data: PhantomData,
        })
    }
    
}

impl<Context, RequestState, Serializer: RpcSerializer, Deserializer: RpcDeserializer> ServerAddFunctionality<Context, RequestState> for ServerBuilder<Context, RequestState, Serializer, Deserializer> {
    fn add_function<F, PhantomGeneric>(&mut self, name: impl Into<String>, args: F::NameTuple, function: F)
    where
        F: IntoDynFunction<Context, RequestState, PhantomGeneric>,
    {
        self.functions.push((vec![name.into()], F::get_type_description(args), IntoDynFunction::into_dyn_fn::<Serializer, Deserializer>(function)));
    }
    fn add_namespace<'n>(&'n mut self, name: impl Into<String>) -> impl ServerAddFunctionality<Context, RequestState> {
        ServerNamespace {
            path: vec![name.into()],
            server: self,
            _phantom_data: PhantomData,
        }
    }
}

pub struct ServerNamespace<'a, Context, RequestState, Serializer: RpcSerializer, Deserializer: RpcDeserializer> {
    server: &'a mut ServerBuilder<Context, RequestState, Serializer, Deserializer>,
    path: Vec<String>,
    _phantom_data: PhantomData<(Serializer, Deserializer)>,
}

impl<'a, Context, RequestState, Serializer: RpcSerializer, Deserializer: RpcDeserializer> ServerAddFunctionality<Context, RequestState> for ServerNamespace<'a, Context, RequestState, Serializer, Deserializer> {
    fn add_function<F, PhantomGeneric>(&mut self, name: impl Into<String>, args: F::NameTuple, function: F)
    where
        F: IntoDynFunction<Context, RequestState, PhantomGeneric>,
    {
        let mut path = Vec::with_capacity(self.path.len() + 1);
        path.extend_from_slice(&self.path);
        path.push(name.into());
        self.server.functions.push((path, F::get_type_description(args), IntoDynFunction::into_dyn_fn::<Serializer, Deserializer>(function)));
    }
    fn add_namespace<'n>(&'n mut self, name: impl Into<String>) -> impl ServerAddFunctionality<Context, RequestState> {
        let mut path = Vec::with_capacity(self.path.len() + 1);
        path.extend_from_slice(&self.path);
        path.push(name.into());
        ServerNamespace {
            path,
            server: self.server,
            _phantom_data: PhantomData,
        }
    }
}

pub trait ServerAddFunctionality<Context, RequestState> {
    fn add_function<F, PhantomGeneric>(&mut self, name: impl Into<String>, args: F::NameTuple, function: F)
    where
        F: IntoDynFunction<Context, RequestState, PhantomGeneric>;
    fn add_namespace<'n>(&'n mut self, name: impl Into<String>) -> impl ServerAddFunctionality<Context, RequestState>;
}

impl<Context, RequestState, T> ServerAddFunctionality<Context, RequestState> for &mut T
where
    T: ServerAddFunctionality<Context, RequestState>,
{
    fn add_function<F, PhantomGeneric>(&mut self, name: impl Into<String>, args: F::NameTuple, function: F)
    where
        F: IntoDynFunction<Context, RequestState, PhantomGeneric> {
        T::add_function(self, name, args, function)
    }

    fn add_namespace<'n>(&'n mut self, name: impl Into<String>) -> impl ServerAddFunctionality<Context, RequestState> {
        T::add_namespace(self, name)
    }
}