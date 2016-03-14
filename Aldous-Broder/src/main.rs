extern crate rand;

use rand::Rng;

// trying to implemented a maze generating algorithm in
// rust. 
//
// I am following the details from the website of 
// http://weblog.jamisbuck.org/2011/1/17/maze-generation-aldous-broder-algorithm
//

struct Cell {
	visited: u8,
	carve: u8, 
}



fn main() {
	
	let mut grid : [Cell; 2] = 
        [
            Cell{visited:0x01, carve:0x01},
            Cell{visited:0x01, carve:0x01},
         ];
	
    let mut grid1 :Cell;

    // The following line complains that a copy implementation is
    // available for Cell.
    let mut grid2 : [Cell; 100] = [ Cell{visited:0x01, carve:0x01}; 100 ];


//	let secret_number = rand::thread_rng().gen_range(1,101);
//		
//	println!("The secret number is : {}", secret_number);

    let width = 10;
    let height = 10;
   
    grid[0].visited = 0x01;
    grid[0].carve = 0x01;

    grid1.visited = 0x01;
    grid1.carve = 0x01;
    
    let x = rand::thread_rng().gen_range(1,width);
    let y = rand::thread_rng().gen_range(1,height);
    
    println!("x is {}", x);
    println!("y is {}", y);
}
