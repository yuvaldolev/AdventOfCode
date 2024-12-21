use day6_2024_common::InputParser;
use glam::UVec2;

use crate::{guard::Guard, lab::Lab, world::World};

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
        let lab_tiles = world_data.get_lab_tiles();

        let result = (0..lab_tiles.len())
            .flat_map(|y| (0..lab_tiles[0].len()).map(move |x| (x, y)))
            .filter(|&(obstacle_x, obstacle_y)| {
                (!lab_tiles[obstacle_y][obstacle_x])
                    && (UVec2::new(obstacle_x as u32, obstacle_y as u32)
                        != world_data.get_initial_guard_position())
            })
            .filter(|&(obstacle_x, obstacle_y)| {
                let mut lab_tiles = lab_tiles.to_vec();
                lab_tiles[obstacle_y][obstacle_x] = true;

                let mut world = World::new(
                    Lab::new(lab_tiles),
                    Guard::new(
                        world_data.get_initial_guard_position(),
                        world_data.get_initial_guard_velocity(),
                    ),
                );

                !world.simulate()
            })
            .count();

        Ok(result)
    }
}
