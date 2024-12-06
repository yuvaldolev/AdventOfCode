use std::collections::HashMap;

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
        let (left, right) = self.input_processor.process(input)?;
        Ok(Self::calculate_similarity_score(&left, &right))
    }

    fn calculate_similarity_score(left: &[u32], right: &[u32]) -> u32 {
        let mut location_id_scores: HashMap<u32, u32> = HashMap::new();

        left.iter()
            .map(|&left_item| {
                *(location_id_scores.entry(left_item).or_insert_with(|| {
                    left_item
                        * (right
                            .iter()
                            .filter(|&&right_item| left_item == right_item)
                            .count() as u32)
                }))
            })
            .sum()
    }
}
