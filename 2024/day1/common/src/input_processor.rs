use crate::error;

pub struct InputProcessor;

impl InputProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process(&self, input: &str) -> error::Result<(Vec<u32>, Vec<u32>)> {
        let mut left: Vec<u32> = Vec::new();
        let mut right: Vec<u32> = Vec::new();

        for line in input.lines() {
            let mut split = line.split_whitespace();
            left.push(
                split
                    .next()
                    .ok_or_else(|| error::Error::InputLineMissingLeftLocationId(line.to_owned()))?
                    .parse()
                    .map_err(error::Error::ParseLeftLocationId)?,
            );
            right.push(
                split
                    .next()
                    .ok_or_else(|| error::Error::InputLineMissingRightLocationId(line.to_owned()))?
                    .parse()
                    .map_err(error::Error::ParseRightLocationId)?,
            );
        }

        Ok((left, right))
    }
}
