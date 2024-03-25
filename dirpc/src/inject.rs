use crate::{request::Request, base_types::DeserializeFromBytes};

pub trait Inject<Context, RequestState> where Self: Sized {
    fn inject(ctx: &Context, request: &mut Request<RequestState>) -> anyhow::Result<Self>;
}

impl<Context, RequestState, T> Inject<Context, RequestState> for T
where
    T: DeserializeFromBytes,
{
    fn inject(_ctx: &Context, request: &mut Request<RequestState>) -> anyhow::Result<Self> {
        T::deserialize_from_bytes(&mut request.data)
    }
}