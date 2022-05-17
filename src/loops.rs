/*

		

*/

pub fn run() {
	let mut count = 0;

	// infinte loop (w/o conditional)
	loop {
		// uncomment println! to see result
		count += 1;
		// println!("Number: {}", count);
		
		if count == 5 {
			break;
		}
	} count = 0;
	
	// while loop (FizzBuzz)
	while count <= 100 {
		// uncomment println! to see result
		if count % 15 == 0 {
			// println!("FizzBuzz");
		} else if count % 3 == 0 {
			// println!("Fizz");
		} else if count % 5 == 0 {
			// println!("Buzz");
		} else {
			// println!("{}", count);
		} count += 1;
	}
	
	// for range (FizzBuzz)
	for x in 0..100 {
		// uncomment println! to see result
		if x % 15 == 0 {
			// println!("FizzBuzz");
		} else if x % 3 == 0 {
			// println!("Fizz");
		} else if x % 5 == 0 {
			// println!("Buzz");
		} else {
			// println!("{}", x);
		}
	}
	
}

