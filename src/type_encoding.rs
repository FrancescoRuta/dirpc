use crate::request::Request;

pub trait TypeEncoding where Self: Sized {
    type EncodedType;
    const NAME: &'static str;
    fn get_from_request(request: &mut Request) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>>;
}