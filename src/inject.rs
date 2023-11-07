use crate::request::Request;

pub trait Inject<'ctx, Context> where Self: Sized {
    fn inject(ctx: &'ctx Context, request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>>;
}