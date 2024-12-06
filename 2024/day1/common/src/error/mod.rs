mod result;

use std::num::ParseIntError;

pub use result::Result;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("input line '{0}' missing left location ID")]
    InputLineMissingLeftLocationId(String),

    #[error("failed parsing left location ID to int")]
    ParseLeftLocationId(#[source] ParseIntError),

    #[error("input line '{0}' missing right location ID")]
    InputLineMissingRightLocationId(String),

    #[error("failed parsing right location ID to int")]
    ParseRightLocationId(#[source] ParseIntError),
}
