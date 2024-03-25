pub mod serialization {
    pub mod json;
    pub mod raw;
}
pub mod description;
pub mod inject;
mod base_types;
pub mod request_builder;
pub mod request;
pub mod server;

mod dyn_fn;
mod fake_variaddic;

pub use anyhow;

pub use dirpc_proc_macro::*;
pub use base_types::*;

mod test {
    use crate as dirpc;
    use dirpc_proc_macro::SerializeToBytes;

    #[derive(SerializeToBytes)]
    struct Test1;
    #[derive(SerializeToBytes)]
    struct Test2(String);
    #[derive(SerializeToBytes)]
    struct Test3(String, u8);
    #[derive(SerializeToBytes)]
    struct Test4 {}
    #[derive(SerializeToBytes)]
    struct Test5 { name: u32 }
    #[derive(SerializeToBytes)]
    struct Test6 { name: u32, acc: String, }
}
