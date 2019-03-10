use std::io::Write;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("1. Guessing");
	println!("2. Enum");
	println!("3. Option Enum");
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
		'2' => enum_test(),
		'3' => option_test(),
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
			Err(_) => continue, //_ means anything
		};
		let random_number = rand::thread_rng().gen_range(1,51); //crypto-grade apparently, range includes min and excludes max
		println!("Random Number: {}", random_number);
		match input.cmp(&random_number) {
			Ordering::Less => println!("{} < {}", input, random_number), //comma to end single statement
			Ordering::Greater => println!("{} > {}", input, random_number),
			Ordering::Equal => { //with braces for multiple
				println!("{} == {}", input, random_number);
				break;
			}
		}
	}
}

enum Message {
	Quit,
	Move {x: i32, y: i32}, //contains struct
	Write(String), //contains tuple
	ChangeColor(i32, i32, i32),
}

impl Message { //add methods to the Message enum, works with structs as well
	fn print_variant(&self, actually_print : bool) {
		if actually_print {
			enum_match(&self);
		}
	}
}

fn enum_test() {
	let move_msg = Message::Move{x: 2, y: 4}; //struct, need to specify names
	let x = 7;
	let y = 9;
	let move_msg2 = Message::Move{x, y}; //here x & y are already defined, shorter way
	let color_msg = Message::ChangeColor(2, 4, 6); //tuple, specify directly
	if let Message::Move{x, y} = move_msg { //names inside {} have to match the ones in the definition (except order)
		println!("First move_msg will have x={} and y={}", x, y);
	}
	if let Message::ChangeColor(_, _, z) = color_msg { //names in () can be anything, even _ if not needed
		println!("Third value of color_msg is {}", z);
	}
	enum_match(&move_msg);
	move_msg2.print_variant(false);
	enum_match(&color_msg);
	enum_match(&Message::Quit);
	enum_match(&Message::Write(String::from("lol"))); //alternatively just construct it in there, no need for variable
}

fn enum_match(msg : &Message) { //& so the enum will not be copied/moved into this function's scope
	match msg {
		Message::Move{x, y} => println!("Move to ({}, {})", x, y),
		Message::Write(x) => println!("{}", x),
		Message::ChangeColor(x, y, z) => println!("Change color to ({}, {}, {})", x, y, z),
		_ => () //empty curly brackets also works here, but this is how the Rust book does it
	}
}

fn option_test() {
	let some_string = Some(String::from("string"));
	let null_string : Option<String> = None;
	print_string(&some_string);
	if !print_string(&null_string) {
		println!("null");
	}
}

fn print_string(value : &Option<String>) -> bool {
	match value {
		Some(x) => {
			println!("{}", x);
			true
		}
		None => false
	}
}