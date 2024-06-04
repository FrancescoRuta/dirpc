use bytes::{Buf, BufMut, BytesMut};

use crate::rpc_serde::{RpcDeserializer, RpcSerializer};

pub struct JsonSerializer;

impl RpcSerializer for JsonSerializer {
    #[inline]
    fn serialize_unfallible<T: serde::Serialize>(element: T) -> anyhow::Result<bytes::Bytes> {
        let mut buff = BytesMut::new();
        serde_json::to_writer((&mut buff).writer(), &element)?;
        Ok(buff.freeze())
    }
    #[inline]
    fn serialize_ok<T: serde::Serialize>(element: T) -> anyhow::Result<bytes::Bytes> {
        let mut buff = BytesMut::new();
        buff.put_u8(0);
        serde_json::to_writer((&mut buff).writer(), &element)?;
        Ok(buff.freeze())
    }
    #[inline]
    fn serialize_error<T: serde::Serialize>(element: String) -> anyhow::Result<bytes::Bytes> {
        let mut buff = BytesMut::new();
        buff.put_u8(1);
        serde_json::to_writer((&mut buff).writer(), &element)?;
        Ok(buff.freeze())
    }
}
pub struct JsonDeserializer;

impl RpcDeserializer for JsonDeserializer {
    #[inline]
    fn deserialize_unfallible<T: serde::de::DeserializeOwned>(data: bytes::Bytes) -> anyhow::Result<T> {
        if cfg!(debug_assertions) {
            match serde_json::from_slice(&data) {
                Ok(data) => Ok(data),
                Err(error) => Err(anyhow::anyhow!("JsonDeserializer error: {}; OriginalMessage: {:?}", error, std::str::from_utf8(&data))),
            }
        } else {
            Ok(serde_json::from_slice(&data)?)
        }
    }
    #[inline]
    fn deserialize<T: serde::de::DeserializeOwned>(mut data: bytes::Bytes) -> anyhow::Result<std::result::Result<T, String>> {
        let error = data.get_u8();
        if error == 0 {
            Ok(Ok(serde_json::from_slice(&data)?))
        } else if error == 1 {
            Ok(Err(serde_json::from_slice(&data)?))
        } else {
            unreachable!()
        }
    }
}