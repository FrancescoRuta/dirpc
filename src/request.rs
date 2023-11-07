#[derive(Clone)]
pub struct Request {
    pub conn_id: u32,
    pub data: bytes::Bytes,
}