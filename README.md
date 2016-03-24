# Aldous-Broder #

This is an exercise in exploring rust as a language, and for this project, I am attempting to implement the maze generating algorithm of Aldous-Broder. 

I got the main algorithm from the site

http://weblog.jamisbuck.org/2011/1/17/maze-generation-aldous-broder-algorithm

There are probably many things that I could have done better in this implementation. If you also believe so and have some suggestions, please let me know.

# What I learned during this exercise. #

This is my first attempt to learn the rust.

Below are some items that I learned while I was working through the implementation of the Aldous-Broder algorithm. Some of them are rookie items where a seasoned rust developer would do without much sweat.

## Versions ##

This notes are relevant for the following versions.

    C:\Project\rustlang\Aldous-Broder\Aldous-Broder\Aldous-Broder>rustc --version
    rustc 1.6.0 (c30b771ad 2016-01-19)
    
    C:\Project\rustlang\Aldous-Broder\Aldous-Broder\Aldous-Broder>rustdoc --version
    rustdoc 1.6.0 (c30b771ad 2016-01-19)
    
    C:\Project\rustlang\Aldous-Broder\Aldous-Broder\Aldous-Broder>cargo --version
    cargo 0.7.0-nightly (1af03be 2015-12-08)

## rustdoc ##

rust has the ability to generate html documentation of the code but it's default configuration is only limited to public functions and modules. Not 100% sure of the reasoning, but I would have thought that rustdocs should work on all levels. As it stands now, getting it generate documents for private code needs some extra flags.

I have created a [batch file](https://github.com/tyc/Aldous-Broder/blob/master/Aldous-Broder/gen_doc.bat "https://github.com/tyc/Aldous-Broder/blob/master/Aldous-Broder/gen_doc.bat") to generate the documentations.

It uses `cargo doc` to generate the public functions first, and then execute 
`rustdoc` via `cargo` to generate the private functions and modules. These two steps creates a set of documentation that seems to be complete. Being html documentation, the links are useful for navigating through the code.

## ownership and borrowing ##

[https://doc.rust-lang.org/book/ownership.html](https://doc.rust-lang.org/book/ownership.html "https://doc.rust-lang.org/book/ownership.html")

My development background is from embedded systems and C is my language of choice. The concept of ownership and borrowing are not relevant in C unless it is explicitly in your design.

I really like this idea as it forms the core aspect of safety. C, by default, does not promote C memory safety.

This SO entry is good in explaning the concept and how to apply it.

[http://stackoverflow.com/questions/27305585/difference-between-pass-by-reference-and-by-box](http://stackoverflow.com/questions/27305585/difference-between-pass-by-reference-and-by-box "http://stackoverflow.com/questions/27305585/difference-between-pass-by-reference-and-by-box")

## for..loop ##

The for..loop in rust seems to be quite special. In this project, I needed the loop to reverse, ie for a  high number to a lower number. In the end, I had to implemented it using a `while..loop` with an decreasing counter.

After googling, it appears that I need to construct the code as in the following.

    fn main() {
    
    for x in (1..10).rev() {
    	println!("{}", x);
    }

The for..loop function is not as primitve as the C's version. Underneath, it uses an iterator to traverse over the range of values. As it is an iterator, the last value is excluded. So in the above code snippet, it would print from 1 to 9.

## conditional compilation ##

This is broken at the moment as it only works on a module by module basis. If I want to just conditional compile a chunk of code, this is not possible at this stage.

rust uses the `cfg` attribute 

This reddit entry shows some insights, but still to be solved.

[https://www.reddit.com/r/rust/comments/2nlyhl/conditional_compilation/ ](https://www.reddit.com/r/rust/comments/2nlyhl/conditional_compilation/ "https://www.reddit.com/r/rust/comments/2nlyhl/conditional_compilation/") 


## coding style guide checking during compile ##

The coding style guide is also checked when the code is being compiled. I am not 100% sure if this a good idea. However, it only checkes a small subset, more specifically the naming style of the symbols. 

const are named all upper case as in

	 const WIDTH : i32 = 20;    


There are still area for customisation for each project or organisation.

