use std::collections::HashSet;

use glam::UVec2;

pub struct Tracker {
    guard_positions: HashSet<UVec2>,
}

impl Tracker {
    pub fn new(initial_guard_position: UVec2) -> Self {
        let mut guard_positions: HashSet<UVec2> = HashSet::new();
        guard_positions.insert(initial_guard_position);

        Self { guard_positions }
    }

    pub fn get_guard_positions(&self) -> &HashSet<UVec2> {
        &self.guard_positions
    }

    pub fn track_guard_movement(&mut self, position: UVec2) {
        self.guard_positions.insert(position);
    }
}
