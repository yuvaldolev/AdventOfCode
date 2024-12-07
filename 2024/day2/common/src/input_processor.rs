use crate::error;

pub struct InputProcessor;

impl InputProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process(&self, input: &str) -> error::Result<Vec<Vec<u32>>> {
        let mut reports: Vec<Vec<u32>> = Vec::new();

        for line in input.lines() {
            reports.push(
                line.split_whitespace()
                    .map(|level| {
                        level
                            .parse()
                            .map_err(|e| error::Error::ParseLevel(e, level.to_owned()))
                    })
                    .collect::<error::Result<Vec<_>>>()?,
            );
        }

        Ok(reports)
    }
}
