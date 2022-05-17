/*

	Enums are types w/ definate values

*/

enum Movement {
	// variants
	Up, 
	Down, 
	Left,
	Right
}

fn move_avatar(m: Movement) {
	// perform action depending on info
	match m {
		Movement::Up => println!("Avatar: UP"),
		Movement::Down => println!("Avatar: DOWN"),
		Movement::Left => println!("Avatar: LEFT"),
		Movement::Right => println!("Avatar: RIGHT")
	}
}

pub fn run() {

	let avatar1 = Movement::Left;
	let avatar2 = Movement::Right;
	let avatar3 = Movement::Up;
	let avatar4 = Movement::Down;
	
	move_avatar(avatar1);
	move_avatar(avatar2);
	move_avatar(avatar3);
	move_avatar(avatar4);
}





