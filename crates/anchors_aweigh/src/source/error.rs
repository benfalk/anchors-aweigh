#[derive(Debug, thiserror::Error)]
pub enum SourceError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Language(#[from] tree_sitter::LanguageError),
}
