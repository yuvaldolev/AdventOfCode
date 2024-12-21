use glam::{IVec2, UVec2};

pub struct Guard {
    position: UVec2,
    velocity: IVec2,
}

impl Guard {
    pub fn new(position: UVec2, velocity: IVec2) -> Self {
        Self { position, velocity }
    }

    pub fn get_next_position(&self) -> IVec2 {
        self.position.as_ivec2() + self.velocity
    }

    pub fn move_to_position(&mut self, position: UVec2) {
        self.position = position;
    }

    pub fn turn(&mut self) {
        self.velocity = match self.velocity {
            IVec2 { x: 0, y: -1 } => IVec2::new(1, 0),
            IVec2 { x: 1, y: 0 } => IVec2::new(0, 1),
            IVec2 { x: 0, y: 1 } => IVec2::new(-1, 0),
            IVec2 { x: -1, y: 0 } => IVec2::new(0, -1),
            _ => self.velocity,
        }
    }
}
