extern crate sdl2;
extern crate unicode_segmentation;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas};
use sdl2::video::{Window, WindowContext};

use std::io::prelude::*;
use std::env;

const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 1000;
const BG_COLOR: Color = Color{r: 25, g: 25, b: 25, a: 255};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let args: Vec<String> = env::args().collect();

    let window = video_subsystem.window("ColorCoding", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .allow_highdpi()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(BG_COLOR);

    let lines: Vec<String> = read_file(&args[1]).split("\n").map(|x| x.to_owned()).collect();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },

                _ => {}
            }
        }

        let duration = std::time::Duration::new(frame_time as u64, ((frame_time - (frame_time as usize) as f64)*1_000_000_000.0) as u32);
        std::thread::sleep(duration);
        canvas.clear();
    }
}

fn read_file(path: &str) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let mut s = String::new();
    let _ = file.read_to_string(&mut s);

    s
}
