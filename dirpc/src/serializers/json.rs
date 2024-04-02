use bytes::{BufMut, BytesMut};

use crate::rpc_serde::{RpcDeserializer, RpcSerializer};

pub struct JsonSerializer;

impl RpcSerializer for JsonSerializer {
    #[inline]
    fn serialize<T: serde::Serialize>(element: T) -> anyhow::Result<bytes::Bytes> {
        let mut buff = BytesMut::new();
        serde_json::to_writer((&mut buff).writer(), &element)?;
        Ok(buff.freeze())
    }
}
pub struct JsonDeserializer;

impl RpcDeserializer for JsonDeserializer {
    #[inline]
    fn deserialize<T: serde::de::DeserializeOwned>(data: bytes::Bytes) -> anyhow::Result<T> {
        Ok(serde_json::from_slice(&data)?)
    }
}