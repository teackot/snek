mod game;

use std::time::{Duration, SystemTime};

use sdl2::{pixels::Color, keyboard::Keycode, event::Event, rect::Rect, render::Canvas, video::Window};

use game::Game;

const CELL_SIZE: u32 = 32;

fn main() -> Result<(), String> {
    let mut game = Game::new(25, 25);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("snek", game.width() * CELL_SIZE, game.height() * CELL_SIZE)
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut frame_start = SystemTime::now();
    let mut tick_start = SystemTime::now();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown {keycode: Some(Keycode::Escape),..} => break 'running,
                _ => (),
            }
        }

        const FRAME: Duration = Duration::new(0, 1_000_000_000u32 / 60); // target 60 FPS
        const TICK: Duration = Duration::new(0, 250_000_000u32); // target 60 FPS

        let now = SystemTime::now();
        let dt_frame = now.duration_since(frame_start).map_err(|e| e.to_string())?;
        let dt_tick = now.duration_since(tick_start).map_err(|e| e.to_string())?;

        if dt_frame < FRAME {
            std::thread::sleep(FRAME - dt_frame);
        }
        frame_start = SystemTime::now();

        if dt_tick >= TICK {
            let (gameover, kp) = game.tick();

            draw_cell(&mut canvas, kp.old_tail.0, kp.old_tail.1, Color::BLACK)?;
            draw_cell(&mut canvas, kp.old_head.0, kp.old_head.1, Color::RGB(0, 255, 0))?;
            draw_cell(&mut canvas, kp.new_head.0, kp.new_head.1, Color::RGB(0, 180, 0))?;
            draw_cell(&mut canvas, kp.food.0, kp.food.1, Color::RED)?;
            canvas.present();

            tick_start = SystemTime::now();
        }

        // canvas.clear();
        // canvas.present();
    }

    Ok(())
}

fn draw_cell(canvas: &mut Canvas<Window>, x: i32, y: i32, color: Color) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(x * CELL_SIZE as i32, y * CELL_SIZE as i32, CELL_SIZE, CELL_SIZE))
}
