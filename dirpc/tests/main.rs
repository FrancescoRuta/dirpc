use dirpc::{export_types::typescript, server::ServerBuilder, DeserializeFromBytes, GetTypeDescription, SerializeToBytes};

#[derive(SerializeToBytes, DeserializeFromBytes, GetTypeDescription)]
struct T0 {
    t1: T1,
    t2_custom: nested_module_test::T2,
    number: f32,
}

#[derive(SerializeToBytes, DeserializeFromBytes, GetTypeDescription)]
struct T1(T3, T4);

mod nested_module_test {
    use dirpc::{DeserializeFromBytes, GetTypeDescription, SerializeToBytes};

    use crate::T3;

        
    #[derive(SerializeToBytes, DeserializeFromBytes, GetTypeDescription)]
    pub struct T2(T3);
}

#[derive(SerializeToBytes, DeserializeFromBytes, GetTypeDescription)]
struct T3 {
    number: f32,
    string: String,
}

#[derive(SerializeToBytes, DeserializeFromBytes, GetTypeDescription)]
struct T4 {}

async fn extract_string(input: T0) -> String {
    input.t1.0.string
}

#[test]
fn test1() {
    let mut server = ServerBuilder::<(), ()>::new();
    server.add_namespace("root").add_function("extract_string", ("input", ), extract_string);
    let descr = server.get_descr();
    server.build(());
    panic!("{}", typescript::get_code("TestApp", descr).unwrap());
}