use chip8_core::*;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::env;
use std::fs::File;
use std::io::Read;

const SCALE: u32 = 10;
const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT as u32) * SCALE;

fn draw_screen(chip8: &Chip8, canvas: &mut Canvas<Window>) {
    // clear canvas
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = chip8.get_display();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            let x = (i % DISPLAY_WIDTH) as u32;
            let y = (i / DISPLAY_WIDTH) as u32;

            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/rom");
        return;
    }
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip-8", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump: sdl2::EventPump = sdl_context.event_pump().unwrap();

    let mut chip8: Chip8 = Chip8::new();
    let mut rom: File = File::open(&args[1]).expect("Unable to open file");
    let mut buffer: Vec<u8> = Vec::new();

    rom.read_to_end(&mut buffer).unwrap();
    chip8.load(&buffer);

    'gameloop: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit { .. } => {
                    break 'gameloop;
                }
                _ => (),
            }
        }
        chip8.tick();
        draw_screen(&chip8, &mut canvas);
    }
}
