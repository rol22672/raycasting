use ggez::audio::{self, SoundSource};
use ggez::graphics::{self, Color, DrawParam, Font, Text, Image, Mesh};
use ggez::{Context, GameResult, input::keyboard};
use ggez::timer;
use std::time::Instant;

use crate::player::Player;
use crate::map::Map;
use crate::raycaster;

pub struct Renderer {
    wall_texture: Image,
    key_texture: Image,
    font: Font,
    key_pickup_sound: audio::Source,
    key_collected: bool,
    victory_time: Option<Instant>,  // Track when the player won
}

impl Renderer {
    pub fn new(ctx: &mut Context) -> GameResult<Renderer> {
        let wall_texture = Image::new(ctx, "/wall.png")?;
        let key_texture = Image::new(ctx, "/key.png")?;
        let font = Font::default();
        let key_pickup_sound = audio::Source::new(ctx, "/key-pickup.wav")?;
        Ok(Renderer {
            wall_texture,
            key_texture,
            font,
            key_pickup_sound,
            key_collected: false,
            victory_time: None,
        })
    }

    pub fn render_scene(&mut self, ctx: &mut Context, player: &Player, map: &Map) -> GameResult {
        let screen_width = graphics::drawable_size(ctx).0;
        let screen_height = graphics::drawable_size(ctx).1;

        // If the player has won, show the "Ganaste" message and stop further rendering
        if let Some(victory_time) = self.victory_time {
            self.show_victory_message(ctx, screen_width, screen_height)?;
            if victory_time.elapsed().as_secs() > 3 {
                if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Return) {
                    // Reset game logic here if needed
                    self.reset_game();
                }
            }
            return Ok(()); // Skip rendering other elements
        }

        // Draw sky and floor
        self.draw_sky_and_floor(ctx, screen_width, screen_height)?;

        // Draw walls
        self.draw_walls(ctx, player, map, screen_width, screen_height)?;

        // Draw the key in 3D space
        self.draw_key(ctx, player, map, screen_width, screen_height)?;

        // Detect if the player has collected the key
        self.detect_key_pickup(ctx, player, map)?;

        // Draw the minimap and FPS counter
        self.draw_minimap(ctx, player, map)?;
        self.draw_fps(ctx)?;

        Ok(())
    }

    fn draw_sky_and_floor(&self, ctx: &mut Context, screen_width: f32, screen_height: f32) -> GameResult {
        // Draw sky (top half of the screen)
        let sky_color = Color::from_rgb(135, 206, 235); // Light sky blue
        let sky_rect = graphics::Rect::new(0.0, 0.0, screen_width, screen_height / 2.0);
        let sky_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), sky_rect, sky_color)?;
        graphics::draw(ctx, &sky_mesh, DrawParam::default())?;

        // Draw floor (bottom half of the screen)
        let floor_color = Color::from_rgb(139, 69, 19); // Saddle brown
        let floor_rect = graphics::Rect::new(0.0, screen_height / 2.0, screen_width, screen_height / 2.0);
        let floor_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), floor_rect, floor_color)?;
        graphics::draw(ctx, &floor_mesh, DrawParam::default())?;

        Ok(())
    }

    fn draw_walls(&self, ctx: &mut Context, player: &Player, map: &Map, screen_width: f32, screen_height: f32) -> GameResult {
        for x in 0..screen_width as i32 {
            let ray_angle = player.angle + ((x as f64 / screen_width as f64) - 0.5) * 60.0_f64.to_radians();
            let ray_hit = raycaster::cast_ray(player, map, ray_angle);

            let distance = ray_hit.distance * (ray_angle - player.angle).cos();
            let wall_height = (screen_height as f64 / distance) as f32;
            let wall_start = (screen_height - wall_height) / 2.0;

            // Determine the x-coordinate on the texture
            let texture_x = (self.wall_texture.width() as f64 * (ray_hit.distance % 1.0)) as f32;

            // Render the wall
            let params = DrawParam::default()
                .src(graphics::Rect::new(
                    texture_x / self.wall_texture.width() as f32,
                    0.0,
                    1.0 / self.wall_texture.width() as f32,
                    1.0,
                ))
                .dest([x as f32, wall_start])
                .scale([1.0, wall_height / self.wall_texture.height() as f32]);

            graphics::draw(ctx, &self.wall_texture, params)?;
        }

        Ok(())
    }
    fn draw_key(&self, ctx: &mut Context, player: &Player, map: &Map, screen_width: f32, screen_height: f32) -> GameResult {
        // Calculate the distance from the player to the key
        let dx = map.key_position.0 as f64 - player.x as f64;
        let dy = map.key_position.1 as f64 - player.y as f64;
        let key_distance = (dx * dx + dy * dy).sqrt();
    
        // Calculate the angle to the key from the player's perspective
        let angle_to_key = (dy).atan2(dx) - player.angle;
    
        // Check if the key is within the player's field of view and within a certain distance
        if key_distance < 10.0 && angle_to_key.abs() < std::f64::consts::PI / 4.0 {
            // Calculate the key's screen position
            let screen_x = (screen_width as f64 / 2.0) + (angle_to_key.to_degrees() / 60.0 * screen_width as f64);
            let key_height = (screen_height as f64 / key_distance) as f32;
            let screen_y = (screen_height / 2.0) + key_height / 4.0;
    
            // Increase the scale to make the key larger
            let scale_factor = 1.5;  // Adjust this value to make the key bigger or smaller
            let params = DrawParam::default()
                .dest([screen_x as f32, screen_y])
                .scale([scale_factor, scale_factor]);
    
            graphics::draw(ctx, &self.key_texture, params)?;
        }
    
        Ok(())
    }
    
    
    

    fn detect_key_pickup(&mut self, ctx: &mut Context, player: &Player, map: &Map) -> GameResult {
        // Check if the player is close enough to the key to pick it up
        let key_distance = (((player.x as f64) - (map.key_position.0 as f64)).powi(2) + 
                            ((player.y as f64) - (map.key_position.1 as f64)).powi(2)).sqrt();

        if !self.key_collected && key_distance < 0.5 {
            self.key_pickup_sound.play(ctx)?;
            self.key_collected = true;
            self.victory_time = Some(Instant::now()); // Record the victory time
        }

        Ok(())
    }

    fn show_victory_message(&self, ctx: &mut Context, screen_width: f32, screen_height: f32) -> GameResult {
        // Display "Ganaste" message
        let victory_text = Text::new(("Ganaste", self.font, 48.0));
        let text_position = [
            screen_width / 2.0 - victory_text.width(ctx) as f32 / 2.0, 
            screen_height / 2.0 - victory_text.height(ctx) as f32 / 2.0
        ];
        graphics::draw(ctx, &victory_text, DrawParam::default().dest(text_position))?;
        Ok(())
    }

    fn reset_game(&mut self) {
        self.key_collected = false;
        self.victory_time = None;
        // Reset other game states if necessary
    }

    fn draw_minimap(&self, ctx: &mut Context, player: &Player, map: &Map) -> GameResult {
        let minimap_scale = 4.0;
        let minimap_size = 50.0;

        // Define the minimap background
        let minimap_rect = graphics::Rect::new(10.0, 10.0, minimap_size, minimap_size);
        let minimap_bg = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), minimap_rect, Color::from_rgb(50, 50, 50))?;
        graphics::draw(ctx, &minimap_bg, DrawParam::default())?;

        // Draw walls on the minimap
        for y in 0..map.grid.len() {
            for x in 0..map.grid[y].len() {
                if map.grid[y][x] == 1 {
                    let wall_rect = graphics::Rect::new(
                        10.0 + x as f32 * minimap_scale,
                        10.0 + y as f32 * minimap_scale,
                        minimap_scale,
                        minimap_scale,
                    );
                    let wall_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), wall_rect, Color::from_rgb(255, 255, 255))?;
                    graphics::draw(ctx, &wall_mesh, DrawParam::default())?;
                }
            }
        }

        // Draw player on the minimap
        let player_rect = graphics::Rect::new(
            10.0 + player.x as f32 * minimap_scale - 2.0,
            10.0 + player.y as f32 * minimap_scale - 2.0,
            4.0,
            4.0,
        );
        let player_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), player_rect, Color::from_rgb(255, 0, 0))?;
        graphics::draw(ctx, &player_mesh, DrawParam::default())?;

        Ok(())
    }

    fn draw_fps(&self, ctx: &mut Context) -> GameResult {
        let fps = timer::fps(ctx);
        let fps_display = format!("FPS: {:.0}", fps);

        let fps_text = Text::new((fps_display, self.font, 16.0));
        let fps_position = [10.0, 70.0];
        graphics::draw(ctx, &fps_text, (fps_position,))?;

        Ok(())
    }
}
