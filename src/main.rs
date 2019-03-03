use std::io::Write;
use std::io;

fn main() {
    println!("Hello, world!");

	print!("Input: ");
	io::stdout().flush()
		.expect("Failed to flush"); //if there is a method that requires that a Result be handled expect() is one way to do so
	let mut input = String::new(); //mut --> mutable
	io::stdin().read_line(&mut input) //&input for immutable reference
		.expect("Failed to read");
	println!("Echoed Input: {}", input);
}
