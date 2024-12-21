use std::collections::HashSet;

use glam::{IVec2, UVec2};

pub struct Tracker {
    guard_tracks: HashSet<(UVec2, IVec2)>,
}

impl Tracker {
    pub fn new(initial_guard_position: UVec2, initial_guard_velocity: IVec2) -> Self {
        let mut guard_tracks: HashSet<(UVec2, IVec2)> = HashSet::new();
        guard_tracks.insert((initial_guard_position, initial_guard_velocity));

        Self { guard_tracks }
    }

    pub fn track_guard_movement(&mut self, position: UVec2, velocity: IVec2) {
        self.guard_tracks.insert((position, velocity));
    }

    pub fn is_guard_movement_already_tracked(&mut self, position: UVec2, velocity: IVec2) -> bool {
        self.guard_tracks.contains(&(position, velocity))
    }
}
