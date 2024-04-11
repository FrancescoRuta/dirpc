use bytes::Buf;
use serde::de::DeserializeOwned;

use crate::{request::Request, rpc_serde::RpcDeserializer, GetTypeDescription, TypeDescription};

pub trait Inject<Context, RequestState> where Self: Sized {
    const EXPORT_DEFINITION: bool = false;
    fn inject<Deserializer: RpcDeserializer>(ctx: &Context, request: &mut Request<RequestState>) -> anyhow::Result<Self>;
    fn get_type_description() -> TypeDescription {
        TypeDescription::void()
    }
}

impl<'de, Context, RequestState, T> Inject<Context, RequestState> for T
where
    T: DeserializeOwned + GetTypeDescription,
{
    const EXPORT_DEFINITION: bool = true;
    fn inject<Deserializer: RpcDeserializer>(_ctx: &Context, request: &mut Request<RequestState>) -> anyhow::Result<Self> {
        let size = request.data.get_u32() as usize;
        if size > request.data.len() { anyhow::bail!("Deserialization error: expected {} bytes, but only {} found", size, request.data.len()); }
        let result = request.data.slice(..size);
        request.data.advance(size);
        Ok(Deserializer::deserialize(result)?)
    }
    fn get_type_description() -> TypeDescription {
        <Self as GetTypeDescription>::get_type_description()
    }
}