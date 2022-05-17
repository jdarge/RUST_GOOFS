/*

Prim Types:
Int: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
Float: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays // fixed, we have vectors that are adjustable

*/

pub fn run() {

	println!("Max i32: {}", std::i32::MAX);
	println!("Max u32: {}", std::u32::MAX);
	
	let _x = 1;
	let _y: i64 = 69420;
	let is_active: bool = true;
	println!("{:?}", (_x,_y, is_active));
	
	let is_greater = 10 > 5;
	println!("Is 10 > 5? : {:?}", is_greater);
	
	// emoji unicode char example
	let face = '\u{1F600}';
	println!("{}", face);
}
