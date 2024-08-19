pub struct Map {
    pub grid: Vec<Vec<i32>>,
    pub key_position: (f32, f32),  // Changed to f32 to match the player's position type
}

impl Map {
    pub fn new() -> Map {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 1, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1, 1, 0, 1],
            vec![1, 0, 1, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        let key_position = (5.5, 3.5);  // Position the key at the center of the cell

        Map { grid, key_position }
    }
}
