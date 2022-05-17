/*

	Fixed list, same data types

*/

// use std::mem; // WE CAN BRING IN A LIBRARY LIKE THIS

pub fn run() {

	let numbers: [i32; 5] = [1,2,3,4,5]; // i32 = type, 5 = length
	// let numbers: [i32; 5] = [1,2,3,4]; // THIS WILL GEN AN ERROR
	
	// print entire list
	println!("List: {:?}", numbers);
	
	// print single val
	println!("Single at index 0: {:?}", numbers[0]);
	
	
	// reassign an index value
	let mut numberz: [i32; 5] = [1,2,3,4,5]; // must be mutable
	numberz[0] = 69;
	
	// array length = numbers.len()
	// amount of array mem in bytes = std::mem::size_of_val(&numbers);
	
	// get slice
	let slice: &[i32] = &numbers[1..3];
	println!("Slice: {:?}", slice);

}

