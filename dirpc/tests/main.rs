use bytes::{BufMut, BytesMut};
use dirpc::{context::{RequestArgDeserializer, ResponseSerializer, ServerContext}, export_types::typescript, inject::Inject, server::ServerBuilder, GetTypeDescription};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, GetTypeDescription)]
struct T0 {
    t1: T1,
    t2_custom: nested_module_test::T2,
    number: f32,
}

#[derive(Serialize, Deserialize, GetTypeDescription)]
struct T1(T3, T4);

mod nested_module_test {
    use dirpc::GetTypeDescription;
    use serde::{Deserialize, Serialize};

    use crate::T3;

        
    #[derive(Serialize, Deserialize, GetTypeDescription)]
    pub struct T2(T3);
}

#[derive(Serialize, Deserialize, GetTypeDescription)]
struct T3 {
    number: f32,
    string: String,
}

#[derive(Serialize, Deserialize, GetTypeDescription)]
struct StringWrapper(String);

#[derive(Serialize, Deserialize, GetTypeDescription)]
struct T4 {}

async fn extract_string(_conn: DbConnection, input: T0) -> String {
    input.t1.0.string
}

struct Context {
    db_connection_pool: (),
}

impl ServerContext for Context {
    type Serializer = DataSerializer;
    type Deserializer = DataDeserializer;
}

struct DataSerializer;
impl ResponseSerializer for DataSerializer {
    fn serialize<T: Serialize>(data: T) -> anyhow::Result<bytes::Bytes> {
        let mut writer = BytesMut::new();
        serde_json::to_writer((&mut writer).writer(), &data)?;
        Ok(writer.freeze())
    }
}
struct DataDeserializer;
impl RequestArgDeserializer for DataDeserializer {
    fn deserialize<T: DeserializeOwned>(data: bytes::Bytes) -> anyhow::Result<T> {
        Ok(serde_json::from_slice(&data)?)
    }
}

struct DbConnection;
impl GetTypeDescription for DbConnection {
    fn get_type_description() -> dirpc::TypeDescription {
        dirpc::TypeDescription::void()
    }
}
impl<RequestState> Inject<Context, RequestState> for DbConnection {
    const EXPORT_DEFINITION: bool = false;

    fn inject(ctx: &Context, _request: &mut dirpc::request::Request<RequestState>) -> anyhow::Result<Self> {
        let _db_connection_pool = ctx.db_connection_pool;
        Ok(Self)
    }
}

#[test]
fn test1() {
    let mut server = ServerBuilder::<Context, ()>::new();
    server.add_namespace("root").add_function("extract_string", ("conn", "input", ), extract_string);
    let descr = server.get_descr();
    server.build(Context { db_connection_pool: () }).unwrap();
    panic!("{}", typescript::get_code("TestApp", descr).unwrap());
}