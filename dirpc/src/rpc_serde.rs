use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};

pub trait RpcSerializer {
    fn serialize_unfallible<T: Serialize>(element: T) -> anyhow::Result<Bytes>;
    fn serialize_ok<T: Serialize>(element: T) -> anyhow::Result<Bytes>;
    fn serialize_error<T: serde::Serialize>(error: String) -> anyhow::Result<Bytes>;
}

pub trait RpcDeserializer {
    fn deserialize<T: DeserializeOwned>(data: Bytes) -> anyhow::Result<std::result::Result<T, String>>;
    fn deserialize_unfallible<T: DeserializeOwned>(data: Bytes) -> anyhow::Result<T>;
}