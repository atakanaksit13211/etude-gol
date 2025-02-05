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

    fn add(&mut self, other:&Coord){
        self.x += other.x;
        self.y += other.y;
    }
}
struct VisualGrid{
    grid_cell_size:i32,
    grid_width:i32, 
    grid_height:i32,

    window_width :i32,
    window_height:i32,

    curr_coord: Coord,
}

impl VisualGrid {
    const WHITE:Color = Color::RGB(246, 226, 157);
    const GRAY:Color = Color::RGB(127, 106, 85);
    const YELLOW:Color = Color::RGB(218, 130, 19);
    const BLACK:Color = Color::RGB(69, 50, 32);

    fn from(grid_cell_size:i32,
            grid_width:i32, 
            grid_height:i32) -> Self{
                Self{
                    grid_cell_size,
                grid_width, 
                grid_height,
                
                // + 1 so that the last grid lines fit in the screen.
                window_width  : ((grid_width  * grid_cell_size) + 1).try_into().unwrap(),
                window_height : ((grid_height * grid_cell_size) + 1).try_into().unwrap(),

                curr_coord: Coord::from((5000, 5000)), //middle of the grid.
                }
            }

    fn normalise_coord(&mut self){
        self.grid_width  = (self.window_width -1)/self.grid_cell_size;
        self.grid_height = (self.window_height-1)/self.grid_cell_size;
    }
}

impl From<&VisualGrid> for Rect{
    fn from(vg:&VisualGrid) -> Self{
        let rec = Rect::new(
            ((vg.grid_width - 1) / 2 * vg.grid_cell_size).try_into().unwrap(),
            ((vg.grid_height - 1) / 2 * vg.grid_cell_size).try_into().unwrap(),
            vg.grid_cell_size.try_into().unwrap(),
            vg.grid_cell_size.try_into().unwrap(),
        );
    
        rec
    }
}



fn normalise_coord(rec: &mut Rect, vg:&VisualGrid){
    rec.set_width(vg.grid_cell_size.try_into().unwrap());
    rec.set_height(vg.grid_cell_size.try_into().unwrap());
    rec.x = (rec.x / vg.grid_cell_size) * vg.grid_cell_size;
    rec.y = (rec.y / vg.grid_cell_size) * vg.grid_cell_size;
}

fn snap_to_closest(rec: &mut Rect, vg: &VisualGrid, x:i32, y:i32){
    rec.x = (x / vg.grid_cell_size) * vg.grid_cell_size;
    rec.y = (y / vg.grid_cell_size) * vg.grid_cell_size;
}

fn get_snap_coord(coor: &Coord, vg: &VisualGrid) -> Coord{
    let x = (coor.x / vg.grid_cell_size) * vg.grid_cell_size;
    let y = (coor.y / vg.grid_cell_size) * vg.grid_cell_size;

    Coord::from((x,y))
}


struct GolGrid {
    grid:[bool; 100_000_000], //10_000x10_000 = 10_000^2
}

impl GolGrid {
    fn get_val(&self, coord:&Coord) -> bool{
        let index:usize = GolGrid::get_index(coord);

        self.grid[index]
    }

    fn complement_val(&mut self, coord:&Coord){
        let index:usize = GolGrid::get_index(coord);
        self.grid[index] = !self.grid[index];
    }

    fn get_index(coord:&Coord) -> usize{
        let index:usize = (coord.x + coord.y*10_000).try_into().unwrap();

        index
    }

    fn get_coord(index:&usize) -> Coord{
        return Coord::from((
            <usize as TryInto<i32>>::try_into(index % 10_000).unwrap(),
            <usize as TryInto<i32>>::try_into(index / 10_1000).unwrap()
        ))
    }

    fn new() -> Self{
        Self{
            grid: [false; 100_000_000]
        }
    }

    

    fn simulate(&self) -> GolGrid{
        let mut dup = GolGrid::new();

        let mut curr_coor= Coord::from((0,0));
        for (i,val) in self.grid.iter().enumerate(){
            curr_coor = GolGrid::get_coord(&i);

            let mut num_neigh = 0;

            for neighbour in curr_coor.get_neighbours(){
                if self.get_val(&neighbour){ //there is a alive neighbour
                    num_neigh += 1;
                }
            }

            if *val{ //current cell is alive
                match num_neigh{
                    2 | 3 => dup.grid[i] = true,
                    _     => dup.grid[i] = false,
                }
            } else{ //current cell is dead
                match num_neigh {
                    3 => dup.grid[i] = true,
                    _ => dup.grid[i] = false,         
                }
            }

            
        }

        return dup
    }
    
}

pub fn main() {

    let mut vg:VisualGrid = VisualGrid::from( 36, 29, 23 );


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("etude-gol", vg.window_width.try_into().unwrap(), vg.window_height.try_into().unwrap())
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(VisualGrid::BLACK);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    

    let mut running: bool = false;


    let mut grid_cursor: Rect = Rect::from(&vg);
    let mut grid_cursor_ghost: Rect = Rect::from(&vg);


    let mut mygrid = GolGrid::new();


    'running: loop {
        canvas.set_draw_color(VisualGrid::BLACK);
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
                    vg.grid_cell_size += 2;
                    vg.normalise_coord();
                    
                    normalise_coord(&mut grid_cursor, &vg);
                    normalise_coord(&mut grid_cursor_ghost, &vg);
                },
                Event::KeyDown { keycode: Some(Keycode::Minus), .. } => {
                    if vg.grid_cell_size > 3 {
                        vg.grid_cell_size -= 2;
                        vg.normalise_coord();
                        
                        normalise_coord(&mut grid_cursor, &vg);
                        normalise_coord(&mut grid_cursor_ghost, &vg);
                    }
                },

                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    grid_cursor.y += vg.grid_cell_size;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    grid_cursor.y -= vg.grid_cell_size;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    grid_cursor.x -= vg.grid_cell_size;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    grid_cursor.x += vg.grid_cell_size;
                },

                Event::MouseMotion {x, y, ..} => {
                    snap_to_closest(&mut grid_cursor_ghost, &vg, x, y);
                }
                Event::MouseButtonDown {x, y, ..} => {
                    snap_to_closest(&mut grid_cursor, &vg, x, y);
                }


                Event::KeyDown { keycode: Some(Keycode::SPACE), .. } => {
                    let mut coor = get_snap_coord(&Coord::from((grid_cursor.x,grid_cursor.y)), &vg);
                    coor.add(&vg.curr_coord);

                    mygrid.complement_val(&coor);
                },
                _ => {}
            }
        }
        // The rest of the loop goes here..

        if running{
            let mygrid = mygrid.simulate(); //shadow old grid
        }

        canvas.set_draw_color(VisualGrid::GRAY);
        //vertical lines
        /*
        for x in (0..1 + vg.grid_width * vg.grid_cell_size).step_by(vg.grid_cell_size.try_into().unwrap()){
            let _ = canvas.draw_line(Point::new(x, 0), Point::new(x, vg.window_height));
        }
        //horizontal lines
        for y in (0..1 + vg.grid_height * vg.grid_cell_size).step_by(vg.grid_cell_size.try_into().unwrap()){
            let _ = canvas.draw_line(Point::new(0, y), Point::new(vg.window_width, y));
        }
        */

        for x in (0..vg.grid_width * vg.grid_cell_size).step_by(vg.grid_cell_size.try_into().unwrap()){
            for y in (0..1 + vg.grid_height * vg.grid_cell_size).step_by(vg.grid_cell_size.try_into().unwrap()){
                let mut coor = get_snap_coord(&Coord::from((x,y)), &vg);
                coor.add(&vg.curr_coord);

                let rec = Rect::new(x, y, vg.grid_cell_size.try_into().unwrap(), vg.grid_cell_size.try_into().unwrap());

                match mygrid.get_val(&coor){
                    true  =>{
                        canvas.set_draw_color(VisualGrid::WHITE);
                        let _ = canvas.fill_rect(rec);
                        canvas.set_draw_color(VisualGrid::GRAY);
                    }
                    false =>()
                }
            }
        }


        canvas.set_draw_color(VisualGrid::WHITE);
        let _ = canvas.draw_rect(grid_cursor);


        canvas.set_draw_color(VisualGrid::YELLOW);
        let _ = canvas.draw_rect(grid_cursor_ghost);


        
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}