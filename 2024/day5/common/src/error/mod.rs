mod result;

use std::num::ParseIntError;

pub use result::Result;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("parse ordering rule '{0}' format is invalid")]
    InvalidPageOrderingRuleFormat(String),

    #[error("failed parsing page '{1}' number to int")]
    ParsePageNumber(#[source] ParseIntError, String),

    #[error("failed computing page ordering according to rules")]
    ComputePageOrdering,
}
