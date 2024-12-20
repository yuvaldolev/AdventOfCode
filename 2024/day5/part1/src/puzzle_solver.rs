use day5_2024_common::InputParser;
use the_algorithms_rust::graph;

pub struct PuzzleSolver {
    input_parser: InputParser,
}

impl PuzzleSolver {
    pub fn new() -> Self {
        Self {
            input_parser: InputParser::new(),
        }
    }

    pub fn solve(&self, input: &str) -> day5_2024_common::Result<u32> {
        let safety_manual_updates = self.input_parser.parse(input)?;

        let result = safety_manual_updates
            .get_pages_to_produce_per_update()
            .iter()
            .filter(|&pages_to_produce| {
                match graph::topological_sort(
                    &safety_manual_updates
                        .get_page_ordering_rules()
                        .iter()
                        .cloned()
                        .filter(|(before, after)| {
                            pages_to_produce.contains(before) && pages_to_produce.contains(after)
                        })
                        .collect(),
                ) {
                    Ok(ordered_pages_to_produce) => ordered_pages_to_produce == *pages_to_produce,
                    Err(_) => false,
                }
            })
            .map(|pages_to_produce| pages_to_produce[pages_to_produce.len() / 2])
            .sum();

        Ok(result)
    }
}
