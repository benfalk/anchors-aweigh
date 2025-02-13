//! Source Files
//!
//! Everything around reading files and parsing them
//!

mod error;
mod file;
mod lang;
mod list;
mod query;
mod range;

pub use error::SourceError;
pub use file::File;
pub use lang::Language;
pub use list::{SharedFile, SourceList};
pub use query::{Query, QueryError, QueryList};
pub use range::SourceRange;

type SourceResult<T> = Result<T, SourceError>;
