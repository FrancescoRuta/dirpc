#[derive(Debug)]
pub struct Response {
    pub data: Vec<bytes::Bytes>,
}

impl Response {
    
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(32),
        }
    }
    
}