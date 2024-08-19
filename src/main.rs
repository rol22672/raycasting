use ggez::audio::{self, SoundSource};
use ggez::event::{self, EventHandler, KeyCode, MouseButton};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use std::env;
use std::path;

mod player;
mod raycaster;
mod map;
mod renderer;

struct MyGame {
    player: player::Player,
    map: map::Map,
    renderer: renderer::Renderer,
    last_mouse_x: f32,
    music: audio::Source, // Add a field to hold the music
}

impl MyGame {
    fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let player = player::Player::new(3.0, 3.0, 0.0);
        let map = map::Map::new();
        let renderer = renderer::Renderer::new(ctx)?;

        // Load the music file
        let mut music = audio::Source::new(ctx, "/music.ogg").map_err(|e| {
            eprintln!("Failed to load music: {:?}", e);
            e
        })?;
        music.set_repeat(true); // Set the music to loop
        music.play(ctx).map_err(|e| {
            eprintln!("Failed to play music: {:?}", e);
            e
        })?;

        Ok(MyGame {
            player,
            map,
            renderer,
            last_mouse_x: 0.0,
            music,
        })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Handle keyboard input to move the player
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.player.move_forward(0.1, &self.map);
        }
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.player.move_backward(0.1, &self.map);
        }
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.player.rotate(-0.1);
        }
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.player.rotate(0.1);
        }

        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, _y: f32, _dx: f32, _dy: f32) {
        // Calculate how much the mouse moved since the last frame
        let delta_x = x - self.last_mouse_x;

        // Rotate the player based on mouse movement
        self.player.rotate((delta_x as f64) * 0.005);

        // Update the last mouse X position
        self.last_mouse_x = x;
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        // Render the scene
        self.renderer.render_scene(ctx, &self.player, &self.map)?;

        graphics::present(ctx)
    }
}

fn main() -> GameResult {
    // Set up a resource path for loading files if needed
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("racaster", "author")
        .add_resource_path(resource_dir)
        .build()?;
    let mut game = MyGame::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}
