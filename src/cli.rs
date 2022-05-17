use std::env;

// IN src/
// $ cargo run some_input_here

pub fn run() {
	
	let args: Vec<String> = env::args().collect();
	let command = args[1].clone();
	let name = "Jan";
	let status = "100%";
	
	// println!("Command: {}", command);
	
	if command == "hello" {
		println!("Hi {}, how are you?", name);
	} else if command == "status" {
		println!("Status: {}", status);
	} else {
		println!("Go fuck yourself.");
	}
}
