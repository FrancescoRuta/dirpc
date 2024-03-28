use bytes::Buf;

use crate::{dyn_fn::{DynFunction, IntoDynFunction}, request::Request, FunctionDescription, SerializationHelper, SerializeToBytes, ServerDescription};

pub struct Server<Context, RequestState> {
    ctx: Context,
    functions: Vec<DynFunction<Context, RequestState>>,
}

impl<Context, RequestState> Server<Context, RequestState>
where
    Context: Sync,
    RequestState: Clone,
{
    pub fn call<Fut: std::future::Future<Output = ()> + Send + Sync>(&self, state: RequestState, mut req_data: bytes::Bytes, send_response: impl FnOnce(Vec<Vec<bytes::Bytes>>) -> Fut + Send + Sync + 'static) -> impl std::future::Future<Output = ()> + Send + Sync + 'static {
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

pub struct ServerBuilder<Context, RequestState> {
    functions: Vec<(Vec<String>, FunctionDescription, DynFunction<Context, RequestState>)>,
}

impl<Context, RequestState> ServerBuilder<Context, RequestState> {
    
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }
    
    pub fn add_namespace<'n>(&'n mut self, name: impl Into<String>) -> ServerNamespace<'n, Context, RequestState> {
        ServerNamespace {
            path: vec![name.into()],
            server: self,
        }
    }
    
    pub fn get_descr(&self) -> ServerDescription {
        ServerDescription {
            functions: self.functions.iter().enumerate().map(|(index, (p, f, _))| (p.join("::"), (index as u32, f.clone()))).collect(),
        }
    }
    
    pub fn build(self, ctx: Context) -> Server<Context, RequestState> {
        let descr = serde_json::to_string(&self.get_descr()).unwrap();
        let mut functions: Vec<DynFunction<Context, RequestState>> = Vec::with_capacity(self.functions.len() + 1);
        functions.push(Box::new(move |_, _| {
            let descr = descr.clone();
            Box::pin(async move {
                let mut ser = SerializationHelper::new();
                (&descr).serialize_to_bytes(&mut ser)?;
                Ok(ser.chain)
            })
        }));
        functions.extend(self.functions.into_iter().map(|(_, _, f)| f));
        Server {
            ctx,
            functions,
        }
    }
    
}

pub struct ServerNamespace<'a, Context, RequestState> {
    server: &'a mut ServerBuilder<Context, RequestState>,
    path: Vec<String>,
}

impl<'a, Context, RequestState> ServerNamespace<'a, Context, RequestState> {
    pub fn add_function<F, PhantomGeneric>(&mut self, name: impl Into<String>, args: F::NameTuple, function: F)
    where
        F: IntoDynFunction<Context, RequestState, PhantomGeneric>,
    {
        let mut path = Vec::with_capacity(self.path.len() + 1);
        path.extend_from_slice(&self.path);
        path.push(name.into());
        self.server.functions.push((path, F::get_type_description(args), IntoDynFunction::into_dyn_fn(function)));
    }
    pub fn add_namespace<'n>(&'n mut self, name: impl Into<String>) -> ServerNamespace<'n, Context, RequestState> {
        let mut path = Vec::with_capacity(self.path.len() + 1);
        path.extend_from_slice(&self.path);
        path.push(name.into());
        ServerNamespace {
            path,
            server: self.server,
        }
    }
}