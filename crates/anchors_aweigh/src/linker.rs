//! # Linker
//!
//! Responsible for linking documents with the
//! files to which they refer to.
//!

mod assembly;
mod linkage;

pub use assembly::{Assembly, Node};
pub use linkage::Linkage;

use crate::doc::{DocFile, Token};
use crate::error::Result;
use crate::source::{QueryList, SourceList};

#[derive(Debug, Default)]
pub struct Linker {
    pub sources: SourceList,
    pub queries: QueryList,
}

impl Linker {
    pub fn build_assembly(&self, doc: &DocFile) -> Result<Assembly> {
        let mut nodes = Vec::with_capacity(doc.ast.len());
        for token in doc.ast.iter() {
            nodes.push(match token {
                Token::Content(text) => Node::Text(text.clone()),
                Token::Anchor(anchor) => {
                    let source = self.sources.fetch(anchor)?;
                    let contents = anchor.link.strategy.find_content(&source, self);
                    Node::Link(Linkage {
                        source,
                        contents,
                        strategy: anchor.link.strategy.clone(),
                        decoration: anchor.decoration.clone(),
                    })
                }
                Token::RawAnchor(text) => Node::Text(format!("{{#aa {text} }}")),
            });
        }
        Ok(Assembly { nodes })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::doc::DocList;
    use crate::support::fixtures;

    #[test]
    fn it_works() {
        let linker = Linker::default();
        let full_path = fixtures::sample_doc_filename();
        let file_dir = full_path.parent().unwrap().to_owned();
        let filename = full_path.file_name().unwrap().to_owned();
        let mut list = DocList::new(file_dir).unwrap();
        let doc = list.fetch(filename).unwrap();
        let assembly = linker.build_assembly(&doc).unwrap();
        assert!(matches!(assembly.nodes[0], Node::Text(_)));
        assert!(matches!(assembly.nodes[1], Node::Link(_)));
        assert!(matches!(assembly.nodes[2], Node::Text(_)));
    }
}
