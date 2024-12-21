use day6_2024_common::WorldData;

use crate::{guard::Guard, lab::Lab, tracker::Tracker};

pub struct World {
    lab: Lab,
    guard: Guard,
    tracker: Tracker,
}

impl World {
    pub fn new(data: WorldData) -> Self {
        let lab = Lab::new(data.get_lab_tiles().to_vec());
        let guard = Guard::new(
            data.get_initial_guard_position(),
            data.get_initial_guard_velocity(),
        );
        let tracker = Tracker::new(data.get_initial_guard_position());

        Self {
            lab,
            guard,
            tracker,
        }
    }

    pub fn simulate(&mut self) {
        loop {
            let next_guard_position = self.guard.get_next_position();

            if self.lab.is_out_of_bounds(next_guard_position) {
                break;
            }

            if self.lab.is_colliding_with_obstacle(next_guard_position) {
                self.guard.turn();
                continue;
            }

            self.guard.move_to_position(next_guard_position.as_uvec2());
            self.tracker
                .track_guard_movement(next_guard_position.as_uvec2());
        }
    }

    pub fn get_tracker(&self) -> &Tracker {
        &self.tracker
    }
}
