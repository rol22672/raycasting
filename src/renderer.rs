use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::player::Player;
use crate::map::Map;
use crate::raycaster::{self, RayHit};

pub struct Renderer<'a> {
    texture_creator: &'a TextureCreator<Window>,
}

impl<'a> Renderer<'a> {
    pub fn new(texture_creator: &'a TextureCreator<Window>) -> Renderer<'a> {
        Renderer { texture_creator }
    }

    pub fn render_scene(&mut self, canvas: &mut Canvas<Window>, player: &Player, map: &Map) {
        let screen_width = canvas.viewport().width();
        let screen_height = canvas.viewport().height();

        for x in 0..screen_width {
            let ray_angle = player.angle + ((x as f64 / screen_width as f64) - 0.5) * 60.0_f64.to_radians();
            let ray_hit = raycaster::cast_ray(player, map, ray_angle);

            let distance = ray_hit.distance * (ray_angle - player.angle).cos();
            let wall_height = (screen_height as f64 / distance) as u32;

            let color = if ray_hit.is_vertical {
                Color::RGB(150, 150, 150)
            } else {
                Color::RGB(200, 200, 200)
            };

            canvas.set_draw_color(color);
            let start_y = (screen_height / 2) - (wall_height / 2);
            let end_y = start_y + wall_height;
            canvas.fill_rect(Rect::new(x as i32, start_y as i32, 1, wall_height)).unwrap();
        }
    }
}
