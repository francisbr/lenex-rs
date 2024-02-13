mod collection;
mod error;
mod file;
pub mod model;
mod serialization;

pub type Result<R> = std::result::Result<R, error::Error>;

pub use file::open_path;
