use crate::rpc_serde::{RpcDeserializer, RpcSerializer};

pub struct FlexbuffersSerializer;

impl RpcSerializer for FlexbuffersSerializer {
    #[inline]
    fn serialize<T: serde::Serialize>(element: T) -> anyhow::Result<bytes::Bytes> {
        let mut s = flexbuffers::FlexbufferSerializer::new();
        element.serialize(&mut s)?;
        Ok(bytes::Bytes::from(s.take_buffer()))
    }
}
pub struct FlexbuffersDeserializer;

impl RpcDeserializer for FlexbuffersDeserializer {
    #[inline]
    fn deserialize<T: serde::de::DeserializeOwned>(data: bytes::Bytes) -> anyhow::Result<T> {
        Ok(T::deserialize(flexbuffers::Reader::get_root(data.as_ref())?)?)
    }
}