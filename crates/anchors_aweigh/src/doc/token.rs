mod parser;

use super::Anchor;
pub use parser::ParseError;

/// Each chunk parsed is either just raw content
/// or is a link that needs further processing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// text from the markdown that was not found
    /// to contain any notable tokens to analyze
    Content(String),

    /// raw contents of a `{{#aa ...}}` tag
    RawAnchor(String),

    /// fully parsed anchor, ready for action
    Anchor(Anchor),
}

impl Token {
    pub(super) fn parse_tokens(source: &str) -> Result<Vec<Self>, ParseError> {
        parser::parse(source)
    }
}
