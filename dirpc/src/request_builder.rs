use std::marker::PhantomData;

use serde::Serialize;

use crate::{base_types::SerializationHelper, for_all_tuples, rpc_serde::RpcSerializer};

pub struct RequestBuilder<Serializer: RpcSerializer> {
    res: SerializationHelper,
    _phantom_data: PhantomData<Serializer>,
}

pub trait IntoFunctionArgs<Serializer> {
    fn append(self, v: &mut Vec<bytes::Bytes>) -> anyhow::Result<()>;
}

impl<Serialize> IntoFunctionArgs<Serialize> for () {
    fn append(self, _: &mut Vec<bytes::Bytes>) -> anyhow::Result<()> {
        Ok(())
    }
}

macro_rules! count {
    () => {
        0
    };
    ($t0:ident $( $t:ident )*) => {
        1 + count!($( $t )*)
    };
}

macro_rules! impl_for_tuple {
    ( $( $t:ident $t_idx:tt; )* ) => {
        impl<Serializer, $( $t, )*> IntoFunctionArgs<Serializer> for ($( $t, )*)
        where
            Serializer: RpcSerializer,
            $($t: Serialize),*
        {
            #[allow(non_snake_case)]
            fn append(self, v: &mut Vec<bytes::Bytes>) -> anyhow::Result<()> {
                v.reserve(count!($( $t )*));
                let ($( $t, )*) = self;
                $( let $t = Serializer::serialize_unfallible($t)?; )*
                $(
                    v.push(bytes::Bytes::copy_from_slice(&u32::to_be_bytes($t.len() as u32)));
                    v.push($t);
                )*
                Ok(())
            }
        }
    };
}

for_all_tuples!(impl_for_tuple);

impl<Serializer: RpcSerializer> RequestBuilder<Serializer> {
    pub fn new() -> Self {
        Self {
            res: SerializationHelper::new(),
            _phantom_data: PhantomData,
        }
    }
    pub fn push_call(&mut self, index: u32, args: impl IntoFunctionArgs<Serializer>) -> anyhow::Result<()> {
        let mut buffer = bytes::BytesMut::with_capacity(8);
        bytes::BufMut::put_u32(&mut buffer, index);
        let last_index = self.res.chain.len();
        self.res.chain.push(bytes::Bytes::from_static(&[]));
        args.append(&mut self.res.chain)?;
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