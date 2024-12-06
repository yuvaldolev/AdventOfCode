use std::env;

use reqwest::{blocking::Client, header};

use crate::error;

const SESSION_TOKEN_ENVIRONMENT_VARIABLE: &str = "SESSION_TOKEN";

pub struct PuzzleInputDownloader {
    client: Client,
    cookie: String,
}

impl PuzzleInputDownloader {
    pub fn new() -> error::Result<Self> {
        Ok(Self {
            client: Client::new(),
            cookie: format!("session={}", Self::retrieve_session_token()?),
        })
    }

    pub fn download(&self, year: u16, day: u8) -> error::Result<String> {
        let download_url = format!("https://adventofcode.com/{year}/day/{day}/input");

        self.client
            .get(&download_url)
            .header(header::COOKIE, &self.cookie)
            .send()
            .map_err(|e| error::Error::DownloadPuzzleInput(e, download_url.clone()))?
            .error_for_status()
            .map_err(|e| error::Error::DownloadPuzzleInput(e, download_url))?
            .text()
            .map_err(error::Error::DecodePuzzleInputText)
    }

    fn retrieve_session_token() -> error::Result<String> {
        env::var(SESSION_TOKEN_ENVIRONMENT_VARIABLE).map_err(|e| {
            error::Error::RetrieveSessionToken(e, SESSION_TOKEN_ENVIRONMENT_VARIABLE.to_owned())
        })
    }
}
