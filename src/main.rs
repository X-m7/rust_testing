use std::io::Write;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	print!("Input: "); //need flush() afterwards so it is actually printed (in case of line buffering)
	io::stdout().flush()
		.expect("Failed to flush"); //if there is a method that requires that a Result be handled expect() is one way to do so
	let mut input = String::new(); //mut --> mutable
	io::stdin().read_line(&mut input) //&input for immutable reference
		.expect("Failed to read");
	let input: u32 = input.trim().parse() //u32 --> unsigned 32bit integer
		.expect("Number expected");
	let random_number = rand::thread_rng().gen_range(1,51); //crypto-grade apparently, range includes min and excludes max
	println!("Random Number: {}", random_number);
	match input.cmp(&random_number) {
		Ordering::Less => println!("{} < {}", input, random_number),
		Ordering::Greater => println!("{} > {}", input, random_number),
		Ordering::Equal => println!("{} == {}", input, random_number),
	}
}
