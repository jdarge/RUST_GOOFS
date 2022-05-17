pub fn run() {
	// Print to console
	println!("Hello from the print.rs file!");
	
	println!("Number: {}", 69420); // string literal example
	
	// Positional Arg
	println!("{0} one and {0} two", "Test"); // "Test one and Test two"
	
	// Named args
	println!("{name} likes to play {activity}", name = "Jan", activity = "with himself UwU");
	
	// Placeholder tratis
	println!("Binary: {:b} | Hex: {:x} | Octal: {:o}", 10,10,10);
	
	// Placeholder for debug trait
	println!("{:?}", (666, true, "yeet"));
	
	// Basic math
	println!("10 + 10 = {}", 10 + 10);
}
