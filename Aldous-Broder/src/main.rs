extern crate rand;

use rand::Rng;

// trying to implemented a maze generating algorithm in
// rust. 
//
// I am following the details from the website of 
// http://weblog.jamisbuck.org/2011/1/17/maze-generation-aldous-broder-algorithm
//

// defines some const for wall identification
const NORTH : u8 = (1<<3);
const SOUTH : u8 = (1<<2);
const EAST  : u8 = (1<<1);
const WEST  : u8 = (1<<0);


const NORTH_CARVE : u8 = !NORTH;
const SOUTH_CARVE : u8 = !SOUTH;
const EAST_CARVE  : u8 = !EAST;
const WEST_CARVE  : u8 = !WEST;

// This the cell structure block. Each cell has the following attributes
//    visited: if visited is 0x01 otherwise it is 0x00.
//    carve: a bitfield indicate which of its four walls is missing.
//           
struct Cell {
	visited: u8,  	// 0x01 = visited
	carve: u8,		// either North, South, East or West.
}


impl Copy for Cell {}
impl Clone for Cell { fn clone(&self) -> Cell {*self}}

fn calculate_vector_position(x :i32, y: i32, width : i32)->i32 {
	
	let ret_value:i32;
	
	ret_value = x + (y * width);

    // println!("calculated vector position = {}", ret_value);

	return ret_value;
}

fn dump_grid(width : i32, height : i32, cell : Vec<Cell>) {

    let mut vec_pos : usize = 0;
    let mut x_pos : i32 = 0;

    while x_pos < width{
        print!(" _");
        x_pos += 1;
    }
    println!("");

    for y_pos in 0..height {
        print!("|");
        for x_pos in 0..width {
            vec_pos = calculate_vector_position(x_pos, y_pos, width) as usize;

            // check the south wall
            if (cell[vec_pos].carve & SOUTH) != 0 {
                print!("_");
            }
            else
            {
                print!(" ");
            }

            // check the east wall
            if (cell[vec_pos].carve & EAST) != 0 {
                print!("|");
            }
            else
            {
                print!(" ");
            }
        }
        println!("");
    }
    
}



fn main() {

    // define some constants that determine the size of the grid
    const WIDTH : i32 = 10;
    const HEIGHT : i32 = 20;

	const GRID_SIZE :usize = WIDTH as usize * HEIGHT as usize;
    
    let mut cell_remaining : i32 = (GRID_SIZE as i32) - 1;

	let mut vec_pos : usize;
	
	let mut cell = vec![Cell{visited:0x00, carve:(NORTH|SOUTH|EAST|WEST)}; (GRID_SIZE)];
	
    // setup up the first cell via  random selection.
    //
    let mut x_pos = rand::thread_rng().gen_range(0,WIDTH-1);
    let mut y_pos = rand::thread_rng().gen_range(0,HEIGHT-1);

    println!("starting position {} {}", x_pos, y_pos);

	vec_pos = calculate_vector_position(x_pos, y_pos, WIDTH) as usize;
   
    cell[vec_pos].visited = 0x01;    
    
    dump_grid(WIDTH, HEIGHT, cell);
    
    while cell_remaining != 0 {

        // get the next step to take.
        let direction_shuffle = rand::thread_rng().gen_range(1,5);

//        println!("direciton_shuffle {}", direction_shuffle);

    // bitfields 0b0000NSEW.
        match direction_shuffle {

            // direction is NORTH
            1 => {
                if y_pos < HEIGHT-1 {
                    
                    vec_pos = calculate_vector_position(x_pos, y_pos, WIDTH) as usize;
                    cell[vec_pos].carve &= NORTH_CARVE;
                    
                    y_pos += 1;    
                    
                    // carve the current cell
                    vec_pos = calculate_vector_position(x_pos, y_pos, WIDTH) as usize;
                    cell[vec_pos].carve &= SOUTH_CARVE; 

                    println!("direction is North, position {} {}", x_pos, y_pos);
                }
            },

            // direction is EAST
            2 => {
                if x_pos < WIDTH-1 {

                    vec_pos = calculate_vector_position(x_pos, y_pos, WIDTH) as usize;
                    cell[vec_pos].carve &= EAST_CARVE;

                    x_pos += 1;    

                    vec_pos = calculate_vector_position(x_pos, y_pos, WIDTH) as usize;
                    cell[vec_pos].carve &= WEST_CARVE;

                    println!("direction is East, position {} {}", x_pos, y_pos);
                }
            },

            // direction is South
            3 => {
                if y_pos > 0 {

                    vec_pos = calculate_vector_position(x_pos, y_pos, WIDTH) as usize;
                    cell[vec_pos].carve &= SOUTH_CARVE;

                    y_pos -= 1;    
                    
                    vec_pos = calculate_vector_position(x_pos, y_pos, WIDTH) as usize;
                    cell[vec_pos].carve &= NORTH_CARVE;

                    
                    println!("direction is South, position {} {}", x_pos, y_pos);
                }
            }

            // direction is West
            4 => {
                if x_pos > 0 {
                	
                    vec_pos = calculate_vector_position(x_pos, y_pos, WIDTH) as usize;
                    cell[vec_pos].carve &= WEST_CARVE;

                    x_pos -= 1;    

                    vec_pos = calculate_vector_position(x_pos, y_pos, WIDTH) as usize;
                    cell[vec_pos].carve &= EAST_CARVE;

                    println!("direction is West, position {} {}", x_pos, y_pos);
                }
            }

            _ => {},
        }

        vec_pos = calculate_vector_position(x_pos, y_pos, WIDTH) as usize;
        

        // if the cell has not been visited, we mark it as 
        // visited and adjust the carve attribute.
        if cell[vec_pos].visited != 0x01 {

//            println!("cell remaining = {}", cell_remaining);
            cell_remaining -= 1;
        }
        

    }

	dump_grid(WIDTH, HEIGHT, cell);
}

