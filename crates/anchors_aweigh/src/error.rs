#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Document(#[from] crate::doc::DocError),

    #[error(transparent)]
    Source(#[from] crate::source::SourceError),
}

pub type Result<T> = std::result::Result<T, Error>;
