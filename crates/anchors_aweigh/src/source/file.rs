use super::{Language, SourceResult};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tree_sitter::Tree;

/// # Source File
///
/// Represents a source file from a project.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub path: PathBuf,
    pub contents: String,
    pub language: Option<Language>,
    #[serde(skip)]
    pub tree: Option<Tree>,
}

impl File {
    pub fn open<T>(path: T) -> SourceResult<Self>
    where
        T: AsRef<Path>,
    {
        let path = std::fs::canonicalize(path)?;
        let contents = std::fs::read_to_string(&path)?;
        let language = Language::determine_from_path(&path);
        let mut file = Self {
            path,
            contents,
            language,
            tree: None,
        };
        file.recalculate_tree()?;
        Ok(file)
    }

    pub fn recalculate_tree(&mut self) -> SourceResult<()> {
        if let Some(lang) = self.language {
            self.tree = lang.parse(&self.contents)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::support::fixtures;

    #[test]
    fn open_works() -> SourceResult<()> {
        let file = File::open(fixtures::sample_ruby_filename())?;
        assert_eq!(file.contents, fixtures::sample_ruby_file_contents());
        Ok(())
    }
}
