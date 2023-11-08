use crate::{request::Request, io_bytes::DeserializeFromBytes};

pub trait Inject<'ctx, Context, RequestState> where Self: Sized {
    fn inject(ctx: &'ctx Context, request: &mut Request<RequestState>) -> anyhow::Result<Self>;
}

impl<'ctx, Context, RequestState, T> Inject<'ctx, Context, RequestState> for T
where
    T: DeserializeFromBytes,
{
    fn inject(_ctx: &'ctx Context, request: &mut Request<RequestState>) -> anyhow::Result<Self> {
        T::deserialize_from_bytes(&mut request.data)
    }
}