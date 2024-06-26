use crate::rpc_serde::{RpcDeserializer, RpcSerializer};

pub struct FlexbuffersSerializer;

impl RpcSerializer for FlexbuffersSerializer {
    #[inline]
    fn serialize_unfallible<T: serde::Serialize>(element: T) -> anyhow::Result<bytes::Bytes> {
        let mut s = flexbuffers::FlexbufferSerializer::new();
        element.serialize(&mut s)?;
        Ok(bytes::Bytes::from(s.take_buffer()))
    }
    #[inline]
    fn serialize_ok<T: serde::Serialize>(element: T) -> anyhow::Result<bytes::Bytes> {
        let mut s = flexbuffers::FlexbufferSerializer::new();
        serde::Serialize::serialize(&std::result::Result::<T, String>::Ok(element), &mut s)?;
        Ok(bytes::Bytes::from(s.take_buffer()))
    }
    #[inline]
    fn serialize_error<T: serde::Serialize>(element: String) -> anyhow::Result<bytes::Bytes> {
        let mut s = flexbuffers::FlexbufferSerializer::new();
        serde::Serialize::serialize(&std::result::Result::<T, String>::Err(element), &mut s)?;
        Ok(bytes::Bytes::from(s.take_buffer()))
    }
}
pub struct FlexbuffersDeserializer;

impl RpcDeserializer for FlexbuffersDeserializer {
    #[inline]
    fn deserialize_unfallible<T: serde::de::DeserializeOwned>(data: bytes::Bytes) -> anyhow::Result<T> {
        Ok(T::deserialize(flexbuffers::Reader::get_root(data.as_ref())?)?)
    }
    #[inline]
    fn deserialize<T: serde::de::DeserializeOwned>(data: bytes::Bytes) -> anyhow::Result<std::result::Result<T, String>> {
        Ok(<std::result::Result<T, String> as serde::de::Deserialize>::deserialize(flexbuffers::Reader::get_root(data.as_ref())?)?)
    }
}