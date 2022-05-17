/*
	
	t o o p u l l z
	
	Tuples group together values, can be different types
	MAX IS 12 ELEMENTS	
	
*/

pub fn run() {

	let person: (&str, &str, i8) = ("Jan", "Orlando", 22);
	
	println!("{} is from {} and is {} years old.", person.0, person.1, person.2);

}
