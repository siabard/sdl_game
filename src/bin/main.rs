use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};

use sdl2::image::{self, InitFlag, LoadTexture};

const FPS: u32 = 60;
const FRAME_DELAY: u32 = 1000 / FPS;

fn render(canvas: &mut WindowCanvas, color: Color, texture: &Texture) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    canvas.copy(texture, None, None)?;

    canvas.present();
    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _img_context = image::init(InitFlag::PNG | InitFlag::JPG);
    let mut timer = sdl_context.timer().unwrap();

    let window = video_subsystem
        .window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    let mut game_running = true;

    while game_running {
        let start_tick = timer.ticks();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    game_running = false;
                }
                _ => {}
            }
        }

        i = (i + 1) % 255;
        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture)?;

        // FPS
        let tick_span = timer.ticks() - start_tick;

        if tick_span < FRAME_DELAY {
            std::thread::sleep(std::time::Duration::from_millis(
                (FRAME_DELAY - tick_span) as u64,
            ));
        }
    }

    Ok(())
}
