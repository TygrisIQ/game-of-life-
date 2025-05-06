use std::{thread::sleep, time::Duration};

use macroquad::{miniquad::window::screen_size, prelude::*};

const CELL_SIZE : f32 = 15.0;
struct Cell{
    alive: bool,
    x: f32,
    y: f32,
    next_state: bool
        
}
impl Cell{
    fn kill(&mut self){
        self.alive = false;
    } 
    fn revive(&mut self){
        self.alive = true;
    }
    // fn coordinates(&self) -> (f32, f32){
        // (self.x, self.y)
    // }
    fn change_state(&mut self){
        !self.alive;
    } 
}

struct Grid{
    cells: Vec<Cell>,
    generation : i32
}
impl Grid{
     fn new() -> Self{
        let mut grid : Vec<Cell> = vec![];    
        let mut x : f32= 0.0;
        while x < screen_width(){ 
        let mut y : f32 = 0.0;
            while y < screen_height(){
           let cell = Cell{
               alive: false,
               x : x,
               y : y,
               next_state: false
                };
             y += CELL_SIZE;
             grid.push(cell);
            }
            x += CELL_SIZE;

        }
        
        Grid { cells: grid, generation : 0 }
     }
     //new mesaures screen size and creates a new grid, 
     //draw draws rectangles 
    fn draw(&self){
        
        for cell in &self.cells{
            match cell.alive{ 
            true => {
                draw_rectangle(cell.x, cell.y, CELL_SIZE - 0.5, CELL_SIZE -0.5, LIGHTGRAY);
            },
            false => {

                draw_rectangle_lines(cell.x, cell.y, CELL_SIZE, CELL_SIZE, 1.0, GRAY);
            }
        }
    }}
    
    fn get_cell_by_coordinates(&mut self, x:f32, y:f32) -> Option<&mut Cell >{
         return  self.cells.iter_mut().find(
                |cell| x >= cell.x && x < cell.x + CELL_SIZE &&
                y >= cell.y && y < cell.y + CELL_SIZE
            );

            
    }

fn change(&mut self,x: f32, y:f32){
           if let Some(cell) = self.get_cell_by_coordinates(x,y){ 
            if cell.alive == false{
               cell.revive();
               
            }else {
                cell.kill();
            }
           }else {
               ()
           }
           
    }
    fn count_neighbour(&mut self, x:f32, y:f32,alive:bool) -> bool{
        let count = &self.get_neighbour_coordinates(x, y).iter().filter(|&&alive| alive).count();
         
        match (alive,count){
            (true, 2) | (true, 3) => true,
            (false, 3) => true,
            _ => false
        }
} 
    fn get_neighbour_coordinates(&mut self, x: f32, y:f32) -> Vec<bool>{
        let mut neighbour_cells : Vec<bool> = vec![];
        //neighbour to the right
        if let Some(n1) = self.get_cell_by_coordinates(x + CELL_SIZE, y ){
            neighbour_cells.push(n1.alive);
        }
        //neighbour above
        if let Some(n2) = self.get_cell_by_coordinates(x, y + CELL_SIZE){
            neighbour_cells.push(n2.alive);
        }
        // neighbour right upper corner
        if let Some(n3) = self.get_cell_by_coordinates(x + CELL_SIZE, y +CELL_SIZE){
            neighbour_cells.push(n3.alive);
        }
        // neighbour left upper corner
        if let Some(n4) = self.get_cell_by_coordinates(x - CELL_SIZE, y +CELL_SIZE){
            neighbour_cells.push(n4.alive);
        }
        //neighbour to the left
        if let Some(n5) = self.get_cell_by_coordinates(x - CELL_SIZE, y){
            neighbour_cells.push(n5.alive);
        }
        //neighbour right lower corner
        if let Some(n6) = self.get_cell_by_coordinates(x + CELL_SIZE, y - CELL_SIZE){
            neighbour_cells.push(n6.alive);
        }
        //neighbour left lower corner
        if let Some(n7) = self.get_cell_by_coordinates(x - CELL_SIZE, y - CELL_SIZE){
            neighbour_cells.push(n7.alive);
        }
        //neighbour below
        if let Some(n8) = self.get_cell_by_coordinates(x , y - CELL_SIZE){
            neighbour_cells.push(n8.alive);
        }
        neighbour_cells
    }
    fn update_grid(&mut self){
        let len = self.cells.len();
        let mut should_change: Vec<(f32,f32)> = vec![];
        for i in 0..len{
            let (x,y,_nextstate , alive) = {
                let cell = &self.cells[i];
                (cell.x, cell.y ,cell.next_state, cell.alive)
            };
            let set_next_state = self.count_neighbour(x,y,alive);
            //change the next state
            if set_next_state != alive{
                self.change_nstate(x, y);
            }
           

            if alive != set_next_state{
                should_change.push((x,y));
                dbg!(should_change.len());
            }
        
        }
         should_change.iter().enumerate().for_each(|(_i, tup)| {let (x,y) =  *tup; self.change(x, y); dbg!(x,y);} );
        if self.cells.iter().any(|cell| cell.alive){
                self.generation += 1;
        }
        // self.draw();
    }

    
    
    fn change_nstate(&mut self, x:f32, y:f32){
            if let Some(cell) = self.get_cell_by_coordinates(x, y){
                cell.change_state();
            }
            else{
                dbg!("CANNOT CHANGE STATE!");
            }
    }
}

fn new_grid() -> Grid{
    Grid::new()
}
#[macroquad::main("gameoflife")]
async fn main() {
    let mut running : bool = false;
    let mut sleep_duration = 100; 
    let mut size = screen_size();
    let mut grid : Grid =  Grid::new();
    
    loop{ 
        if screen_width() != size.0 && screen_height() != size.1{
             dbg!("window size changed.....redrawing!");
             grid = new_grid();
             size = screen_size();
        }
        clear_background(BLANK);
        let text = format!("Generation: {}", grid.generation);
        draw_text(&text, 30.0, 30.0, 30.0, RED);
        grid.draw();
        if is_key_pressed(KeyCode::Space){
             running = !running;
             dbg!("STATE IS :", running);
        }

        if is_mouse_button_pressed(MouseButton::Left){
            let pos = mouse_position();
            grid.change(pos.0, pos.1);
        }
        if running {  
            grid.update_grid();
            sleep(Duration::from_millis(sleep_duration));
        }
        // debug/cheat keys 
        if is_key_down(KeyCode::Equal){
            sleep_duration += 100;
            dbg!("updates are at :", sleep_duration, " milli seconds");
        }
        else if is_key_down(KeyCode::Minus) && sleep_duration > 100{
            sleep_duration -= 100;
            dbg!("updates are at : ", sleep_duration, " milli seconds");

        }
        
        next_frame().await;
             }
        }
    





