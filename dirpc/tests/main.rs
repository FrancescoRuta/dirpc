use anyhow::Result;
use bytes::Bytes;
use dirpc::{
    inject::{ExportDefinitionFalse, Inject},
    publish,
    request_builder::RequestBuilder,
    rpc_serde::RpcDeserializer,
    serializers::flexbuffers::{FlexbuffersDeserializer, FlexbuffersSerializer},
    server::ServerBuilder,
    GetTypeDescription,
};
use serde::{Deserialize, Serialize};

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
    pub struct T2(pub T3);
}

#[derive(Serialize, Deserialize, GetTypeDescription)]
pub struct T3 {
    number: f32,
    string: String,
}

#[derive(Serialize, Deserialize, GetTypeDescription)]
struct StringWrapper(String);

#[derive(Serialize, Deserialize, GetTypeDescription)]
struct T4 {}

struct Context {
    db_connection_pool: (),
}

struct DbConnection;
impl GetTypeDescription for DbConnection {
    fn get_type_description() -> dirpc::TypeDescription {
        dirpc::TypeDescription::void()
    }
}
impl<RequestState> Inject<Context, RequestState> for DbConnection {
    type TExportDefinition = ExportDefinitionFalse;
    fn inject<D>(
        ctx: &Context,
        _request: &mut dirpc::request::Request<RequestState>,
    ) -> anyhow::Result<Self> {
        let _db_connection_pool = ctx.db_connection_pool;
        Ok(Self)
    }
}

async fn extract_string(_conn: DbConnection, input1: T0, input2: T0) -> Result<String> {
    Ok(format!("{}.{}", input1.t1.0.string, input2.t1.0.string))
}

async fn test() -> Result<String> {
    Ok(format!(""))
}

#[tokio::test]
async fn test1() {
    let mut server =
        ServerBuilder::<Context, (), FlexbuffersSerializer, FlexbuffersDeserializer>::new();

    publish! (server => {
        extract_string(i1, i2);
        test();
    });
    

    let _descr = server.get_descr();
    let server = server
        .build(Context {
            db_connection_pool: (),
        })
        .unwrap();

    struct Client {
        connection_in: tokio::sync::mpsc::Receiver<bytes::Bytes>,
        connection_out: tokio::sync::mpsc::Sender<bytes::Bytes>,
    }
    impl Client {
        pub async fn extract_string(&mut self, input1: T0, input2: T0) -> anyhow::Result<String> {
            let mut request: RequestBuilder<FlexbuffersSerializer> = RequestBuilder::new();
            request.push_call(1, (input1, input2))?;
            self.connection_out.send(request.build_request()).await?;
            let response = self
                .connection_in
                .recv()
                .await
                .ok_or_else(|| anyhow::anyhow!("Error"))?;
            Ok(FlexbuffersDeserializer::deserialize(response)?)
        }
    }

    let (server_in_tx, mut server_in_rx) = tokio::sync::mpsc::channel(3);
    let (server_out_tx, server_out_rx) = tokio::sync::mpsc::channel(3);

    let mut client = Client {
        connection_in: server_out_rx,
        connection_out: server_in_tx,
    };

    let server = tokio::spawn(async move {
        while let Some(request) = server_in_rx.recv().await {
            let server_out_tx = server_out_tx.clone();
            server
                .call((), request, move |response| async move {
                    server_out_tx
                        .send(Bytes::from(response.concat()))
                        .await
                        .unwrap();
                })
                .await;
        }
    });

    assert_eq!(
        client
            .extract_string(
                T0 {
                    number: 0.0,
                    t1: T1(
                        T3 {
                            number: 0.0,
                            string: String::from("test1")
                        },
                        T4 {}
                    ),
                    t2_custom: nested_module_test::T2(T3 {
                        number: 0.0,
                        string: String::from("A")
                    }),
                },
                T0 {
                    number: 0.0,
                    t1: T1(
                        T3 {
                            number: 0.0,
                            string: String::from("test2")
                        },
                        T4 {}
                    ),
                    t2_custom: nested_module_test::T2(T3 {
                        number: 0.0,
                        string: String::from("B")
                    }),
                }
            )
            .await
            .unwrap(),
        "test1.test2"
    );

    assert_ne!(
        client
            .extract_string(
                T0 {
                    number: 0.0,
                    t1: T1(
                        T3 {
                            number: 0.0,
                            string: String::from("test1")
                        },
                        T4 {}
                    ),
                    t2_custom: nested_module_test::T2(T3 {
                        number: 0.0,
                        string: String::from("A")
                    }),
                },
                T0 {
                    number: 0.0,
                    t1: T1(
                        T3 {
                            number: 0.0,
                            string: String::from("test2")
                        },
                        T4 {}
                    ),
                    t2_custom: nested_module_test::T2(T3 {
                        number: 0.0,
                        string: String::from("B")
                    }),
                }
            )
            .await
            .unwrap(),
        "test1.test21"
    );

    server.abort();
}
