pub struct InputParser;

impl InputParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, input: &str) -> Vec<Vec<char>> {
        input.lines().map(|line| line.chars().collect()).collect()
    }
}
