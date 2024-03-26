use bytes::Buf;

use crate::{description::{GetTypeDescription, TypeDescription, TypeInfo}, for_all_tuples};

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
            impl SerializeToBytes for &$t {
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
                        Err(anyhow::anyhow!("Deserialization error: expected {} bytes, but only {} found", std::mem::size_of::<Self>(), data.len()))
                    }
                }
            }
        )*
    };
}

macro_rules! get_type_description_number {
    ($($t0:ident $t1:ident;)*) => {
        $(
            impl GetTypeDescription for $t0 {
                fn get_type_description() -> TypeDescription {
                    TypeDescription {
                        module_path: String::new(),
                        name: String::new(),
                        typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::$t1),
                    }
                }
            }
            impl GetTypeDescription for &$t0 {
                fn get_type_description() -> TypeDescription {
                    TypeDescription {
                        module_path: String::new(),
                        name: String::new(),
                        typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::$t1),
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

get_type_description_number! {
    i8 I8;
    u8 U8;
    i16 I16;
    u16 U16;
    i32 I32;
    u32 U32;
    i64 I64;
    u64 U64;
    f32 F32;
    f64 F64;
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
impl GetTypeDescription for () {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::Void),
        }
    }
}

impl SerializeToBytes for &() {
    #[inline]
    fn serialize_to_bytes(self, _serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        Ok(())
    }
}
impl GetTypeDescription for &() {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::Void),
        }
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
            v => Err(anyhow::anyhow!("Deserialization error: expected the value to be 1 or 0, but {v} was found.")),
        }
    }
}
impl GetTypeDescription for bool {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::Bool),
        }
    }
}

impl SerializeToBytes for &bool {
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        if *self {
            1u8.serialize_to_bytes(serialization_helper)
        } else {
            0u8.serialize_to_bytes(serialization_helper)
        }
    }
}
impl GetTypeDescription for &bool {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::Bool),
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
            v => Err(anyhow::anyhow!("Deserialization error: expected the value to be 1 or 0, but {v} was found.")),
        }
    }
}
impl<T, E> GetTypeDescription for Result<T, E>
where
    T: GetTypeDescription,
    E: std::error::Error,
{
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::Result(Box::new(T::get_type_description())),
        }
    }
}
impl<'a, 'b, T, E> SerializeToBytes for &'b Result<T, E>
where
    'b: 'a,
    &'a T: SerializeToBytes + 'a,
    E: std::error::Error,
{
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        match self.as_ref() {
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
impl<T, E> GetTypeDescription for &Result<T, E>
where
    T: GetTypeDescription,
    E: std::error::Error,
{
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::Result(Box::new(T::get_type_description())),
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
            v => Err(anyhow::anyhow!("Deserialization error: expected the value to be 1 or 0, but {v} was found.")),
        }
    }
}
impl<T> GetTypeDescription for Option<T>
where
    T: GetTypeDescription,
{
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::Option(Box::new(T::get_type_description())),
        }
    }
}
impl<'a, 'b, T> SerializeToBytes for &'b Option<T>
where
    'b: 'a,
    &'a T: SerializeToBytes + 'a,
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
impl<T> GetTypeDescription for &Option<T>
where
    T: GetTypeDescription,
{
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::Option(Box::new(T::get_type_description())),
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
        if size > data.len() { anyhow::bail!("Deserialization error: expected {} bytes, but only {} found", size, data.len()); }
        let result = data.slice(..size);
        data.advance(size);
        Ok(result)
    }
}
impl GetTypeDescription for bytes::Bytes {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::ByteArray),
        }
    }
}

impl SerializeToBytes for &bytes::Bytes {
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        (self.len() as u32).serialize_to_bytes(serialization_helper)?;
        serialization_helper.chain.push(self.clone());
        Ok(())
    }
}
impl GetTypeDescription for &bytes::Bytes {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::ByteArray),
        }
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
impl GetTypeDescription for String {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::String),
        }
    }
}

impl SerializeToBytes for &String {
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        bytes::Bytes::copy_from_slice(self.as_bytes()).serialize_to_bytes(serialization_helper)
    }
}
impl GetTypeDescription for &String {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::String),
        }
    }
}

impl SerializeToBytes for &str {
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        bytes::Bytes::copy_from_slice(self.as_bytes()).serialize_to_bytes(serialization_helper)
    }
}
impl GetTypeDescription for &str {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::String),
        }
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
        if size > data.len() { anyhow::bail!("Deserialization error: expected {} bytes, but only {} found", size, data.len()); }
        let mut result = bytes::BytesMut::with_capacity(size);
        bytes::BufMut::put(&mut result, &data[..size]);
        data.advance(size);
        Ok(result)
    }
}
impl GetTypeDescription for bytes::BytesMut {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::ByteArray),
        }
    }
}

impl SerializeToBytes for &[u8] {
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        bytes::Bytes::copy_from_slice(self).serialize_to_bytes(serialization_helper)
    }
}
impl GetTypeDescription for &[u8] {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::ByteArray),
        }
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
        if size > data.len() { anyhow::bail!("Deserialization error: expected {} bytes, but only {} found", size, data.len()); }
        let result = &data[..size];
        let result = result.try_into()?;
        data.advance(size);
        Ok(result)
    }
}
impl<const SIZE: usize> GetTypeDescription for [u8; SIZE] {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::ByteArray),
        }
    }
}

impl<const SIZE: usize> SerializeToBytes for &[u8; SIZE] {
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        bytes::Bytes::copy_from_slice(self).serialize_to_bytes(serialization_helper)
    }
}
impl<const SIZE: usize> GetTypeDescription for &[u8; SIZE] {
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::BaseType(crate::description::BaseTypeDescription::ByteArray),
        }
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
        impl<$( $t, )*> GetTypeDescription for ($( $t, )*)
        where
            $($t: GetTypeDescription),*
        {
            fn get_type_description() -> TypeDescription {
                TypeDescription {
                    module_path: String::new(),
                    name: String::new(),
                    typeinfo: TypeInfo::Tuple(vec![$( $t::get_type_description(), )*]),
                }
            }
        }
    };
}

for_all_tuples!(ser_de_tuple);



impl<T> SerializeToBytes for Vec<T>
where
    T: SerializeToBytes,
{
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        (self.len() as u32).serialize_to_bytes(serialization_helper)?;
        for i in self {
            i.serialize_to_bytes(serialization_helper)?;
        }
        Ok(())
    }
}
impl<T> DeserializeFromBytes for Vec<T>
where
    T: DeserializeFromBytes,
{
    #[inline]
    fn deserialize_from_bytes(data: &mut bytes::Bytes) -> anyhow::Result<Self> {
        let size = u32::deserialize_from_bytes(data)? as usize;
        let mut result = Vec::with_capacity(size);
        for _ in 0..size {
            result.push(DeserializeFromBytes::deserialize_from_bytes(data)?);
        }
        Ok(result)
    }
}
impl<T> GetTypeDescription for Vec<T>
where
    T: GetTypeDescription,
{
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::Array(Box::new(T::get_type_description())),
        }
    }
}


impl<'a, 'b, T> SerializeToBytes for &'b Vec<T>
where
    'b: 'a,
    &'a T: SerializeToBytes + 'a,
{
    #[inline]
    fn serialize_to_bytes(self, serialization_helper: &mut SerializationHelper) -> anyhow::Result<()> {
        (self.len() as u32).serialize_to_bytes(serialization_helper)?;
        for i in self {
            i.serialize_to_bytes(serialization_helper)?;
        }
        Ok(())
    }
}
impl<T> GetTypeDescription for &Vec<T>
where
    T: GetTypeDescription,
{
    fn get_type_description() -> TypeDescription {
        TypeDescription {
            module_path: String::new(),
            name: String::new(),
            typeinfo: TypeInfo::Array(Box::new(T::get_type_description())),
        }
    }
}
