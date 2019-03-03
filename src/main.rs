use std::io::Write;
use std::io;
use rand::Rng;

fn main() {
    println!("Hello, world!");

	print!("Input: "); //need flush() afterwards so it is actually printed (in case of line buffering)
	io::stdout().flush()
		.expect("Failed to flush"); //if there is a method that requires that a Result be handled expect() is one way to do so
	let mut input = String::new(); //mut --> mutable
	io::stdin().read_line(&mut input) //&input for immutable reference
		.expect("Failed to read");
	println!("Echoed Input: {}", input);
	
	let random_number = rand::thread_rng().gen_range(1,51); //crypto-grade apparently
	print!("Random Number: {}", random_number);
	io::stdout().flush()
		.expect("Failed to flush");
}
