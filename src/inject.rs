use crate::{request::Request, server::TypeDescription};

pub trait Inject<'ctx, Context> where Self: Sized {
    fn get_type_description() -> Option<TypeDescription> { None }
    fn inject(ctx: &'ctx Context, request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>>;
}