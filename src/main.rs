extern crate rand;
#[macro_use] extern crate log;

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

/// The `Option` type. See [the module level documentation](index.html) for more.
const NORTH_CARVE : u8 = !NORTH;
const SOUTH_CARVE : u8 = !SOUTH;
const EAST_CARVE  : u8 = !EAST;
const WEST_CARVE  : u8 = !WEST;

// This the cell structure block. Each cell has thef following attributes
//    visited: if visited is 0x01 otherwise it is 0x00.
//    carve: a bitfield indicate which of its four walls is missing.
//           
struct Cell {
	visited: bool, 	// 0x01 = visited
	carve: u8,		// either North, South, East or West.
}


impl Copy for Cell {}
impl Clone for Cell { fn clone(&self) -> Cell {*self}}

fn calculate_vector_position(x :i32, y: i32, width : i32)->usize {
	
	let ret_value:i32;
	
	ret_value = x + (y * width);

    // println!("calculated vector position = {}", ret_value);

	return ret_value as usize;
}

/// # dump_grid
/// 
/// This will dump the grid in a ASCII fashion out onto stdout.
/// The algorithm will only check the carving on the EAST wall and
/// the carving on the south wall. It uses the natural movement of
/// the cursor from left to right, and then top to down.
/// 
/// Graphically, the bottom left hand corner of the finished 
/// dumped grid has the coordinate (0,0).
///
fn dump_grid(width : i32, height : i32, cell : &Vec<Cell>) {

    let mut x_pos : i32 = 0;
    let mut y_pos : i32;
    let mut done : bool = false;

    while x_pos < width{
        print!(" _");
        x_pos += 1;
    }
    println!("");

    y_pos = height - 1;
    while done != true {

        print!("|");

        for x_pos in 0..width {


            let vec_pos = calculate_vector_position(x_pos, y_pos, width);

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
        

//            println!("{} {}", x_pos, y_pos);
        }
    
        println!("");

        if (x_pos == width) && (y_pos == 0)
        {
            done = true;
        }
        
        if y_pos > 0 
        {
            y_pos -= 1;
        }
    }
}
/// dump out the contents of the grid cell.
/// 
/// # Examples
/// 
///   dump_cell(&cell);
/// 
/// the qwerty contents of cell will be dumped out to stdout.
///
#[cfg(debug="true")]
fn dump_cell(cell : &Vec<Cell>){

    for idx in 0..cell.len() {
        print!("cell[{}] {:?} ", idx, cell[idx].visited );
    
        if (cell[idx].carve & NORTH) != 0 {
            print!("N");
        }
        else
        {
            print!(".");
        }
    
        if (cell[idx].carve & SOUTH) != 0 {
            print!("S");
        }
        else
        {
            print!(".");
        }
        
        if (cell[idx].carve & EAST) != 0 {
            print!("E");
        }
        else
        {
            print!(".");
        }
    
        if (cell[idx].carve & WEST) != 0 {
            print!("W");
        }
        else
        {
            print!(".");
        }

        println!("");
    
    
    }
}


// The main entry point.
// To configure the size of the grid, change the value for const of
// WIDTH and HEIGHT. The width and height does not have to be the same.
//
// todo: update the code so that it can take a command line option.
fn main() {

    // define some constants that determine the size of the grid
    const WIDTH : i32 = 20; 
    const HEIGHT : i32 = 20;

	const GRID_SIZE :usize = WIDTH as usize * HEIGHT as usize;
    
    let mut cell_remaining : i32 = (GRID_SIZE as i32) - 1;

	let mut vec_pos : usize;
	
	let mut cell = vec![Cell{visited:false, carve:(NORTH|SOUTH|EAST|WEST)}; (GRID_SIZE)];
	
    // setup up the first cell via  random selection.
    let mut x_pos = rand::thread_rng().gen_range(0,WIDTH-1);
    let mut y_pos = rand::thread_rng().gen_range(0,HEIGHT-1);

	vec_pos = calculate_vector_position(x_pos, y_pos, WIDTH);
   
    debug!("starting position {} {}, vec {}", x_pos, y_pos, vec_pos);
    cell[vec_pos].visited = true;    

 /*   
  *  if cfg!(debug="true") {
  *  	dump_cell(&cell);
  *  }
  */

    // iterate until all the cells have been visited.
    while cell_remaining != 0 {

        debug!("***********************************");
		
        // get the next step to take.
        let direction_shuffle = rand::thread_rng().gen_range(1,5);

        let old_x_pos = x_pos;
        let old_y_pos = y_pos;

        // check if cell have been visited or not after taken the step.
        // Mark it accordingly if has not been visited before.
        //
        // For the cells that has not been visited, the wall between 
        // the new cell and the old cell is carved.
        // The number of remaining cells are also reduced.
        //
        // The code also checks for the limits fo the grid.
        match direction_shuffle {

            // direction is NORTH
            1 => {
                if y_pos < HEIGHT-1 {
                    
                    y_pos += 1;    
                    vec_pos = calculate_vector_position(x_pos, y_pos, WIDTH);
                    if cell[vec_pos].visited != true {
                        
                        cell[vec_pos].carve &= SOUTH_CARVE; 
                        cell[vec_pos].visited = true;

                        vec_pos = calculate_vector_position(old_x_pos, old_y_pos, WIDTH);
                        
                        cell[vec_pos].carve &= NORTH_CARVE;
            
                        cell_remaining -= 1;
                    }
                }
            },

            // direction is EAST
            2 => {
                if x_pos < WIDTH-1 {
                    x_pos += 1;    
                    vec_pos = calculate_vector_position(x_pos, y_pos, WIDTH);
                    if cell[vec_pos].visited != true {
                        
                        cell[vec_pos].carve &= WEST_CARVE; 
                        cell[vec_pos].visited = true;

                        vec_pos = calculate_vector_position(old_x_pos, old_y_pos, WIDTH);
                        
                        cell[vec_pos].carve &= EAST_CARVE;
            
                        cell_remaining -= 1;
                    }
                }
            },

            // direction is South
            3 => {
                if y_pos > 0 {
                   
                    y_pos -= 1;    
                    vec_pos = calculate_vector_position(x_pos, y_pos, WIDTH);
                    if cell[vec_pos].visited != true {
                        
                        cell[vec_pos].carve &= NORTH_CARVE; 
                        cell[vec_pos].visited = true;

                        vec_pos = calculate_vector_position(old_x_pos, old_y_pos, WIDTH);
                        
                        cell[vec_pos].carve &= SOUTH_CARVE;
            
                        cell_remaining -= 1;
                    }
                }
            }

            // direction is West
            4 => {
                if x_pos > 0 {
                	
                    x_pos -= 1;    
                    vec_pos = calculate_vector_position(x_pos, y_pos, WIDTH);
                    if cell[vec_pos].visited != true {
                        
                        cell[vec_pos].carve &= EAST_CARVE; 
                        cell[vec_pos].visited = true;

                        vec_pos = calculate_vector_position(old_x_pos, old_y_pos, WIDTH);
                        
                        cell[vec_pos].carve &= WEST_CARVE;
            
                        cell_remaining -= 1;
                    }
                }
            }

            _ => { println!("strange direction {}", direction_shuffle) },
        }

        // vec_pos = calculate_vector_position(x_pos, y_pos, WIDTH) as usize;
        // println!("direction is East, position {} {}, vec {}, cell remaining {}", x_pos, y_pos, vec_pos, cell_remaining);

    }

	// The maze is completed, so lets dump it out into the
	// stdout so that it can be enjoyed.
	dump_grid(WIDTH, HEIGHT, &cell);
    //dump_cell(&cell);
}

