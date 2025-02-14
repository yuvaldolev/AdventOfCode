use day6_2024_common::InputParser;

use crate::world::World;

pub struct PuzzleSolver {
    input_parser: InputParser,
}

impl PuzzleSolver {
    pub fn new() -> Self {
        Self {
            input_parser: InputParser::new(),
        }
    }

    pub fn solve(&self, input: &str) -> day6_2024_common::Result<usize> {
        let world_data = self.input_parser.parse(input);

        let mut world = World::new(world_data);
        world.simulate();
        let unique_guard_positions = world.get_unique_guard_positions();

        Ok(unique_guard_positions.len())
    }
}
