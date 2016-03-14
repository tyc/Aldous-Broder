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
	let i32_size:i32;
	
	i32_size = size as i32;
	
	ret_value = (x * i32_size) + y;
	
	return ret_value;
}


fn main() {
	
	const GRID_SIZE :usize = 10;
    
    let mut grid : [Cell; 2] = 
        [
            Cell{visited:0x01, carve:0x01},
            Cell{visited:0x01, carve:0x01},
         ];
	
    let mut grid1 :Cell;

    // let mut num_grid :[i32; GRID_SIZE] = [ 0; GRID_SIZE];

    
    // define a 100 cell array but it uses the Copy and Clone
    // implementation. It is initialised with the value of 0x01 and 0x01
    // for both visited and carve.
    let mut grid2 : [Cell; GRID_SIZE] = [ Cell{visited:0x01, carve:0x01}; GRID_SIZE ];


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

	let mut vec_pos : usize;
	
	let mut xs = vec![Cell{visited:0x01, carve:0x01}; (GRID_SIZE)];
	
	// The `len` method yields the current size of the vector
    println!("Vector size: {}", xs.len());

    // Indexing is done using the square brackets (indexing starts at 0)
    println!("Second element: {}", xs[1].visited);

	xs[1].visited = 2;

	vec_pos = calculate_vector_position(0, 1, GRID_SIZE) as usize;
	xs[vec_pos].carve = 3;

   // Indexing is done using the square brackets (indexing starts at 0)
    println!("Second element: {}", xs[1].visited);
   	println!("Second element: {}", xs[1].carve);

	vec_pos = calculate_vector_position(0, 3, GRID_SIZE) as usize;
	xs[vec_pos].carve = 3;



    // `pop` removes the last element from the vector and returns it
    let popped_cell : Cell = Cell{visited:0x01, carve:0x01};
    
    // popped_cell = xs.pop();
    println!("Pop last element: {:?}", popped_cell.visited);

    // Out of bounds indexing yields a panic
    println!("Fourth element: {}", xs[3].visited);
    
    

}
