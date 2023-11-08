use std::ops::{DerefMut, Deref};
use bytes::Buf;

use crate::{put_into_response::PutIntoResponse, response::Response, request::Request, get_from_request::GetFromRequest, for_all_tuples, type_encoding::TypeEncoding};

pub struct Raw<T>(pub T);
impl<T> TypeEncoding for Raw<T>
where
    Self: GetFromRequest,
{
    type EncodedType = T;
    const NAME: &'static str = "raw";
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        <Self as GetFromRequest>::get_from_request(request)
    }
}

impl<T> Deref for Raw<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Raw<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait IntoRaw {
    type Output;
    fn into_raw(self) -> Self::Output;
}

impl<T: TypeEncoding> IntoRaw for T {
    type Output = T;
    fn into_raw(self) -> Self::Output {
        self
    }
}

impl IntoRaw for i32 {
    type Output = Raw<i32>;
    fn into_raw(self) -> Self::Output {
        Raw(self)
    }
}
impl IntoRaw for u32 {
    type Output = Raw<u32>;
    fn into_raw(self) -> Self::Output {
        Raw(self)
    }
}
impl IntoRaw for () {
    type Output = Raw<()>;
    fn into_raw(self) -> Self::Output {
        Raw(self)
    }
}

impl<T, E> IntoRaw for Result<T, E>
where
    T: IntoRaw,
    E: std::error::Error,
{
    type Output = Raw<Result<T::Output, E>>;
    fn into_raw(self) -> Self::Output {
        Raw(self.map(|v| v.into_raw()))
    }
}

impl<T> IntoRaw for Option<T>
where
    T: IntoRaw,
{
    type Output = Raw<Option<T::Output>>;
    fn into_raw(self) -> Self::Output {
        Raw(self.map(|v| v.into_raw()))
    }
}

impl PutIntoResponse for Raw<()> {
    fn put_into_response(self, _response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(())
    }
}

impl<T, E> PutIntoResponse for Raw<Result<T, E>>
where
    T: PutIntoResponse,
    E: std::error::Error,
{
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        match self.0 {
            Ok(v) => {
                response.data.push(bytes::Bytes::from_static(&[0]));
                v.put_into_response(response)
            },
            Err(v) => {
                response.data.push(bytes::Bytes::from_static(&[1]));
                Raw(v.to_string()).put_into_response(response)
            },
        }
    }
}

impl<T> GetFromRequest for Raw<Result<T, String>>
where
    T: GetFromRequest,
{
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let res = bytes::Buf::get_u8(&mut request.data);
        if res == 0 {
            Ok(Raw(Ok(T::get_from_request(request)?)))
        } else if res == 1 {
            let size = bytes::Buf::get_u32(&mut request.data) as usize;
            let string = String::from_utf8(request.data[..size].into());
            request.data.advance(size);
            Ok(Raw(Err(string?)))
        } else {
            Err("Deserialization error".into())
        }
    }
}

impl<T> PutIntoResponse for Raw<Option<T>>
where
    T: PutIntoResponse,
{
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        match self.0 {
            Some(v) => {
                response.data.push(bytes::Bytes::from_static(&[1]));
                v.put_into_response(response)
            },
            None => {
                response.data.push(bytes::Bytes::from_static(&[0]));
                Ok(())
            },
        }
    }
}

impl<T> GetFromRequest for Raw<Option<T>>
where
    T: GetFromRequest,
{
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let res = bytes::Buf::get_u8(&mut request.data);
        if res == 1 {
            Ok(Raw(Some(T::get_from_request(request)?)))
        } else if res == 0 {
            Ok(Raw(None))
        } else {
            Err("Deserialization error".into())
        }
    }
}

impl PutIntoResponse for Raw<bytes::Bytes> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        response.data.push(bytes::Bytes::copy_from_slice(&u32::to_be_bytes(self.len() as u32)));
        response.data.push(self.0);
        Ok(())
    }
}

impl GetFromRequest for Raw<bytes::Bytes> {
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let size = bytes::Buf::get_u32(&mut request.data) as usize;
        let data = request.data.slice(..size);
        request.data.advance(size);
        Ok(Raw(data))
    }
}

impl PutIntoResponse for Raw<bytes::BytesMut> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        Raw(self.0.freeze()).put_into_response(response)
    }
}

impl GetFromRequest for Raw<bytes::BytesMut> {
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Raw(bytes::BytesMut::from(&*<Raw<bytes::Bytes> as GetFromRequest>::get_from_request(request)?.0)))
    }
}

impl PutIntoResponse for Raw<Vec<u8>> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        Raw(bytes::Bytes::from(self.0)).put_into_response(response)
    }
}

impl GetFromRequest for Raw<Vec<u8>> {
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Raw(Vec::from(&*<Raw<bytes::Bytes> as GetFromRequest>::get_from_request(request)?.0)))
    }
}

impl PutIntoResponse for Raw<&[u8]> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut buffer = bytes::BytesMut::with_capacity(4 + self.len());
        bytes::BufMut::put_u32(&mut buffer, self.len() as u32);
        bytes::BufMut::put(&mut buffer, self.0);
        response.data.push(buffer.freeze());
        Ok(())
    }
}

impl PutIntoResponse for Raw<&str> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        Raw(self.as_bytes()).put_into_response(response)
    }
}

impl PutIntoResponse for Raw<String> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        Raw(self.as_bytes()).put_into_response(response)
    }
}

impl GetFromRequest for Raw<String> {
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Raw(String::from_utf8(Vec::from(&*<Raw<bytes::Bytes> as GetFromRequest>::get_from_request(request)?.0))?))
    }
}

impl<const SIZE: usize> PutIntoResponse for Raw<[u8; SIZE]> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        Raw(&self.0 as &[u8]).put_into_response(response)
    }
}

impl<const SIZE: usize> PutIntoResponse for Raw<&[u8; SIZE]> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        Raw(self.0 as &[u8]).put_into_response(response)
    }
}

impl PutIntoResponse for Raw<bool> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        if self.0 {
            response.data.push(bytes::Bytes::from_static(&[1]))
        } else {
            response.data.push(bytes::Bytes::from_static(&[0]))
        }
        Ok(())
    }
}

impl GetFromRequest for Raw<bool> {
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        match bytes::Buf::get_u8(&mut request.data) {
            0 => Ok(Raw(false)),
            1 => Ok(Raw(true)),
            _ => Err("Deserialization error".into()),
        }
    }
}
impl PutIntoResponse for Raw<i8> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        response.data.push(bytes::Bytes::copy_from_slice(&self.0.to_be_bytes()));
        Ok(())
    }
}

impl GetFromRequest for Raw<i8> {
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Raw(bytes::Buf::get_i8(&mut request.data)))
    }
}

impl PutIntoResponse for Raw<u8> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        response.data.push(bytes::Bytes::copy_from_slice(&self.0.to_be_bytes()));
        Ok(())
    }
}

impl GetFromRequest for Raw<u8> {
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Raw(bytes::Buf::get_u8(&mut request.data)))
    }
}

impl PutIntoResponse for Raw<i16> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        response.data.push(bytes::Bytes::copy_from_slice(&self.0.to_be_bytes()));
        Ok(())
    }
}

impl GetFromRequest for Raw<i16> {
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Raw(bytes::Buf::get_i16(&mut request.data)))
    }
}

impl PutIntoResponse for Raw<u16> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        response.data.push(bytes::Bytes::copy_from_slice(&self.0.to_be_bytes()));
        Ok(())
    }
}

impl GetFromRequest for Raw<u16> {
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Raw(bytes::Buf::get_u16(&mut request.data)))
    }
}

impl PutIntoResponse for Raw<i32> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        response.data.push(bytes::Bytes::copy_from_slice(&self.0.to_be_bytes()));
        Ok(())
    }
}

impl GetFromRequest for Raw<i32> {
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Raw(bytes::Buf::get_i32(&mut request.data)))
    }
}

impl PutIntoResponse for Raw<u32> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        response.data.push(bytes::Bytes::copy_from_slice(&self.0.to_be_bytes()));
        Ok(())
    }
}

impl GetFromRequest for Raw<u32> {
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Raw(bytes::Buf::get_u32(&mut request.data)))
    }
}

impl PutIntoResponse for Raw<i64> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        response.data.push(bytes::Bytes::copy_from_slice(&self.0.to_be_bytes()));
        Ok(())
    }
}

impl GetFromRequest for Raw<i64> {
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Raw(bytes::Buf::get_i64(&mut request.data)))
    }
}

impl PutIntoResponse for Raw<u64> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        response.data.push(bytes::Bytes::copy_from_slice(&self.0.to_be_bytes()));
        Ok(())
    }
}

impl GetFromRequest for Raw<u64> {
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Raw(bytes::Buf::get_u64(&mut request.data)))
    }
}

impl PutIntoResponse for Raw<f32> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        response.data.push(bytes::Bytes::copy_from_slice(&self.0.to_be_bytes()));
        Ok(())
    }
}

impl GetFromRequest for Raw<f32> {
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Raw(bytes::Buf::get_f32(&mut request.data)))
    }
}

impl PutIntoResponse for Raw<f64> {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        response.data.push(bytes::Bytes::copy_from_slice(&self.0.to_be_bytes()));
        Ok(())
    }
}

impl GetFromRequest for Raw<f64> {
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Raw(bytes::Buf::get_f64(&mut request.data)))
    }
}

macro_rules! into_raw_for_tuple {
    ( $( $t:ident $t_idx:tt; )* ) => {
        impl<$( $t, )*> IntoRaw for ($( $t, )*)
        where
            $( $t: IntoRaw, )*
        {
            type Output = Raw<($( $t::Output, )*)>;
            fn into_raw(self) -> Self::Output {
                Raw(($( self.$t_idx.into_raw(), )*))
            }
        }
    };
}

macro_rules! put_into_response_raw_tuple {
    ( $( $t:ident $t_idx:tt; )* ) => {
        impl<$( $t, )*> PutIntoResponse for Raw<($( $t, )*)>
        where
            $($t: PutIntoResponse),*
        {
            fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
                $($t::put_into_response(self.0.$t_idx, response)?;)*
                Ok(())
            }
        }
    };
}

macro_rules! get_from_request_raw_tuple {
    ( $( $t:ident $t_idx:tt; )* ) => {
        impl<$( $t, )*> GetFromRequest for Raw<($( $t, )*)>
        where
            $($t: GetFromRequest),*
        {
            fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
                Ok(Raw(($($t::get_from_request(request)?,)*)))
            }
        }
    };
}

for_all_tuples!(into_raw_for_tuple);
for_all_tuples!(put_into_response_raw_tuple);
for_all_tuples!(get_from_request_raw_tuple);
