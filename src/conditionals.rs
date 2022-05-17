/*

	

*/

pub fn run() {

	let age: u8 = 21;
	let check_id: bool = false;
	let knows_person: bool = true;
	
	if (age >= 21 && check_id) || knows_person {
		println!("Bartender: What would you like to drink?");
	} else if age < 21 && check_id {
		println!("Get the fuck out of here kid!");
	} else {
		println!("Baretender: I'll need to see your ID.'");
	}

	// shorthand if
	let is_of_age = if age >= 21 {true} else {false};
	println!("Is of age: {}", is_of_age);

}
