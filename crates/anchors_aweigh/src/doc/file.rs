use super::{Anchor, DocResult, Token};
use std::path::PathBuf;

/// Represents a file that has been parsed into
/// tokens and is ready for processing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocFile {
    /// location of the source file on disk
    pub path: PathBuf,

    /// raw original contents of the source file
    pub source: String,

    /// all of the parsed tokens in the order they
    /// where found in the source
    pub ast: Vec<Token>,
}

impl DocFile {
    pub fn with_path_and_source<T, S>(path: T, source: S) -> DocResult<Self>
    where
        T: Into<PathBuf>,
        S: Into<String>,
    {
        let source = source.into();
        let path: PathBuf = path.into();
        let mut ast = Token::parse_tokens(&source)?;

        // TODO: Need a better and easier to understand
        //       method of building these anchors
        for token in &mut ast {
            if let Token::RawAnchor(data) = token {
                let mut anchor = Anchor::parse(data)?;
                if anchor.link.path.is_relative() {
                    let root = path.parent().unwrap();
                    anchor.link.path = root.join(anchor.link.path);
                }
                *token = Token::Anchor(anchor);
            }
        }
        Ok(Self { path, source, ast })
    }

    pub(super) fn parse_from_path(path: PathBuf) -> DocResult<Self> {
        let source = std::fs::read_to_string(&path)?;
        Self::with_path_and_source(path, source)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        doc::{Decoration, Link, Strategy},
        support::fixtures,
    };

    #[test]
    fn parse_from_path_works() -> DocResult<()> {
        let doc_file = fixtures::sample_doc_filename();
        let file = DocFile::parse_from_path(doc_file.clone())?;
        assert_eq!(file.path, doc_file);
        assert_eq!(file.source, fixtures::sample_doc_contents());
        assert_eq!(
            file.ast,
            vec![
                Token::Content("# Sample Doc\n\n```ruby\n".to_owned()),
                Token::Anchor(Anchor {
                    decoration: Decoration::None,
                    link: Link {
                        path: fixtures::sample_ruby_filename(),
                        strategy: Strategy::Full,
                    }
                }),
                Token::Content("\n```\n".to_owned()),
            ]
        );
        Ok(())
    }
}
