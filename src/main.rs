extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
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

impl Coord {
    fn get_neighbours(&self) -> [Coord;8]{
        let mut arr:Vec<Coord> = Vec::new();

        for x in [self.x-1,self.x,self.x+1]{
            for y in [self.y-1,self.y,self.y+1]{
                if !(x == y && x == 0){
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
    let test = Coord::from((0,0));
    let neighbours = test.get_neighbours();

    println!("{neighbours:?}");


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
    'running: loop {
        
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
                _ => {}
            }
        }
        // The rest of the loop goes here..

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}