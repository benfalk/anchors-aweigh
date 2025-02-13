use super::{Query, QueryError, SourceResult};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tree_sitter::Tree;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Language {
    Ruby,
    Rust,
}

impl Language {
    pub fn determine_from_path<T>(path: T) -> Option<Self>
    where
        T: AsRef<Path>,
    {
        let ext = path.as_ref().extension()?;
        match ext.to_str()? {
            "rb" => Some(Self::Ruby),
            "rs" => Some(Self::Rust),
            _ => None,
        }
    }

    pub fn build_query<T>(&self, template: T) -> Result<Query, QueryError>
    where
        T: AsRef<str>,
    {
        Query::new(*self, template)
    }

    pub fn parse(&self, source: &str) -> SourceResult<Option<Tree>> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter::Language::from(*self))?;
        Ok(parser.parse(source, None))
    }
}

impl From<Language> for tree_sitter::Language {
    fn from(value: Language) -> Self {
        match value {
            Language::Rust => tree_sitter_rust::LANGUAGE.into(),
            Language::Ruby => tree_sitter_ruby::LANGUAGE.into(),
        }
    }
}
