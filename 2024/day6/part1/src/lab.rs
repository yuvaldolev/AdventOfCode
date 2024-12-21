use glam::IVec2;

pub struct Lab {
    tiles: Vec<Vec<bool>>,
}

impl Lab {
    pub fn new(tiles: Vec<Vec<bool>>) -> Self {
        Self { tiles }
    }

    pub fn is_out_of_bounds(&self, position: IVec2) -> bool {
        (position.x < 0)
            || ((position.x as usize) >= self.tiles[0].len())
            || (position.y < 0)
            || ((position.y as usize) >= self.tiles.len())
    }

    pub fn is_colliding_with_obstacle(&self, position: IVec2) -> bool {
        self.tiles[position.y as usize][position.x as usize]
    }
}
