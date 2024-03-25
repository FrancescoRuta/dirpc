use bytes::Buf;

use crate::{dyn_fn::{DynFunction, IntoDynFunction}, request::Request};

pub struct Server<Context, RequestState> {
    context: Context,
    functions: Vec<(Vec<String>, DynFunction<Context, RequestState>)>,
}

impl<Context, RequestState> Server<Context, RequestState>
where
    Context: Sync,
    RequestState: Clone,
{
    
    pub fn new(context: Context) -> Self {
        Self {
            context,
            functions: Vec::new(),
        }
    }
    
    pub fn add_function<F, PhantomGeneric>(&mut self, name: impl Into<String>, function: F)
    where
        F: IntoDynFunction<Context, RequestState, PhantomGeneric>,
    {
        self.functions.push((vec![name.into()], IntoDynFunction::into_dyn_fn(function)))
    }
    
    //pub fn add_const(&mut self, name: impl Into<String>, value: ConstDescription) {
    //    self.consts.push((vec![name.into()], value))
    //}
    //
    //pub fn add_namespace(&mut self, name: impl Into<String>, namespace: ServerNamespace<Context>) {
    //    let name = name.into();
    //    self.functions.reserve(namespace.functions.len());
    //    for (mut path, description, function) in namespace.functions {
    //        path.push(name.clone());
    //        self.functions.push((path, description, function))
    //    }
    //    self.consts.reserve(namespace.consts.len());
    //    for (mut path, value) in namespace.consts {
    //        path.push(name.clone());
    //        self.consts.push((path, value))
    //    }
    //}
    //
    //pub fn get_description(&self) -> ServerDescription {
    //    ServerDescription {
    //        consts: HashMap::new(),
    //        functions: self.functions.iter().map(|(path, desc, _)| (path.clone(), desc.clone())).collect(),
    //        types: HashMap::new(),
    //    }
    //}
    
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
            futures.push((self.functions[index].1)(&self.context, Request { state: state.clone(), data }));
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

//pub struct ServerNamespace<Context> {
//    functions: Vec<(Vec<String>, FunctionDescription, DynFunction<Context>)>,
//    consts: Vec<(Vec<String>, ConstDescription)>,
//}
//
//impl<Context> ServerNamespace<Context> {
//    
//    
//    pub fn add_function<F, PhantomGeneric>(&mut self, name: impl Into<String>, function: F)
//    where
//        F: IntoDynFunction<Context, PhantomGeneric>,
//    {
//        self.functions.push((vec![name.into()], F::get_function_description(), IntoDynFunction::into_dyn_fn(function)))
//    }
//    
//    pub fn add_const(&mut self, name: impl Into<String>, value: ConstDescription) {
//        self.consts.push((vec![name.into()], value))
//    }
//    
//    pub fn add_namespace(&mut self, name: impl Into<String>, namespace: ServerNamespace<Context>) {
//        let name = name.into();
//        self.functions.reserve(namespace.functions.len());
//        for (mut path, description, function) in namespace.functions {
//            path.push(name.clone());
//            self.functions.push((path, description, function))
//        }
//        self.consts.reserve(namespace.consts.len());
//        for (mut path, value) in namespace.consts {
//            path.push(name.clone());
//            self.consts.push((path, value))
//        }
//    }
//    
//    
//}