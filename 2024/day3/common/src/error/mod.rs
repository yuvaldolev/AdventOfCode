mod result;

pub use result::Result;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed parsing input '{1}'")]
    ParseInput(#[source] nom::Err<nom::error::Error<String>>, String),
}
