use crate::{base_types::{DeserializeFromBytes, SerializationHelper, SerializeToBytes}, inject::Inject, request::Request, GetTypeDescription};

pub struct RawSerializer<T>(pub T);

impl<T> From<T> for RawSerializer<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> GetTypeDescription for RawSerializer<T>
where
    T: GetTypeDescription
{
    fn get_type_description() -> crate::TypeDescription {
        T::get_type_description()
    }
}

impl<Context, RequestState, T> Inject<Context, RequestState> for RawSerializer<T>
where
    T: DeserializeFromBytes + GetTypeDescription,
{
    const EXPORT_DEFINITION: bool = true;
    fn inject(_ctx: &Context, request: &mut Request<RequestState>) -> anyhow::Result<Self> {
        Ok(RawSerializer(T::deserialize_from_bytes(&mut request.data)?))
    }
}

impl<const SIZE: usize, T> SerializeToBytes for RawSerializer<[T; SIZE]>
where
    T: SerializeToBytes,
{
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        (SIZE as u32).serialize_to_bytes(serialization_helper)?;
        for e in self.0 {
            e.serialize_to_bytes(serialization_helper)?;
        }
        Ok(())
    }
}
impl<const SIZE: usize, T> DeserializeFromBytes for RawSerializer<[T; SIZE]>
where
    T: DeserializeFromBytes,
{
    #[inline]
    fn deserialize_from_bytes(data: &mut bytes::Bytes) -> anyhow::Result<Self> {
        let size = u32::deserialize_from_bytes(data)? as usize;
        if size != SIZE { anyhow::bail!("Deserialization error: expected {} bytes, but only {} found", size, data.len()); }
        let mut result: std::mem::MaybeUninit<[T; SIZE]> = std::mem::MaybeUninit::uninit();
        for i in 0..(size as isize) {
            unsafe { (result.as_mut_ptr() as *mut T).offset(i).write(T::deserialize_from_bytes(data)?); }
        }
        Ok(RawSerializer(unsafe { result.assume_init() }))
    }
}

impl<'a, const SIZE: usize, T> SerializeToBytes for RawSerializer<&'a [T; SIZE]>
where
    &'a T: SerializeToBytes,
{
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        (SIZE as u32).serialize_to_bytes(serialization_helper)?;
        for e in self.0 {
            e.serialize_to_bytes(serialization_helper)?;
        }
        Ok(())
    }
}

impl<'a, T> SerializeToBytes for RawSerializer<&'a [T]>
where
    &'a T: SerializeToBytes,
{
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        (self.0.len() as u32).serialize_to_bytes(serialization_helper)?;
        for e in self.0 {
            e.serialize_to_bytes(serialization_helper)?;
        }
        Ok(())
    }
}

impl<T> SerializeToBytes for RawSerializer<Vec<T>>
where
    T: SerializeToBytes,
{
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        (self.0.len() as u32).serialize_to_bytes(serialization_helper)?;
        for e in self.0 {
            e.serialize_to_bytes(serialization_helper)?;
        }
        Ok(())
    }
}
impl<T> DeserializeFromBytes for RawSerializer<Vec<T>>
where
    T: DeserializeFromBytes,
{
    #[inline]
    fn deserialize_from_bytes(data: &mut bytes::Bytes) -> anyhow::Result<Self> {
        let size = u32::deserialize_from_bytes(data)? as usize;
        let mut result = Vec::with_capacity(size);
        for _ in 0..size {
            result.push(T::deserialize_from_bytes(data)?);
        }
        Ok(RawSerializer(result))
    }
}