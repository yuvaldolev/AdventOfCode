use glam::{IVec2, UVec2};

use crate::WorldData;

pub struct InputParser;

impl InputParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, input: &str) -> WorldData {
        let mut lab_tiles: Vec<Vec<bool>> = Vec::new();
        let mut initial_guard_position = UVec2::ZERO;
        let mut initial_guard_velocity = IVec2::ZERO;

        for (y, line) in input.lines().enumerate() {
            let mut line_tiles: Vec<bool> = Vec::with_capacity(line.len());

            for (x, character) in line.chars().enumerate() {
                match character {
                    '#' => line_tiles.push(true),
                    '^' => {
                        line_tiles.push(false);
                        initial_guard_position = UVec2::new(x as u32, y as u32);
                        initial_guard_velocity = IVec2::new(0, -1);
                    }
                    '>' => {
                        line_tiles.push(false);
                        initial_guard_position = UVec2::new(x as u32, y as u32);
                        initial_guard_velocity = IVec2::new(1, 0);
                    }
                    'v' => {
                        line_tiles.push(false);
                        initial_guard_position = UVec2::new(x as u32, y as u32);
                        initial_guard_velocity = IVec2::new(0, 1);
                    }
                    '<' => {
                        line_tiles.push(false);
                        initial_guard_position = UVec2::new(x as u32, y as u32);
                        initial_guard_velocity = IVec2::new(-1, 0);
                    }
                    _ => line_tiles.push(false),
                }
            }

            lab_tiles.push(line_tiles);
        }

        WorldData::new(lab_tiles, initial_guard_position, initial_guard_velocity)
    }
}
