use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

mod render;
mod game;

use crate::render::Marbles;
use crate::game::World;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?;
    let width = 1024;
    let height = 768;
    let mut canvas = video
        .window("Bounce", width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?
        .into_canvas()
        .present_vsync()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump()?;

    let mut marbles = Marbles::new(&texture_creator);
    let mut world = World::new(width as f64, height as f64);

    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        world.step(17.0);
        world.draw(|x, y, r, c| {
            let marble = marbles.get(r, c).unwrap();
            canvas.copy(&marble, None, Some(Rect::new((x-r) as i32, (y-r) as i32, 2*r+1, 2*r+1)))
        })?;
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    };
    Ok(())
}
