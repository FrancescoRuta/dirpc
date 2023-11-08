pub struct Request<State> {
    pub state: State,
    pub data: bytes::Bytes,
}

impl<State> Request<State> {
    pub fn new(state: State, data: bytes::Bytes) -> Self {
        Self {
            state,
            data,
        }
    }
}
