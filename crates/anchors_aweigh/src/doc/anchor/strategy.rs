use super::{NamedAnchor, QueryAnchor};
use crate::{linker::Linker, source::File};
use serde::{Deserialize, Serialize};

/// Describes how to select lines from a file for the
/// subject of analysis and rendering
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Strategy {
    /// OG style of anchors provided supported by
    /// mdbook's include link tag.
    Named(NamedAnchor),

    /// the entire file should be selected, indicated
    /// by the absense of a `:` or `#` following the path
    Full,

    /// the start and ending lines in a file to include,
    /// indicated by `:<number>:<number>`
    Between { start: usize, end: usize },

    /// this line and all others till the end of the file,
    /// indicated by `:<number>:`
    HereDown(usize),

    /// from the first line of the file down to this line,
    /// indiciated by `::<number>`
    DownTo(usize),

    /// include only this line of the file, indicated by
    /// `:<number>`
    ThisLine(usize),

    /// anchor leading with `#` that describes
    /// which treesitter query to use when looking
    /// for lines to select
    Query(QueryAnchor),
}

impl Strategy {
    pub fn find_content(&self, source: &File, linker: &Linker) -> Option<String> {
        match self {
            Self::Full => Some(source.contents.clone()),
            Self::Between { start, end } => {
                let mut string = String::new();
                let lines = source.contents.lines();
                for line in lines.skip(start - 1).take(end - start + 1) {
                    string.push_str(line);
                    string.push('\n');
                }
                string.pop();
                Some(string)
            }
            Self::HereDown(line) => {
                let mut string = String::new();
                let lines = source.contents.lines();
                for line in lines.skip(line - 1) {
                    string.push_str(line);
                    string.push('\n');
                }
                string.pop();
                Some(string)
            }
            Self::DownTo(line) => {
                let mut string = String::new();
                let lines = source.contents.lines();
                for line in lines.take(*line) {
                    string.push_str(line);
                    string.push('\n');
                }
                string.pop();
                Some(string)
            }
            Self::ThisLine(line) => {
                let mut lines = source.contents.lines();
                lines.nth(line - 1).map(ToOwned::to_owned)
            }
            Self::Named(anchor) => {
                let mut string = String::new();
                let lines = source.contents.lines();
                let filtered = lines
                    .skip_while(|line| anchor.missing_start(line))
                    .skip(1)
                    .take_while(|line| anchor.missing_end(line));
                for line in filtered {
                    string.push_str(line);
                    string.push('\n');
                }
                string.pop();
                Some(string)
            }
            Self::Query(anchor) => {
                let query = linker.queries.fetch(source.language?, &anchor.name)?;
                let range = query.find(source, &anchor.bindings).unwrap()?;
                Some(range.fetch_lines(source))
            }
        }
    }
}
