use anchors_aweigh::{doc::DocList, linker::Linker, source::Language};
use anyhow::{Context, Result};
use std::path::PathBuf;

fn main() -> Result<()> {
    let file = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .with_context(|| "requires filename")?;

    let mut linker = Linker::default();

    linker.queries.register(
        "class",
        Language::Ruby.build_query(
            r#"
            (
                (comment)*
                .
                (class name: (constant) @name (#eq? @name "{{name}}"))
            ) @match
        "#,
        )?,
    );

    let mut list = DocList::new(".")?;
    let doc = list.fetch(file)?;
    let assembly = linker.build_assembly(&doc)?;
    println!("{}", assembly.compile());
    Ok(())
}
