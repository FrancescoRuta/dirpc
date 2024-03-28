use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};

pub trait ServerContext {
    type Serializer: ResponseSerializer;
    type Deserializer: RequestArgDeserializer;
}

pub trait ResponseSerializer {
    fn serialize<T: Serialize>(data: T) -> anyhow::Result<Bytes>;
}

pub trait RequestArgDeserializer {
    fn deserialize<T: DeserializeOwned>(data: Bytes) -> anyhow::Result<T>;
}