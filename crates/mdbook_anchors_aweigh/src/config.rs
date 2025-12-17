use ::anyhow::{Context, Result};
use ::mdbook_preprocessor::PreprocessorContext;
use ::toml::{Value, map::Map};

pub struct Config {
    toml: Map<String, Value>,
}

impl Config {
    pub fn table(&self, key: &str) -> Map<String, Value> {
        self.toml
            .get(key)
            .and_then(Value::as_table)
            .cloned()
            .unwrap_or_default()
    }

    pub fn try_each_table<F>(&self, key: &str, mut update: F)
    where
        F: FnMut(&str, &Map<String, Value>) -> Result<()>,
    {
        for (ref table_key, value) in self.table(key) {
            if let Some(table) = value.as_table() {
                if let Err(error) = update(table_key, table) {
                    ::log::error!(
                        "[preprocessor.anchors-aweigh.{}.{}] [{error}]",
                        key,
                        table_key
                    );
                }
            } else {
                ::log::error!(
                    "[preprocessor.anchors-aweigh.{}.{}] is not a table",
                    key,
                    table_key
                );
            }
        }
    }

    pub fn try_each_string<F>(&self, key: &str, mut update: F)
    where
        F: FnMut(&str, &str) -> Result<()>,
    {
        for (ref table_key, value) in self.table(key) {
            if let Some(table) = value.as_str() {
                if let Err(error) = update(table_key, table) {
                    ::log::error!(
                        "[preprocessor.anchors-aweigh.{}.{}] [{error}]",
                        key,
                        table_key
                    );
                }
            } else {
                ::log::error!(
                    "[preprocessor.anchors-aweigh.{}.{}] is not a string",
                    key,
                    table_key
                );
            }
        }
    }
}

impl TryFrom<&PreprocessorContext> for Config {
    type Error = anyhow::Error;

    fn try_from(ctx: &PreprocessorContext) -> Result<Self> {
        let toml = ctx
            .config
            .get("preprocessor.anchors-aweigh")
            .context("error fetching preprocessor config")?
            .context("missing [preprocessor.anchors-aweigh")?;
        Ok(Self { toml })
    }
}
