mod player;
mod raycaster;
mod map;
mod renderer;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Raycaster", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut player = player::Player::new(3.0, 3.0, 0.0);
    let map = map::Map::new();
    let mut renderer = renderer::Renderer::new(&texture_creator);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => player.move_forward(0.1, &map),
                Event::KeyDown { keycode: Some(Keycode::S), .. } => player.move_backward(0.1, &map),
                Event::KeyDown { keycode: Some(Keycode::A), .. } => player.rotate(-0.1),
                Event::KeyDown { keycode: Some(Keycode::D), .. } => player.rotate(0.1),
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        renderer.render_scene(&mut canvas, &player, &map);

        canvas.present();
        std::thread::sleep(Duration::from_millis(1000 / 60));
    }
}
