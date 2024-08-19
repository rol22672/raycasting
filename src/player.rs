use crate::map::Map;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
}

impl Player {
    pub fn new(x: f64, y: f64, angle: f64) -> Player {
        Player { x, y, angle }
    }

    pub fn move_forward(&mut self, distance: f64, map: &Map) {
        let new_x = self.x + self.angle.cos() * distance;
        let new_y = self.y + self.angle.sin() * distance;

        // Check for collision before moving
        if !self.collides(new_x, new_y, map) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn move_backward(&mut self, distance: f64, map: &Map) {
        let new_x = self.x - self.angle.cos() * distance;
        let new_y = self.y - self.angle.sin() * distance;

        // Check for collision before moving
        if !self.collides(new_x, new_y, map) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn rotate(&mut self, angle: f64) {
        self.angle += angle;
    }

    fn collides(&self, x: f64, y: f64, map: &Map) -> bool {
        let map_x = x as usize;
        let map_y = y as usize;

        if map_x >= map.grid[0].len() || map_y >= map.grid.len() {
            return true; // Out of bounds is treated as a collision
        }

        map.grid[map_y][map_x] != 0 // Return true if the player would hit a wall
    }
}
