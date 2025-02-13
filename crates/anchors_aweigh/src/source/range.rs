use super::File;
use tree_sitter::QueryMatch;

#[derive(Debug, Clone, Copy, Default)]
pub struct SourceRange {
    pub start: usize,
    pub end: usize,
}

impl SourceRange {
    pub fn fetch_lines(&self, source: &File) -> String {
        let Self { start, end } = self;
        let mut string = String::new();
        let lines = source.contents.lines();
        for line in lines.skip(start - 1).take(end - start + 1) {
            string.push_str(line);
            string.push('\n');
        }
        string.pop();
        string
    }
}

impl From<&QueryMatch<'_, '_>> for SourceRange {
    fn from(value: &QueryMatch<'_, '_>) -> Self {
        let mut range = Self::default();
        if value.captures.is_empty() {
            return range;
        }

        let node = value.captures.first().unwrap().node;
        range.start = node.start_position().row;
        range.end = node.end_position().row;

        value.captures[1..].iter().for_each(|capture| {
            let row = capture.node.end_position().row;
            if range.end < row {
                range.end = row;
            }
        });

        range.start += 1;
        range.end += 1;
        range
    }
}
