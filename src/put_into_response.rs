use crate::response::Response;

pub trait PutIntoResponse {
    fn put_into_response(self, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
}

impl PutIntoResponse for () {
    fn put_into_response(self, _response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(())
    }
}