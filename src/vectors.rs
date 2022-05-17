/*

	sizeable list

*/

pub fn run() {
	// can use the SAME functions as arrays!

	let mut numbers: Vec<i32> = vec![1,2,3,4];
	
	numbers.push(5);
	numbers.push(6);
	
	println!("List: {:?}", numbers);// 123456
	
	numbers.pop();
	
	println!("List: {:?}", numbers);// 12345
	
	for x in numbers.iter() { // 1, 2, 3, 4, 5
		println!("Number: {}", x);
	}
	
	for x in numbers.iter_mut() {// 6, 7, 8, 9, 10
		*x += 5;
		println!("Number: {}", x);
	} // changes stay
	
	println!("List: {:?}", numbers); // 6, 7, 8, 9, 10
}

