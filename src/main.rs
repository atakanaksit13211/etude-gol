extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;

#[derive(Debug)]
struct Coord{x:i32,y:i32}

impl From<(i32,i32)> for Coord{
    fn from((x,y):(i32,i32)) -> Self {
        Self {
            x,
            y
        }
    }
}

impl From<Coord> for Point{
    fn from(value: Coord) -> Self {
        Point::new(value.x, value.y)
    }
}

impl Coord {
    fn get_neighbours(&self) -> [Coord;8]{
        let mut arr:Vec<Coord> = Vec::new();

        for x in [self.x-1,self.x,self.x+1]{
            for y in [self.y-1,self.y,self.y+1]{
                if !(x == y && x == self.x){
                    arr.push(Coord::from((x,y)));
                }
            }
        }

        let arr:[Coord;8] = arr.try_into().expect("");
        arr
    }    
}
struct VisualGrid{
    size:u32, //pixels
    position:Coord
}

pub fn main() {

    let white = Color::RGB(246, 226, 157);
    let black = Color::RGB(69, 50, 32);


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("etude-gol", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(66, 68, 72));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running: bool = false;
    
    let mut grid_len:u32 = 100;
    
    'running: loop {
        canvas.set_draw_color(black);
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    running = !running;
                },
                Event::KeyDown { keycode: Some(Keycode::Equals), .. } => {
                    grid_len += 5;
                },
                Event::KeyDown { keycode: Some(Keycode::Minus), .. } => {
                    if grid_len > 6{
                        grid_len -= 5;
                    }
                },
                _ => {}
            }
        }
        // The rest of the loop goes here..
        canvas.set_draw_color(white);

        for x in (0..800).step_by(grid_len.try_into().unwrap()){
            canvas.draw_line(Point::new(x, 0), Point::new(x, 600));
        }
        for y in (0..600).step_by(grid_len.try_into().unwrap()){
            canvas.draw_line(Point::new(0, y), Point::new(800, y));
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}