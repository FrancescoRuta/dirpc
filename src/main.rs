use serialization::{json::Json, raw::Raw};

pub mod serialization {
    pub mod json;
    pub mod raw;
}
pub mod description;
mod dyn_fn;
mod fake_variaddic;
pub mod inject;
pub mod io_bytes;
pub mod request_builder;
pub mod request;
pub mod server;


struct MyContext {
    connection: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct MyComplexStruct {
    a: u32,
    b: u32,
}

async fn prepare_request() -> (bytes::Bytes, Json<MyComplexStruct>) {
    (bytes::Bytes::from_static(b"TEST"), Json(MyComplexStruct { a: 1, b: 0, }))
}

async fn execute_request(arg0: bytes::Bytes, Json(arg1): Json<MyComplexStruct>) {
    let arg0 = std::str::from_utf8(&arg0).unwrap();
    println!("arg0 = {arg0}; arg1 = {arg1:?};")
}

async fn test_vec(Raw(arg0): Raw<Vec<String>>) -> Raw<Vec<u32>> {
    arg0.iter().for_each(|str| println!("STRING FROM VEC {str}"));
    Raw(arg0.iter().map(|s| s.len() as u32).collect())
}

struct Connection<T>(String, std::marker::PhantomData<T>);

impl<T> inject::Inject<'_, MyContext, u32> for Connection<T> {
    fn inject(ctx: &'_ MyContext, _request: &mut request::Request<u32>) -> anyhow::Result<Self> {
        Ok(Connection(ctx.connection.clone(), std::marker::PhantomData::default()))
    }
}

#[tokio::main]
async fn main() {
    let ctx = MyContext { connection: "test connection".into() };
    let mut server = server::Server::new(&ctx);
    server.add_function("prepare_request", prepare_request);
    server.add_function("execute_request", execute_request);
    server.add_function("test_vec", test_vec);
    
    let (tx, rx) = tokio::sync::oneshot::channel();
    
    let mut req = request_builder::RequestBuilder::new();
    
    req.push_call(0, ()).unwrap();
    
    server.call(0, req.into_request().pop().unwrap(), |res| async move {
        let mut result = bytes::BytesMut::with_capacity(res.iter().map(|r| r.iter().map(|b| b.len()).sum::<usize>()).sum());
        res.into_iter().for_each(|r| r.into_iter().for_each(|b| bytes::BufMut::put(&mut result, b)));
        let mut req = request_builder::RequestBuilder::new();
        println!("OK");
        req.push_call(1, <(bytes::Bytes, bytes::Bytes) as io_bytes::DeserializeFromBytes>::deserialize_from_bytes(&mut result.freeze()).unwrap()).unwrap();
        req.push_call(2, Raw(vec!["str1", "str2", "str3"])).unwrap();
        tx.send(req.into_request().pop().unwrap()).unwrap();
    });
    
    let request = rx.await.unwrap();
    let (tx, rx) = tokio::sync::oneshot::channel();
    
    server.call(0, request, |_| async move {
        tx.send(()).unwrap();
    });
    
    rx.await.unwrap();
    
    //dbg!(server.get_description());
    
}
