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


fn main() {
	
	let grid_size :i32 = 10;
    
    let mut grid : [Cell; 2] = 
        [
            Cell{visited:0x01, carve:0x01},
            Cell{visited:0x01, carve:0x01},
         ];
	
    let mut grid1 :Cell;

    let mut num_grid :[i32; grid_size] = [ 0; grid_size];

    
    // define a 100 cell array but it uses the Copy and Clone
    // implementation. It is initialised with the value of 0x01 and 0x01
    // for both visited and carve.
    let mut grid2 : [Cell; 10] = [ Cell{visited:0x01, carve:0x01}; 10 ];


//	let secret_number = rand::thread_rng().gen_range(1,101);
//		
//	println!("The secret number is : {}", secret_number);

    let width = 10;
    let height = 10;
   
    grid[0].visited = 0x01;
    grid[0].carve = 0x01;

    grid2[0].visited = 0x01;
    grid2[0].carve = 0x01;
    
    grid1.visited = 0x01;
    grid1.carve = 0x01;
    
    let x = rand::thread_rng().gen_range(1,width);
    let y = rand::thread_rng().gen_range(1,height);
    
    println!("x is {}", x);
    println!("y is {}", y);
}
