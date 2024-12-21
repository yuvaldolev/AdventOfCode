use crate::{guard::Guard, lab::Lab, tracker::Tracker};

pub struct World {
    lab: Lab,
    guard: Guard,
    tracker: Tracker,
}

impl World {
    pub fn new(lab: Lab, guard: Guard) -> Self {
        let tracker = Tracker::new(guard.get_position(), guard.get_velocity());

        Self {
            lab,
            guard,
            tracker,
        }
    }

    pub fn simulate(&mut self) -> bool {
        loop {
            let next_guard_position = self.guard.get_next_position();

            if self.lab.is_out_of_bounds(next_guard_position) {
                return true;
            }

            if self.lab.is_colliding_with_obstacle(next_guard_position) {
                self.guard.turn();
                continue;
            }

            self.guard.move_to_position(next_guard_position.as_uvec2());

            if self.tracker.is_guard_movement_already_tracked(
                self.guard.get_position(),
                self.guard.get_velocity(),
            ) {
                return false;
            }

            self.tracker
                .track_guard_movement(self.guard.get_position(), self.guard.get_velocity());
        }
    }
}
