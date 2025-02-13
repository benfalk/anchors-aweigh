use super::{File, SourceResult};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

pub type SharedFile = Arc<File>;
type InnerList = Arc<RwLock<HashMap<PathBuf, SharedFile>>>;

#[derive(Debug, Default)]
pub struct SourceList {
    files: InnerList,
}

impl SourceList {
    pub fn fetch<T>(&self, path: T) -> SourceResult<SharedFile>
    where
        T: AsRef<Path>,
    {
        let path = std::fs::canonicalize(path)?;

        if let Some(file) = self.files.read().unwrap().get(&path).cloned() {
            return Ok(file);
        }

        let file = Arc::new(File::open(&path)?);
        self.files.write().unwrap().insert(path, file.clone());
        Ok(file)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::support::fixtures;

    #[test]
    fn fetch_works() -> SourceResult<()> {
        let list = SourceList::default();

        let file = list.fetch(fixtures::sample_ruby_filename())?;
        assert_eq!(file.contents, fixtures::sample_ruby_file_contents());
        assert_eq!(Arc::strong_count(&file), 2);

        let refetch = list.fetch(fixtures::sample_ruby_filename())?;
        assert_eq!(refetch.contents, fixtures::sample_ruby_file_contents());
        assert_eq!(Arc::strong_count(&file), 3);
        Ok(())
    }
}
