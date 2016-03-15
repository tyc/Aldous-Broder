extern crate rand;

use rand::Rng;

// trying to implemented a maze generating algorithm in
// rust. 
//
// I am following the details from the website of 
// http://weblog.jamisbuck.org/2011/1/17/maze-generation-aldous-broder-algorithm
//

struct Cell {
	visited: u8,  // 0x01 = visited

    // bitfields 0b0000NSEW
	carve: u8,    
}



impl Copy for Cell {}
impl Clone for Cell { fn clone(&self) -> Cell {*self}}

fn calculate_vector_position(x :i32, y: i32, size: usize)->i32 {
	
	let ret_value:i32;
	
	ret_value = (x * size as i32) + y;
	
	return ret_value;
}


fn main() {

    // define some constants that determine the size of the grid
    const WIDTH : i32 = 5;
    const HEIGHT : i32 = 5;

	const GRID_SIZE :usize = WIDTH as usize * HEIGHT as usize;
   
    
    let mut cell_remaining : i32 = (GRID_SIZE as i32) - 1;

	let mut vec_pos : usize;
	
	let mut cell = vec![Cell{visited:0x00, carve:0x00}; (GRID_SIZE)];
	
    // setup up the first cell via  random selection.
    //
    let mut x_pos = rand::thread_rng().gen_range(1,WIDTH);
    let mut y_pos = rand::thread_rng().gen_range(1,HEIGHT);

    println!("starting position {} {}", x_pos, y_pos);

	vec_pos = calculate_vector_position(x_pos, y_pos, GRID_SIZE) as usize;
   
    // cell[vec_pos].visited = 0x01;    
    
    
    while cell_remaining > 0 {

        // get the next step to take.
        let direction_shuffle = rand::thread_rng().gen_range(1,4);

        println!("direciton_shuffle {}", direction_shuffle);

        match direction_shuffle {

            // direction is NORTH
            1 => {
                if (y_pos < HEIGHT) {
                    y_pos += 1;    
                    println!("direction is North, position {} {}", x_pos, y_pos);
                }
            },

            // direction is EAST
            2 => {
                if (x_pos < WIDTH) {
                    x_pos += 1;    
                    println!("direction is East, position {} {}", x_pos, y_pos);
                }
            },

            // direction is South
            3 => {
                if (y_pos > 0) {
                    y_pos -= 1;    
                    println!("direction is South, position {} {}", x_pos, y_pos);
                }
            }

            // direction is West
            4 => {
                if (x_pos > 0) {
                    x_pos -= 1;    
                    println!("direction is West, position {} {}", x_pos, y_pos);
                }
            }

            _ => println!("anything else!"),
        }

        println!("cell remaining = {}", cell_remaining);
        cell_remaining -= 1;
    
    }
}

