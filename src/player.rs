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
        if !map.is_wall(new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn move_backward(&mut self, distance: f64, map: &Map) {
        let new_x = self.x - self.angle.cos() * distance;
        let new_y = self.y - self.angle.sin() * distance;
        if !map.is_wall(new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn rotate(&mut self, angle: f64) {
        self.angle += angle;
    }
}
