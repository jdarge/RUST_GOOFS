// Vars are immutable by default (CANNOT REASSIGN)
// Rust is block-scoped language

pub fn run() {
	
	// let name = "Chad"; //immutable
	let mut name = "Chad";//mutable
	name = "Sauce Boss";
	
	println!("Who tf is {}??", name);
	
	//////////////////////////////////
	
	// Define constant
	const ID:i32 = 001;
	
	println!("ID: {}", ID);
	
	//////////////////////////////////
	
	// Assign Multiple Vars
	let (my_name, my_age) = ("Jan", "22");
	println!("{} is {}", my_name, my_age);
}
