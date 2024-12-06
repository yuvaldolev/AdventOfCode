mod result;

use std::{env::VarError, io};

pub use result::Result;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed opening file at path '{1}'")]
    OpenFile(#[source] io::Error, String),

    #[error("failed reading file '{1}' to string")]
    ReadFileToString(#[source] io::Error, String),

    #[error("failed downloading puzzle input from URL '{1}'")]
    DownloadPuzzleInput(#[source] reqwest::Error, String),

    #[error("couldn't decode puzzle input text")]
    DecodePuzzleInputText(#[source] reqwest::Error),

    #[error("couldn't retrieve Advent of Code session token from environment variable '{1}'")]
    RetrieveSessionToken(#[source] VarError, String),
}
