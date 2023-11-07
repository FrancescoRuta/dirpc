use crate::{inject::Inject, request::Request};

pub trait GetFromRequest where Self: Sized {
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>>;
}

impl<'ctx, Context, T> Inject<'ctx, Context> for T
where
    T: GetFromRequest
{
    fn inject(_ctx: &'ctx Context, request: &mut crate::request::Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        T::get_from_request(request)
    }
}