pub struct Map {
    pub grid: Vec<Vec<i32>>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            grid: vec![
                vec![1, 1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1, 1],
            ],
        }
    }

    pub fn is_wall(&self, x: f64, y: f64) -> bool {
        self.grid[y as usize][x as usize] != 0
    }
}
