use ::anchors_aweigh::{linker::Linker, source::Language};
use ::anyhow::{bail, Context, Result};
use ::mdbook::preprocess::PreprocessorContext;

mod preprocessor;
pub use preprocessor::AnchorsAweighLinker;

pub fn build_linker(ctx: &PreprocessorContext) -> Result<Linker> {
    ::log::debug!("building linker");
    let mut linker = Linker::default();

    let config = ctx
        .config
        .get_preprocessor("anchors-aweigh")
        .context("[preprocessor.anchors-aweigh] config missing")?;

    let queries = config
        .get("queries")
        .and_then(toml::Value::as_table)
        .cloned()
        .unwrap_or_default();

    for lang_queries in queries {
        if let Err(error) = load_linker(&mut linker, lang_queries) {
            ::log::error!("loading lang set: {error}");
        }
    }

    ::log::debug!("linker built");
    Ok(linker)
}

fn load_linker(linker: &mut Linker, (lang, list): (String, toml::Value)) -> Result<()> {
    let language = match lang.as_str() {
        "ruby" => Language::Ruby,
        "rust" => Language::Rust,
        not_supported => {
            bail!("[{not_supported}] is not supported.")
        }
    };

    let queries = list.as_table().with_context(|| {
        format!(
            "[preprocessor.anchors-aweigh.queries.{}] is not a table",
            lang.as_str()
        )
    })?;

    for (name, query_tpl) in queries {
        if let Some(tpl) = query_tpl.as_str() {
            match language.build_query(tpl) {
                Ok(query) => {
                    linker.queries.register(name, query);
                    ::log::trace!("loaded {}.{}", lang.as_str(), name);
                }
                Err(error) => {
                    ::log::error!("loading {}.{}: {}", lang.as_str(), name, error);
                }
            }
        } else {
            ::log::warn!(
                "[preprocessor.anchors-aweigh.queries.{}.{}] is not a string",
                lang.as_str(),
                name
            );
        }
    }

    Ok(())
}
