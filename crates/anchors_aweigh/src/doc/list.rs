use super::{DocFile, DocResult};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

pub type SharedDoc = Arc<DocFile>;
type InnerList = Arc<RwLock<HashMap<PathBuf, SharedDoc>>>;

/// collection of [SourceFile] that have been
/// parsed and are ready for analysis.
#[derive(Debug, Clone)]
pub struct DocList {
    root: PathBuf,
    files: InnerList,
}

#[derive(Debug, thiserror::Error)]
pub enum ListError {
    #[error("expected directory, got {0}")]
    RootIsFile(PathBuf),

    #[error("directory does not exist: {0}")]
    MissingDir(PathBuf),
}

impl DocList {
    /// Creates a doc list with the provided
    /// path directory as the "root" for relative
    /// file paths that are added later.  For this
    /// reason if the provided path is not absolute
    /// it will converted to an absolute path.
    pub fn new<P>(root: P) -> DocResult<Self>
    where
        P: AsRef<Path>,
    {
        let root = std::fs::canonicalize(root)?;

        if root.is_file() {
            Err(ListError::RootIsFile(root.clone()))?;
        }

        if !root.exists() {
            Err(ListError::MissingDir(root.clone()))?;
        }

        Ok(Self {
            root,
            files: InnerList::default(),
        })
    }

    /// Attempts to read the file path, parse it
    /// and fold it into the list.  If the path
    /// is relative it will be expanded to absolute.
    pub fn fetch<P>(&mut self, file_path: P) -> DocResult<SharedDoc>
    where
        P: AsRef<Path>,
    {
        let path_ref = file_path.as_ref();
        let path = if path_ref.is_relative() {
            std::fs::canonicalize(self.root.join(path_ref))?
        } else {
            std::fs::canonicalize(path_ref)?
        };

        if let Some(file) = self.files.read().unwrap().get(&path).cloned() {
            return Ok(file);
        }

        let file = SharedDoc::new(DocFile::parse_from_path(path.clone())?);
        self.files.write().unwrap().insert(path, file.clone());
        Ok(file)
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::support::fixtures;

    #[test]
    fn creating_a_new_list_works_with_dot() -> DocResult<()> {
        let list = DocList::new(".")?;
        assert_eq!(list.root(), &std::env::current_dir()?);
        Ok(())
    }

    #[test]
    fn creating_and_adding_a_file() -> DocResult<()> {
        let full_path = fixtures::sample_doc_filename();
        let file_dir = full_path.parent().unwrap().to_owned();
        let filename = full_path.file_name().unwrap().to_owned();
        let mut list = DocList::new(file_dir)?;
        let doc = list.fetch(filename)?;
        assert_eq!(doc.source, fixtures::sample_doc_contents());
        Ok(())
    }
}
