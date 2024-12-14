use day3_2024_common::{InputParser, Instruction};

pub struct PuzzleSolver {
    input_parser: InputParser,
}

impl PuzzleSolver {
    pub fn new() -> Self {
        Self {
            input_parser: InputParser::new(),
        }
    }

    pub fn solve(&self, input: &str) -> day3_2024_common::Result<u32> {
        let instructions = self.input_parser.parse(input)?;

        let result = instructions
            .iter()
            .filter_map(|instruction| match instruction {
                Instruction::Multiply(x, y) => Some(x * y),
                _ => None,
            })
            .sum();

        Ok(result)
    }
}
