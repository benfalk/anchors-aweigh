//! Query List

use std::collections::HashMap;

use crate::source::Language;

use super::Query;

type LanguageQueries = HashMap<Language, NamedQueries>;
type NamedQueries = HashMap<String, Query>;

#[derive(Debug, Default)]
pub struct QueryList {
    queries: LanguageQueries,
}

impl QueryList {
    pub fn fetch(&self, lang: Language, name: &str) -> Option<&Query> {
        let queries = self.queries.get(&lang)?;
        queries.get(name)
    }

    pub fn register<T>(&mut self, name: T, query: Query)
    where
        T: Into<String>,
    {
        self.queries
            .entry(query.language())
            .or_default()
            .insert(name.into(), query);
    }
}
