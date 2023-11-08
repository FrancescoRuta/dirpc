use std::marker::PhantomData;

use bytes::BufMut;
use get_type_description::GetTypeDescription;
use inject::Inject;
use request::Request;
use serialization::{json::Json, raw::{Raw, IntoRaw}};
use server::{Server, TypeDescription};

mod fake_variaddic;

pub mod serialization {
    pub mod json;
    pub mod raw;
}
pub mod dyn_fn;
pub mod get_from_request;
pub mod get_type_description;
pub mod inject;
pub mod put_into_response;
pub mod request;
pub mod response;
pub mod server;
pub mod type_encoding;


struct MyContext {
    connection: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct MyComplexStruct {
    a: u32,
    b: u32,
}

impl GetTypeDescription for MyComplexStruct{
    fn get_type_description() -> server::TypeDescription {
        TypeDescription {
            encoding: None,
            kind: String::from("object"),
            name: Vec::new(),
            description: Some([
                    (String::from("a"), u32::get_type_description()),
                    (String::from("b"), u32::get_type_description()),
                ].into()),
        }
    }
}

async fn prepare_request() -> Raw<(Raw<u32>, Raw<u32>, Raw<(Raw<u32>, Json<MyComplexStruct>)>)> {
    (1u32, 21u32, (3u32, Json(MyComplexStruct { a: 1, b: 0, }))).into_raw()
}

async fn execute_request(Raw(arg0): Raw<u32>, Json(arg1): Json<MyComplexStruct>) {
    println!("arg0 = {arg0}; arg1 = {arg1:?};")
}

struct Connection<T>(String, std::marker::PhantomData<T>);

impl<T> Inject<'_, MyContext> for Connection<T> {
    fn inject(ctx: &'_ MyContext, _request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Connection(ctx.connection.clone(), PhantomData::default()))
    }
}

#[tokio::main]
async fn main() {
    
    
    let ctx = MyContext { connection: "test connection".into() };
    let mut server = Server::new(&ctx);
    server.add_function("prepare_request", prepare_request);
    server.add_function("execute_request", execute_request);
    
    let (tx, rx) = tokio::sync::oneshot::channel();
    
    server.call(Request { conn_id: 0, data: bytes::Bytes::from_static(&[0, 0, 0, 0, 0, 0, 0, 0]), }, |res| async move {
        let mut result = bytes::BytesMut::with_capacity(res.iter().map(|r| r.data.iter().map(|b| b.len()).sum::<usize>()).sum());
        res.into_iter().for_each(|r| r.data.into_iter().for_each(|b| result.put(b)));
        println!("TEST, {}", result.len());
        tx.send(result.freeze()).unwrap();
    });
    
    let request = rx.await.unwrap();
    let (tx, rx) = tokio::sync::oneshot::channel();
    
    server.call(Request { conn_id: 0, data: request, }, |_| async move {
        tx.send(()).unwrap();
    });
    
    rx.await.unwrap();
    
    dbg!(server.get_description());
    
}
