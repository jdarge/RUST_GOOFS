/*

	

*/

// traditional struct
struct Color {
	red: u8,
	green: u8,
	blue: u8
}

// tuple struct
struct ColorTuple(u8, u8, u8);

////////////////////////////////////////////////

// person struct 
struct Person {
	first_name: String, 
	last_name: String
}

impl Person {
	fn new(first: &str, last: &str) -> Person {
		Person {
			first_name: first.to_string(),
			last_name: last.to_string()
		}
	}
	
	// Get full name
	fn full_name(&self) -> String {
		format!("{} {}", self.first_name, self.last_name)
	}
	
	// Set last name
	fn set_last_name(&mut self, last: &str) {
		self.last_name = last.to_string();
	}
	
	// name to tuple 
	fn to_tuple(self) -> (String, String) {
		(self.first_name, self.last_name)
	}
}

pub fn run() {

	let mut c = Color {
		red: 255,
		green: 0, 
		blue: 0
	};
	
	println!("Color: R-{} G-{} B-{}", c.red, c.blue, c.green);
	
	c.blue = 255;
	c.green = 255;
	
	println!("Color: R-{} G-{} B-{}", c.red, c.blue, c.green);
	
	let mut c_t = ColorTuple(255, 0, 0);
	
	println!("Color Tuple: R-{} G-{} B-{}", c_t.0, c_t.1, c_t.2);
	
	c_t.1 = 255;
	c_t.2 = 255;
	
	println!("Color Tuple: R-{} G-{} B-{}", c_t.0, c_t.1, c_t.2);
	
	/////////////////////////////////////////////////////////////
	
	let mut p = Person::new("Sugg", "Who");
	
	println!("Person: {} {}", p.first_name, p.last_name);	
	println!("Person: {}", p.full_name());
	
	p.set_last_name("Deez");
	println!("Person: {}", p.full_name());
	
	println!("Person Tuple: {:?}", p.to_tuple());
	
}





