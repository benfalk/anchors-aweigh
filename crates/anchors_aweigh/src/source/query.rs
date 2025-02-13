use super::{File, Language, SourceRange};
use handlebars::Handlebars;
use serde::Serialize;
use tree_sitter::Language as TSLanguage;
use tree_sitter::Query as TreesitterQuery;
use tree_sitter::{QueryCursor, StreamingIterator};

mod list;
pub use list::QueryList;

#[derive(Debug)]
pub struct Query {
    lang: Language,
    ts_lang: TSLanguage,
    query: Handlebars<'static>,
}

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error(transparent)]
    Template(#[from] handlebars::TemplateError),

    #[error(transparent)]
    Render(#[from] handlebars::RenderError),

    #[error(transparent)]
    Query(#[from] tree_sitter::QueryError),

    #[error(transparent)]
    Language(#[from] tree_sitter::LanguageError),

    #[error("could not parse source")]
    NoSource,
}

type QueryResult<T> = Result<T, QueryError>;

impl Query {
    pub fn new<T>(lang: Language, template: T) -> QueryResult<Self>
    where
        T: AsRef<str>,
    {
        let mut query = Handlebars::new();
        query.register_template_string("query", template)?;
        Ok(Self {
            ts_lang: lang.into(),
            lang,
            query,
        })
    }

    pub fn language(&self) -> Language {
        self.lang
    }

    pub fn find<T>(&self, source: &File, bindings: &T) -> QueryResult<Option<SourceRange>>
    where
        T: Serialize,
    {
        if source.tree.is_none() {
            return Ok(None);
        }

        let raw = self.query.render("query", bindings)?;
        let query = TreesitterQuery::new(&self.ts_lang, &raw)?;
        let tree = source.tree.as_ref().unwrap();
        let mut cursor = QueryCursor::new();

        Ok(cursor
            .matches(&query, tree.root_node(), source.contents.as_bytes())
            .next()
            .map(SourceRange::from))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::source::Language;
    use crate::support::fixtures;
    use serde_json::json;

    #[test]
    fn it_works() -> QueryResult<()> {
        let ruby_code = &File::open(fixtures::sample_ruby_filename()).unwrap();
        let by_class = Query::new(
            Language::Ruby,
            r#"
                (
                    (comment)*
                    .
                    (class name: (constant) @name (#eq? @name "{{name}}"))
                ) @match
            "#,
        )?;

        let opt = by_class.find(ruby_code, &json!({"name": "Foo"}))?;
        assert!(matches!(opt, Some(SourceRange { start: 6, end: 18 })));

        let opt = by_class.find(ruby_code, &json!({"name": "Bar"}))?;
        assert!(matches!(opt, Some(SourceRange { start: 20, end: 28 })));

        let opt = by_class.find(ruby_code, &json!({"name": "Rofl"}))?;
        assert!(opt.is_none());

        Ok(())
    }
}
