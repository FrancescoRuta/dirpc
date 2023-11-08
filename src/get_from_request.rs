use crate::{inject::Inject, request::Request, server::TypeDescription, get_type_description::GetTypeDescription, type_encoding::TypeEncoding};

pub trait GetFromRequest where Self: Sized {
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>>;
}

impl<'ctx, Context, T, Inner> Inject<'ctx, Context> for T
where
    T: TypeEncoding<EncodedType = Inner>,
    Inner: GetTypeDescription,
{
    fn get_type_description() -> Option<TypeDescription> {
        Some(<T as GetTypeDescription>::get_type_description())
    }
    fn inject(_ctx: &'ctx Context, request: &mut crate::request::Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        T::get_from_request(request)
    }
}