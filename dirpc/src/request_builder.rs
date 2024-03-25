use crate::base_types::{SerializeToBytes, SerializationHelper};

pub struct RequestBuilder {
    res: SerializationHelper,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self {
            res: SerializationHelper::new(),
        }
    }
    pub fn push_call(&mut self, index: u32, args: impl SerializeToBytes) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut buffer = bytes::BytesMut::with_capacity(8);
        bytes::BufMut::put_u32(&mut buffer, index);
        let last_index = self.res.chain.len();
        self.res.chain.push(bytes::Bytes::from_static(&[]));
        args.serialize_to_bytes(&mut self.res)?;
        let size = self.res.chain[last_index..].iter().map(|b| b.len() as u32).sum();
        bytes::BufMut::put_u32(&mut buffer, size);
        self.res.chain[last_index] = buffer.freeze();
        Ok(())
    }
    pub fn into_chunks(self) -> Vec<bytes::Bytes> {
        self.res.chain
    }
    pub fn build_request(&self) -> bytes::Bytes {
        let size = self.res.chain.iter().map(|b| b.len()).sum();
        let mut buffer = bytes::BytesMut::with_capacity(size);
        self.res.chain.iter().for_each(|b| bytes::BufMut::put(&mut buffer, &**b));
        buffer.freeze()
    }
}