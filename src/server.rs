use std::collections::HashMap;

use bytes::Buf;
use serde::{Serialize, Deserialize};

use crate::{dyn_fn::{DynFunction, IntoDynFunction}, request::Request, response::Response};

pub struct Server<'ctx, Context> {
    context: &'ctx Context,
    functions: Vec<(Vec<String>, FunctionDescription, DynFunction<'ctx, Context>)>,
    consts: Vec<(Vec<String>, ConstDescription)>,
}

impl<'ctx, Context> Server<'ctx, Context>
where
    Context: Sync,
{
    
    pub fn new(context: &'ctx Context) -> Self {
        Self {
            context,
            functions: Vec::new(),
            consts: Vec::new(),
        }
    }
    
    pub fn add_function<F, PhantomGeneric>(&mut self, name: impl Into<String>, function: F)
    where
        F: IntoDynFunction<'ctx, Context, PhantomGeneric>,
    {
        self.functions.push((vec![name.into()], F::get_function_description(), IntoDynFunction::into_dyn_fn(function)))
    }
    
    pub fn add_const(&mut self, name: impl Into<String>, value: ConstDescription) {
        self.consts.push((vec![name.into()], value))
    }
    
    pub fn add_namespace(&mut self, name: impl Into<String>, namespace: ServerNamespace<'ctx, Context>) {
        let name = name.into();
        self.functions.reserve(namespace.functions.len());
        for (mut path, description, function) in namespace.functions {
            path.push(name.clone());
            self.functions.push((path, description, function))
        }
        self.consts.reserve(namespace.consts.len());
        for (mut path, value) in namespace.consts {
            path.push(name.clone());
            self.consts.push((path, value))
        }
    }
    
    pub fn get_description(&self) -> ServerDescription {
        ServerDescription {
            consts: HashMap::new(),
            functions: self.functions.iter().map(|(path, desc, _)| (path.clone(), desc.clone())).collect(),
            types: HashMap::new(),
        }
    }
    
    pub fn call<Fut>(&self, request: Request, send_response: impl FnOnce(Vec<Response>) -> Fut + Send + 'static)
    where
        Fut: std::future::Future<Output = ()> + Send,
    {
        let conn_id = request.conn_id;
        let mut futures = Vec::with_capacity(16);
        let mut req_data = request.data;
        while req_data.len() > 0 && futures.len() < 16 {
            if req_data.len() < 8 { return }
            let index = req_data.get_u32() as usize;
            let size = req_data.get_u32() as usize;
            if req_data.len() < size { return }
            let data = req_data.slice(..size);
            req_data.advance(size);
            futures.push((self.functions[index].2)(self.context, Request { conn_id, data }));
        }
        tokio::spawn(async move {
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
        });
    }
    
}

pub struct ServerNamespace<'ctx, Context> {
    functions: Vec<(Vec<String>, FunctionDescription, DynFunction<'ctx, Context>)>,
    consts: Vec<(Vec<String>, ConstDescription)>,
}

impl<'ctx, Context> ServerNamespace<'ctx, Context> {
    
    
    pub fn add_function<F, PhantomGeneric>(&mut self, name: impl Into<String>, function: F)
    where
        F: IntoDynFunction<'ctx, Context, PhantomGeneric>,
    {
        self.functions.push((vec![name.into()], F::get_function_description(), IntoDynFunction::into_dyn_fn(function)))
    }
    
    pub fn add_const(&mut self, name: impl Into<String>, value: ConstDescription) {
        self.consts.push((vec![name.into()], value))
    }
    
    pub fn add_namespace(&mut self, name: impl Into<String>, namespace: ServerNamespace<'ctx, Context>) {
        let name = name.into();
        self.functions.reserve(namespace.functions.len());
        for (mut path, description, function) in namespace.functions {
            path.push(name.clone());
            self.functions.push((path, description, function))
        }
        self.consts.reserve(namespace.consts.len());
        for (mut path, value) in namespace.consts {
            path.push(name.clone());
            self.consts.push((path, value))
        }
    }
    
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDescription {
    pub types: HashMap<Vec<String>, TypeDescription>,
    pub functions: HashMap<Vec<String>, FunctionDescription>,
    pub consts: HashMap<Vec<String>, ConstDescription>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TypeDescription {
    pub encoding: Option<String>,
    pub kind: String,
    pub name: Vec<String>,
    pub description: Option<HashMap<String, TypeDescription>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDescription {
    pub args_types: Vec<TypeDescription>,
    pub return_type: TypeDescription,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstDescription {
    
}