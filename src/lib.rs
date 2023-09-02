pub mod chunk_type;
pub mod chunk;
pub mod png;
pub mod args;
pub mod error;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;