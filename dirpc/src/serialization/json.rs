use crate::base_types::{SerializeToBytes, DeserializeFromBytes, SerializationHelper};

pub struct Json<T>(pub T);

impl<T> SerializeToBytes for Json<T>
where
    T: serde::Serialize,
{
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        let mut buffer = bytes::BytesMut::with_capacity(16 * 1024);
        serde_json::to_writer(bytes::BufMut::writer(&mut buffer), &self.0)?;
        let buffer = buffer.freeze();
        (buffer.len() as u32).serialize_to_bytes(serialization_helper)?;
        serialization_helper.chain.push(buffer);
        Ok(())
    }
}
impl<T> DeserializeFromBytes for Json<T>
where
    T: for<'de> serde::Deserialize<'de>,
{
    #[inline]
    fn deserialize_from_bytes(data: &mut bytes::Bytes) -> anyhow::Result<Self> {
        Ok(Json(serde_json::from_slice(&*bytes::Bytes::deserialize_from_bytes(data)?)?))
    }
}