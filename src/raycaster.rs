use crate::player::Player;
use crate::map::Map;

pub struct RayHit {
    pub distance: f64,
    pub is_vertical: bool,
}

pub fn cast_ray(player: &Player, map: &Map, angle: f64) -> RayHit {
    let mut distance = 0.0;
    let mut is_vertical = false;

    let mut ray_x = player.x;
    let mut ray_y = player.y;

    while !map.is_wall(ray_x, ray_y) && distance < 20.0 {
        ray_x += angle.cos() * 0.1;
        ray_y += angle.sin() * 0.1;
        distance += 0.1;
    }

    RayHit { distance, is_vertical }
}
