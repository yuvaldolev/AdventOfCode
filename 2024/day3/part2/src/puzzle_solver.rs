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

        let (result, _) =
            instructions.iter().fold(
                (0, true),
                |(result, enabled), instruction| match instruction {
                    Instruction::Multiply(x, y) if enabled => (result + (x * y), true),
                    Instruction::Enable => (result, true),
                    Instruction::Disable => (result, false),
                    _ => (result, enabled),
                },
            );

        Ok(result)
    }
}
