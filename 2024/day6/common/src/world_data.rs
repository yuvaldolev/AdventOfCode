use glam::{IVec2, UVec2};

pub struct WorldData {
    lab_tiles: Vec<Vec<bool>>,
    initial_guard_position: UVec2,
    initial_guard_velocity: IVec2,
}

impl WorldData {
    pub fn new(
        lab_tiles: Vec<Vec<bool>>,
        initial_guard_position: UVec2,
        initial_guard_velocity: IVec2,
    ) -> Self {
        Self {
            lab_tiles,
            initial_guard_position,
            initial_guard_velocity,
        }
    }

    pub fn get_lab_tiles(&self) -> &[Vec<bool>] {
        &self.lab_tiles
    }

    pub fn get_initial_guard_position(&self) -> UVec2 {
        self.initial_guard_position
    }

    pub fn get_initial_guard_velocity(&self) -> IVec2 {
        self.initial_guard_velocity
    }
}
