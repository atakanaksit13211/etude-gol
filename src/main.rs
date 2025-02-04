extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;

use sdl2::rect::Rect;

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
    let gray = Color::RGB(127, 106, 85);
    let yellow = Color::RGB(218, 130, 19);
    let black = Color::RGB(69, 50, 32);

    let mut grid_cell_size:i32 = 36;
    let mut grid_width = 29;
    let mut grid_height = 23;

    // + 1 so that the last grid lines fit in the screen.
    let mut window_width = (grid_width * grid_cell_size) + 1;
    let mut window_height = (grid_height * grid_cell_size) + 1;


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("etude-gol", window_width.try_into().unwrap(), window_height.try_into().unwrap())
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(66, 68, 72));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    let mut running: bool = false;
    

    let mut grid_cursor = Rect::new(((grid_width - 1) / 2 * grid_cell_size).try_into().unwrap(),
                                          ((grid_height - 1) / 2 * grid_cell_size).try_into().unwrap(),
                                          grid_cell_size.try_into().unwrap(),
                                          grid_cell_size.try_into().unwrap(),);


    let mut grid_cursor_ghost = Rect::new(grid_cursor.x.clone(),
                                          grid_cursor.y.clone(),
                                          grid_cell_size.try_into().unwrap(),
                                          grid_cell_size.try_into().unwrap(),);


    'running: loop {
        canvas.set_draw_color(black);
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    running = !running;
                },

                Event::KeyDown { keycode: Some(Keycode::Equals), .. } => {
                    grid_cell_size += 2;
                    grid_width = (window_width-1)/grid_cell_size;
                    grid_height = (window_height-1)/grid_cell_size;
                    grid_cursor.set_width(grid_cell_size.try_into().unwrap());
                    grid_cursor.set_height(grid_cell_size.try_into().unwrap());
                    grid_cursor_ghost.set_width(grid_cell_size.try_into().unwrap());
                    grid_cursor_ghost.set_height(grid_cell_size.try_into().unwrap());
                },
                Event::KeyDown { keycode: Some(Keycode::Minus), .. } => {
                    if(grid_cell_size > 3){
                        grid_cell_size -= 2;
                        grid_width = (window_width-1)/grid_cell_size;
                        grid_height = (window_height-1)/grid_cell_size;
                        grid_cursor.set_width(grid_cell_size.try_into().unwrap());
                        grid_cursor.set_height(grid_cell_size.try_into().unwrap());
                        grid_cursor_ghost.set_width(grid_cell_size.try_into().unwrap());
                        grid_cursor_ghost.set_height(grid_cell_size.try_into().unwrap());
                    }
                },

                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    grid_cursor.y += grid_cell_size;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    grid_cursor.y -= grid_cell_size;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    grid_cursor.x -= grid_cell_size;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    grid_cursor.x += grid_cell_size;
                },

                Event::MouseMotion {x, y, ..} => {
                    grid_cursor_ghost.x = (x / grid_cell_size) * grid_cell_size;
                    grid_cursor_ghost.y = (y / grid_cell_size) * grid_cell_size;
                }
                Event::MouseButtonDown {x, y, ..} => {
                    grid_cursor.x = (x / grid_cell_size) * grid_cell_size;
                    grid_cursor.y = (y / grid_cell_size) * grid_cell_size;
                }
                _ => {}
            }
        }
        // The rest of the loop goes here..

        canvas.set_draw_color(gray);
        //vertical lines
        for x in (0..1 + grid_width * grid_cell_size).step_by(grid_cell_size.try_into().unwrap()){
            canvas.draw_line(Point::new(x, 0), Point::new(x, window_height));
        }
        //horizontal lines
        for y in (0..1 + grid_height * grid_cell_size).step_by(grid_cell_size.try_into().unwrap()){
            canvas.draw_line(Point::new(0, y), Point::new(window_width, y));
        }


        canvas.set_draw_color(white);
        canvas.draw_rect(grid_cursor);


        canvas.set_draw_color(yellow);
        canvas.draw_rect(grid_cursor_ghost);


        
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}