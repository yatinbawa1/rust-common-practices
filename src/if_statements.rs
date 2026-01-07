use std::num::Wrapping;

pub fn if_statements() {
	// usize is dependent on
	// operating system architecture
	let the_size: usize = 1303993;
	println!("Size of USIZE is: {}", size_of_val(&the_size));

	let val = expression_example(1998);
	println!("The Double of expression when truncated to i8 is: {val} and it's size is {}", size_of_val(&val));

	// if statement
	// this checks if the number provided is bigger than 65
	println!("You are about to retire: {}", check_age_for_retirement_reached(32));
}

// how expression in rust are different from other languages
// while {} would be considered object in JS, in rust this just
// returns the last expression
fn expression_example(val: i32) -> i8 {
	{
		// this might overflow, this is
		// to understand sizes of variables
		// let val: i8 = val.try_into().expect("The value has overflowed in expression example function");

		// Wrapping results a tuple, wrapping.0 results in
		// first item of the tuple, which is our Two's Compliment truncated item.
		// a simple val as i8 would have also worked
		// Wrapping function is just more explicit
		// it provides addition, subtraction, multiplication
		// for the value.

		let i8val: i8 = (Wrapping(val as i8) + Wrapping(val as i8)).0;
		i8val
	}
}

fn check_age_for_retirement_reached(age: i32) -> bool {

	if age > 65 {
		return true;
	}

	// single line return expression 
	false
}