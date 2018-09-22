extern crate sdl2;
extern crate unicode_segmentation;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::io::prelude::*;
use std::env;

const WINDOW_WIDTH: u32 = 480;
const WINDOW_HEIGHT: u32 = 480;
const BG_COLOR: Color = Color{r: 255, g: 255, b: 255, a: 255};

macro_rules! rect(($x:expr, $y:expr, $w:expr, $h:expr) => (sdl2::rect::Rect::new($x as i32, $y as i32, $w as u32, $h as u32)));

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

    let mut lines_str: Vec<String> = read_file(&args[1]).split("\n").map(|x| x.to_owned()).collect();
    let mut lines: Vec<u8> = Vec::new();

    for _ in 0..lines_str.len() {
        lines.append(&mut lines_str.pop().unwrap().into_bytes());
    }

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

        canvas.set_draw_color(BG_COLOR);
        canvas.clear();
        
        for i in 0..lines.len()/3 {
            let r = match lines.get(i) {
                Some(n) => (27*(n.clone() as u32)/10) - 86,
                None => 0
            };
            let g = match lines.get(i+1) {
                Some(n) => (27*(n.clone() as u32)/10) - 86,
                None => 0
            };
            let b = match lines.get(i+2) {
                Some(n) => (27*(n.clone() as u32)/10) - 86,
                None => 0
            };

            canvas.set_draw_color(Color{r: r as u8, g: g as u8, b: b as u8, a: 255});
            let _ = canvas.fill_rect(rect![16*(i as u32 % (WINDOW_WIDTH/16)), 16*(i as u32 / (WINDOW_WIDTH/16)), 16, 16]);
        }

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        canvas.present();
    }
}

fn read_file(path: &str) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let mut s = String::new();
    let _ = file.read_to_string(&mut s);

    s
}
