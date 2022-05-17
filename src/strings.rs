/*

Primitive str = Immutable
String = growable

*/

pub fn run() {

	let hello = "Hello"; // immutable
	let mut hello_2 = String::from("Hello,"); // mutable
	
	// get length (works for both types)
	println!("Length: {}", hello.len());
	
	hello_2.push(' '); // adds a space
	println!("{}", hello_2);
	
	hello_2.push_str("fuck you!");
	println!("{}", hello_2);
	
	// get capacity in bytes
	println!("Capacity: {}", hello_2.capacity());
	
	// get contains in bool
	println!("Contains 'fuck': {}", hello_2.contains("fuck"));
	
	// replace
	println!("Replace: {}", hello_2.replace("!", "!!!"));
	
	// loop through string by whitespace
	for token in hello_2.split_whitespace() {
		println!("{}", token);
	}
	
	// create string w/ a capacity
	let mut s = String::with_capacity(10);
	s.push('a');
	s.push('b');
	
	// assertion
	assert_eq!(2, s.len());
	assert_eq!(3, s.len()); // will gen fail log in console
}

