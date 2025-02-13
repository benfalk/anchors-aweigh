//! # Doc
//!
//! This module addresses the responsibilities of reading
//! and parsing markdown files.
//!

mod anchor;
mod file;
mod list;
mod token;

pub use anchor::{
    Anchor, Decoration, Link, NamedAnchor, ParseQueryAnchorError, QueryAnchor, Strategy,
};
pub use file::DocFile;
pub use list::DocList;
pub use token::Token;

/// All of the variants of errors that can be encountered
/// when working with the logical concepts in [crate::source]
#[derive(Debug, thiserror::Error)]
pub enum DocError {
    #[error(transparent)]
    ParseToken(#[from] token::ParseError),

    #[error(transparent)]
    ParseAnchor(#[from] anchor::ParseError),

    #[error(transparent)]
    ParseQueryAnchor(#[from] ParseQueryAnchorError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ListError(#[from] list::ListError),
}

type DocResult<T> = Result<T, DocError>;
