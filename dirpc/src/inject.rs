use bytes::Bytes;
use serde::de::DeserializeOwned;

use crate::{base_types::DeserializeFromBytes, context::ServerContext, request::Request, GetTypeDescription};

pub trait Inject<Context, RequestState> where Self: Sized + GetTypeDescription {
    const EXPORT_DEFINITION: bool;
    fn inject(ctx: &Context, request: &mut Request<RequestState>) -> anyhow::Result<Self>;
}

impl<'de, Context, RequestState, T> Inject<Context, RequestState> for T
where
    T: DeserializeOwned + GetTypeDescription,
    Context: ServerContext,
{
    const EXPORT_DEFINITION: bool = true;
    fn inject(_ctx: &Context, request: &mut Request<RequestState>) -> anyhow::Result<Self> {
        let data = Bytes::deserialize_from_bytes(&mut request.data)?;
        <Context::Deserializer as crate::context::RequestArgDeserializer>::deserialize(data)
    }
}