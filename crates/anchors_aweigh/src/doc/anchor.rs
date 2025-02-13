//! # Anchor
//!
//! Provides resources and logic around how portions of files
//! are targeted for import into your documentation.
//!

use serde::{Deserialize, Serialize};
use std::path::Path;

mod decoration;
mod link;
mod named_anchor;
mod parser;
mod query_anchor;
mod strategy;

pub use decoration::Decoration;
pub use link::Link;
pub use named_anchor::NamedAnchor;
pub use parser::ParseError;
pub use query_anchor::{ParseQueryAnchorError, QueryAnchor};
pub use strategy::Strategy;

use super::DocError;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Anchor {
    pub link: Link,
    pub decoration: Decoration,
}

impl Anchor {
    pub fn parse(source: &str) -> Result<Self, DocError> {
        Ok(parser::parse(source)?)
    }
}

impl AsRef<Path> for &Anchor {
    fn as_ref(&self) -> &Path {
        &self.link.path
    }
}
