use super::{Query, QueryError, SourceResult};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tree_sitter::Tree;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Language {
    Ruby,
    Rust,
    Toml,
    Javascript,
    Elixir,
    Json,
    Markdown,
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
            "toml" => Some(Self::Toml),
            "js" => Some(Self::Javascript),
            "ex" | "exs" => Some(Self::Elixir),
            "json" => Some(Language::Json),
            "md" | "txt" => Some(Language::Markdown),
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
            Language::Toml => tree_sitter_toml_ng::LANGUAGE.into(),
            Language::Javascript => tree_sitter_javascript::LANGUAGE.into(),
            Language::Elixir => tree_sitter_elixir::LANGUAGE.into(),
            Language::Json => tree_sitter_json::LANGUAGE.into(),
            Language::Markdown => tree_sitter_md::LANGUAGE.into(),
        }
    }
}
