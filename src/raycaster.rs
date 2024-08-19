use crate::player::Player;
use crate::map::Map;

pub struct RayHit {
    pub distance: f64,
    pub is_vertical: bool,
}

pub fn cast_ray(player: &Player, map: &Map, angle: f64) -> RayHit {
    let mut distance = 0.0;
    let mut hit = false;

    let mut ray_x = player.x;
    let mut ray_y = player.y;

    while !hit && distance < 20.0 {
        ray_x += angle.cos() * 0.1;
        ray_y += angle.sin() * 0.1;

        // Ensure ray stays within map bounds
        if ray_y < 0.0 || ray_y >= map.grid.len() as f64 || ray_x < 0.0 || ray_x >= map.grid[0].len() as f64 {
            break;
        }

        let map_x = ray_x as usize;
        let map_y = ray_y as usize;

        if map.grid[map_y][map_x] != 0 {
            hit = true;
        }

        distance += 0.1;
    }

    RayHit {
        distance,
        is_vertical: false,
    }
}
