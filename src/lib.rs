pub mod serialization {
    pub mod json;
    pub mod raw;
}
pub mod description;
pub mod inject;
pub mod io_bytes;
pub mod request_builder;
pub mod request;
pub mod server;

mod dyn_fn;
mod fake_variaddic;