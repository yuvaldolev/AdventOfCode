mod result;

use std::num::ParseIntError;

pub use result::Result;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed parsing level '{1}' to int")]
    ParseLevel(#[source] ParseIntError, String),
}
