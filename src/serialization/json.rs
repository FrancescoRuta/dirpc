use std::ops::{DerefMut, Deref};

use crate::{put_into_response::PutIntoResponse, response::Response, request::Request, type_encoding::TypeEncoding, get_type_description::GetTypeDescription, get_from_request::GetFromRequest};

pub struct Json<T>(pub T);

impl<T> TypeEncoding for Json<T>
where
    T: for<'de> serde::Deserialize<'de> + GetTypeDescription,
{
    type EncodedType = T;
    const NAME: &'static str = "json";
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        use bytes::Buf;
        let size = request.data.get_u32() as usize;
        let result = serde_json::from_slice(&request.data[..size]);
        request.data.advance(size);
        Ok(Json(result?))
    }
}

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait IntoJson {
    type Output;
    fn into_json(self) -> Self::Output;
}

impl<T> IntoJson for T {
    type Output = Json<T>;
    fn into_json(self) -> Self::Output {
        Json(self)
    }
}

impl<T> From<T> for Json<T> {
    fn from(value: T) -> Json<T> {
        Json(value)
    }
}

impl<T> PutIntoResponse for Json<T> where T: serde::Serialize {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        use bytes::BufMut;
        let mut buffer = bytes::BytesMut::with_capacity(8 * 1024);
        serde_json::to_writer((&mut buffer).writer(), &self.0)?;
        let buffer = buffer.freeze();
        response.data.push(bytes::Bytes::copy_from_slice(&u32::to_be_bytes(buffer.len() as u32)));
        response.data.push(buffer);
        Ok(())
    }
}

impl<T> GetFromRequest for Json<T>
where
    T: for<'de> serde::Deserialize<'de> + GetTypeDescription,
{
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        <Json<T> as TypeEncoding>::get_from_request(request)
    }
}