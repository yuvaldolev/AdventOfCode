use std::{fs::File, io::Read, path::Path};

use crate::error;

pub struct FileReader;

impl FileReader {
    pub fn new() -> Self {
        Self
    }

    pub fn read_to_string(path: impl AsRef<Path>) -> error::Result<String> {
        let mut file = File::open(path.as_ref())
            .map_err(|e| error::Error::OpenFile(e, path.as_ref().display().to_string()))?;

        let mut data = String::new();
        file.read_to_string(&mut data)
            .map_err(|e| error::Error::ReadFileToString(e, path.as_ref().display().to_string()))?;

        Ok(data.trim().to_owned())
    }
}
