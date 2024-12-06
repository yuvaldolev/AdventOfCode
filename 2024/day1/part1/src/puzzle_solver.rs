use std::iter;

use day1_common_2024::InputProcessor;

pub struct PuzzleSolver {
    input_processor: InputProcessor,
}

impl PuzzleSolver {
    pub fn new() -> Self {
        Self {
            input_processor: InputProcessor::new(),
        }
    }

    pub fn solve(&self, input: &str) -> day1_common_2024::Result<u32> {
        let (mut left, mut right) = self.input_processor.process(input)?;

        left.sort();
        right.sort();

        Ok(Self::calculate_total_distance(&left, &right))
    }

    fn calculate_total_distance(left: &[u32], right: &[u32]) -> u32 {
        iter::zip(left, right)
            .map(|(&left_item, &right_item)| left_item.abs_diff(right_item))
            .sum()
    }
}
