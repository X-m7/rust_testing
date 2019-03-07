use std::io::Write;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("1. Guessing");
	println!("To exit type anything else");
	print!("Option: ");
	io::stdout().flush()
		.expect("Failed to flush");
	let mut input = String::new();
	io::stdin().read_line(&mut input)
		.expect("Failed to read");
	let input = match input.trim().parse::<char>() { //trim removes whitespace before and after, another way to indicate to parse() the type needed
		Ok(value) => value,
		Err(_) => std::process::exit(0),
	};
	match input {
		'1' => guessing(),
		_ => std::process::exit(0),
	}
}

fn guessing() {
	loop { //infinite
		print!("Input: "); //need flush() afterwards so it is actually printed (in case of line buffering)
		io::stdout().flush()
			.expect("Failed to flush"); //if there is a method that requires that a Result be handled expect() is one way to do so
		let mut input = String::new(); //mut --> mutable
		io::stdin().read_line(&mut input) //&input for immutable reference
			.expect("Failed to read");
		let input: u32 = match input.trim().parse() { //u32 is a data type (unsigned 32bit integer), match checks the Result and actually handles it instead of crashing like expect() (negative also counts as bad, because unsigned)
			Ok(num) => num,
			Err(_) => continue,
		};
		let random_number = rand::thread_rng().gen_range(1,51); //crypto-grade apparently, range includes min and excludes max
		println!("Random Number: {}", random_number);
		match input.cmp(&random_number) {
			Ordering::Less => println!("{} < {}", input, random_number), //comma to end single statement
			Ordering::Greater => println!("{} > {}", input, random_number),
			Ordering::Equal => { //with braces for multiple, if single statement here comma still needed
				println!("{} == {}", input, random_number);
				break;
			}
		}
	}
}
