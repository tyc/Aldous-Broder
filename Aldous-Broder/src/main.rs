extern crate rand;

use rand::Rng;

// trying to implemented a maze generating algorithm in
// rust. 
//
// I am following the details from the website of 
// http://weblog.jamisbuck.org/2011/1/17/maze-generation-aldous-broder-algorithm
//

struct Cell {
	visted: bool,
	carve: u8, 
}



fn main() {
	
	let mut grid:[i8; 5] : Cell;
	
//	let secret_number = rand::thread_rng().gen_range(1,101);
//		
//	println!("The secret number is : {}", secret_number);

    let width = 10;
    let height = 10;
    
    grid[0].visted = true;
    grid[0].carve = 0x01;
    
    let x = rand::thread_rng().gen_range(1,width);
    let y = rand::thread_rng().gen_range(1,height);
    
    println!("x is {}", x);
    println!("y is {}", y);
}
