use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};

pub trait RpcSerializer {
    fn serialize<T: Serialize>(element: T) -> anyhow::Result<Bytes>;
}

pub trait RpcDeserializer {
    fn deserialize<T: DeserializeOwned>(data: Bytes) -> anyhow::Result<T>;
}