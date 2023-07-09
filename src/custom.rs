#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("NoSubject: {0}")]
    NoSubject(String),

    #[error("NoBody: {0}")]
    NoBody(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
pub type Result<T> = core::result::Result<T, Error>;