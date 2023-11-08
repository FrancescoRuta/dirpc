use crate::io_bytes::{SerializeToBytes, SerializationHelper};

pub struct RequestBuilder {
    res: Vec<(u32, SerializationHelper)>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self {
            res: vec![(0, SerializationHelper::new())],
        }
    }
    pub fn push_call(&mut self, index: u32, args: impl SerializeToBytes) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let current = {
            let last_index = self.res.len() - 1;
            let mut last = &mut self.res[last_index];
            if last.0 == 16 {
                self.res.push((0, SerializationHelper::new()));
                last = &mut self.res[last_index + 1];
            }
            last
        };
        let mut buffer = bytes::BytesMut::with_capacity(8);
        bytes::BufMut::put_u32(&mut buffer, index);
        current.0 += 1;
        let last_index = current.1.chain.len();
        current.1.chain.push(bytes::Bytes::from_static(&[]));
        args.serialize_to_bytes(&mut current.1)?;
        let size = current.1.chain[last_index..].iter().map(|b| b.len() as u32).sum();
        bytes::BufMut::put_u32(&mut buffer, size);
        current.1.chain[last_index] = buffer.freeze();
        Ok(())
    }
    pub fn into_request(&self) -> Vec<bytes::Bytes> {
        let mut result = Vec::with_capacity(self.res.len());
        for (_, req) in &self.res {
            let size = req.chain.iter().map(|b| b.len()).sum();
            let mut buffer = bytes::BytesMut::with_capacity(size);
            req.chain.iter().for_each(|b| bytes::BufMut::put(&mut buffer, &**b));
            result.push(buffer.freeze());
        }
        result
    }
}