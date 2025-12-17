use ::anchors_aweigh::{doc::DocFile, linker::Linker};
use ::anyhow::{Context, Result};
use ::mdbook_preprocessor::book::{Book, BookItem, Chapter};
use ::mdbook_preprocessor::{Preprocessor, PreprocessorContext};

#[derive(Debug)]
pub struct AnchorsAweighLinker;

impl Preprocessor for AnchorsAweighLinker {
    fn name(&self) -> &str {
        "Anchors Aweigh Linker"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        ::log::info!("{} Starting", self.name());

        let linker = crate::build_linker(ctx)?;
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                match link_anchors(ctx, chapter, &linker) {
                    Ok(updated) => {
                        chapter.content = updated;
                    }
                    Err(error) => {
                        log::error!("{} {:?}", error, error.source());
                    }
                }
            }
        });

        ::log::info!("{} Finished", self.name());
        Ok(book)
    }
}

fn link_anchors(
    ctx: &PreprocessorContext,
    chapter: &mut Chapter,
    linker: &Linker,
) -> Result<String> {
    let path = chapter.source_path.clone().context("missing source path")?;
    let path = ctx.config.book.src.join(path);

    let docfile = DocFile::with_path_and_source(path.clone(), &chapter.content)?;

    let assembly = linker
        .build_assembly(&docfile)
        .with_context(|| format!("linking {path:?}"))?;
    Ok(assembly.compile(linker))
}
