//! **This crate is still in very early development**
//!
//! This package provide functions to parse and create lenex files.
//!
//! You can find more information about the official [Lenex Documentation](https://wiki.swimrankings.net/index.php/swimrankings:Lenex).

mod collection;
mod error;
mod file;
pub mod model;
mod serialization;

type Result<R> = std::result::Result<R, error::Error>;

pub use file::open_path;
