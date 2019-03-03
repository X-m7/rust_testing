use std::io::Write;
use std::io;

fn main() {
    println!("Hello, world!");

	print!("Input: ");
	io::stdout().flush();
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read");
	
	println!("Echoed Input: {}", input);
}
