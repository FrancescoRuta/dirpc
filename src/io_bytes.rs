use bytes::Buf;

use crate::for_all_tuples;

pub struct SerializationHelper {
    pub(crate) chain: Vec<bytes::Bytes>,
}

impl SerializationHelper {
    pub(crate) fn new() -> Self {
        Self {
            chain: Vec::with_capacity(32),
        }
    }
}

pub trait SerializeToBytes {
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()>;
}

pub trait DeserializeFromBytes where Self: Sized {
    fn deserialize_from_bytes(data: &mut bytes::Bytes) -> anyhow::Result<Self>;
}

macro_rules! ser_de_number {
    ($($t:ident $get_method:ident;)*) => {
        $(
            impl SerializeToBytes for $t {
                fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
                    serialization_helper.chain.push(bytes::Bytes::copy_from_slice(&self.to_be_bytes()));
                    Ok(())
                }
            }
            impl DeserializeFromBytes for $t {
                fn deserialize_from_bytes(data: &mut bytes::Bytes) -> anyhow::Result<Self> {
                    if data.len() >= std::mem::size_of::<Self>() {
                        Ok(bytes::Buf::$get_method(data))
                    } else {
                        Err(anyhow::anyhow!("Deserialization error."))
                    }
                }
            }
        )*
    };
}
ser_de_number! {
    i8 get_i8;
    u8 get_u8;
    i16 get_i16;
    u16 get_u16;
    i32 get_i32;
    u32 get_u32;
    i64 get_i64;
    u64 get_u64;
    f32 get_f32;
    f64 get_f64;
}

impl SerializeToBytes for () {
    #[inline]
    fn serialize_to_bytes(self, _serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        Ok(())
    }
}
impl DeserializeFromBytes for () {
    #[inline]
    fn deserialize_from_bytes(_data: &mut bytes::Bytes) -> anyhow::Result<Self> {
        Ok(())
    }
}

impl SerializeToBytes for bool {
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        if self {
            1u8.serialize_to_bytes(serialization_helper)
        } else {
            0u8.serialize_to_bytes(serialization_helper)
        }
    }
}
impl DeserializeFromBytes for bool {
    #[inline]
    fn deserialize_from_bytes(data: &mut bytes::Bytes) -> anyhow::Result<Self> {
        match u8::deserialize_from_bytes(data)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(anyhow::anyhow!("Deserialization error.")),
        }
    }
}

impl<T, E> SerializeToBytes for Result<T, E>
where
    T: SerializeToBytes,
    E: std::error::Error,
{
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        match self {
            Ok(v) => {
                1u8.serialize_to_bytes(serialization_helper)?;
                v.serialize_to_bytes(serialization_helper)?;
            },
            Err(v) => {
                let v = v.to_string();
                0u8.serialize_to_bytes(serialization_helper)?;
                v.serialize_to_bytes(serialization_helper)?;
            },
        }
        Ok(())
    }
}
impl<T> DeserializeFromBytes for Result<T, String>
where
    T: DeserializeFromBytes,
{
    #[inline]
    fn deserialize_from_bytes(data: &mut bytes::Bytes) -> anyhow::Result<Self> {
        match u8::deserialize_from_bytes(data)? {
            0 => Ok(Err(DeserializeFromBytes::deserialize_from_bytes(data)?)),
            1 => Ok(Ok(DeserializeFromBytes::deserialize_from_bytes(data)?)),
            _ => Err(anyhow::anyhow!("Deserialization error.")),
        }
    }
}

impl<T> SerializeToBytes for Option<T>
where
    T: SerializeToBytes,
{
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        if let Some(v) = self {
            1u8.serialize_to_bytes(serialization_helper)?;
            v.serialize_to_bytes(serialization_helper)?;
        } else {
            0u8.serialize_to_bytes(serialization_helper)?;
        }
        Ok(())
    }
}
impl<T> DeserializeFromBytes for Option<T>
where
    T: DeserializeFromBytes,
{
    #[inline]
    fn deserialize_from_bytes(data: &mut bytes::Bytes) -> anyhow::Result<Self> {
        match u8::deserialize_from_bytes(data)? {
            0 => Ok(None),
            1 => Ok(Some(DeserializeFromBytes::deserialize_from_bytes(data)?)),
            _ => Err(anyhow::anyhow!("Deserialization error.")),
        }
    }
}

impl SerializeToBytes for bytes::Bytes {
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        (self.len() as u32).serialize_to_bytes(serialization_helper)?;
        serialization_helper.chain.push(self);
        Ok(())
    }
}
impl DeserializeFromBytes for bytes::Bytes {
    #[inline]
    fn deserialize_from_bytes(data: &mut bytes::Bytes) -> anyhow::Result<Self> {
        let size = u32::deserialize_from_bytes(data)? as usize;
        if size > data.len() { anyhow::bail!("Deserialization error."); }
        let result = data.slice(..size);
        data.advance(size);
        Ok(result)
    }
}

impl SerializeToBytes for String {
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        bytes::Bytes::copy_from_slice(self.as_bytes()).serialize_to_bytes(serialization_helper)
    }
}
impl DeserializeFromBytes for String {
    #[inline]
    fn deserialize_from_bytes(data: &mut bytes::Bytes) -> anyhow::Result<Self> {
        let slice = bytes::Bytes::deserialize_from_bytes(data)?;
        Ok(String::from_utf8(slice.into())?)
    }
}

impl SerializeToBytes for &str {
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        bytes::Bytes::copy_from_slice(self.as_bytes()).serialize_to_bytes(serialization_helper)
    }
}

impl SerializeToBytes for bytes::BytesMut {
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        self.freeze().serialize_to_bytes(serialization_helper)
    }
}
impl DeserializeFromBytes for bytes::BytesMut {
    #[inline]
    fn deserialize_from_bytes(data: &mut bytes::Bytes) -> anyhow::Result<Self> {
        let size = u32::deserialize_from_bytes(data)? as usize;
        if size > data.len() { anyhow::bail!("Deserialization error."); }
        let mut result = bytes::BytesMut::with_capacity(size);
        bytes::BufMut::put(&mut result, &data[..size]);
        data.advance(size);
        Ok(result)
    }
}

impl SerializeToBytes for Vec<u8> {
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        bytes::Bytes::from(self).serialize_to_bytes(serialization_helper)
    }
}
impl DeserializeFromBytes for Vec<u8> {
    #[inline]
    fn deserialize_from_bytes(data: &mut bytes::Bytes) -> anyhow::Result<Self> {
        let size = u32::deserialize_from_bytes(data)? as usize;
        if size > data.len() { anyhow::bail!("Deserialization error."); }
        let mut result = Vec::with_capacity(size);
        bytes::BufMut::put(&mut result, &data[..size]);
        data.advance(size);
        Ok(result)
    }
}

impl SerializeToBytes for &[u8] {
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        bytes::Bytes::copy_from_slice(self).serialize_to_bytes(serialization_helper)
    }
}

impl<const SIZE: usize> SerializeToBytes for [u8; SIZE] {
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        bytes::Bytes::copy_from_slice(&self).serialize_to_bytes(serialization_helper)
    }
}
impl<const SIZE: usize> DeserializeFromBytes for [u8; SIZE] {
    #[inline]
    fn deserialize_from_bytes(data: &mut bytes::Bytes) -> anyhow::Result<Self> {
        let size = u32::deserialize_from_bytes(data)? as usize;
        if size > data.len() { anyhow::bail!("Deserialization error."); }
        let result = &data[..size];
        let result = result.try_into()?;
        data.advance(size);
        Ok(result)
    }
}

impl<const SIZE: usize> SerializeToBytes for &[u8; SIZE] {
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        bytes::Bytes::copy_from_slice(self).serialize_to_bytes(serialization_helper)
    }
}

macro_rules! ser_de_tuple {
    ( $( $t:ident $t_idx:tt; )* ) => {
        impl<$( $t, )*> SerializeToBytes for ($( $t, )*)
        where
            $($t: SerializeToBytes),*
        {
            #[inline]
            fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
                $($t::serialize_to_bytes(self.$t_idx, serialization_helper)?;)*
                Ok(())
            }
        }
        impl<$( $t, )*> DeserializeFromBytes for ($( $t, )*)
        where
            $($t: DeserializeFromBytes),*
        {
            #[inline]
            fn deserialize_from_bytes(data: &mut bytes::Bytes) -> anyhow::Result<Self> {
                Ok(($($t::deserialize_from_bytes(data)?,)*))
            }
        }
    };
}

for_all_tuples!(ser_de_tuple);