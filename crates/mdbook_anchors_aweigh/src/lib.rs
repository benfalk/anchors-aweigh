use ::anchors_aweigh::{linker::Linker, source::Language};
use ::anyhow::{Result, bail};
use ::mdbook::preprocess::PreprocessorContext;

mod config;
mod preprocessor;

pub use config::Config;
pub use preprocessor::AnchorsAweighLinker;

pub fn build_linker(ctx: &PreprocessorContext) -> Result<Linker> {
    ::log::debug!("building linker");
    let mut linker = Linker::default();
    let config = Config::try_from(ctx)?;

    config.try_each_table("queries", |lang, queries| {
        let language = match lang {
            "ruby" => Language::Ruby,
            "rust" => Language::Rust,
            "toml" => Language::Toml,
            "javascript" => Language::Javascript,
            "elixir" => Language::Elixir,
            "json" => Language::Json,
            "markdown" => Language::Markdown,
            not_supported => {
                bail!("[{not_supported}] is not supported.")
            }
        };

        for (name, query_tpl) in queries {
            if let Some(tpl) = query_tpl.as_str() {
                match language.build_query(tpl) {
                    Ok(query) => {
                        linker.queries.register(name, query);
                        ::log::trace!("loaded {}.{}", lang, name);
                    }
                    Err(error) => {
                        ::log::error!("loading {}.{}: {}", lang, name, error);
                    }
                }
            } else {
                ::log::warn!(
                    "[preprocessor.anchors-aweigh.queries.{}.{}] is not a string",
                    lang,
                    name
                );
            }
        }

        Ok(())
    });

    config.try_each_string("templates", |name, template| {
        linker.templates.create(name, template)?;
        ::log::trace!("registered template [{name}]");
        Ok(())
    });

    ::log::debug!("linker built");
    Ok(linker)
}
