// how expression in rust are different from other languages
// while {} would be considered object in JS, in rust this just
// returns the last expression
// fn expression_example(val: i32) -> i8 {
//     {
//         // this might overflow, this is
//         // to understand sizes of variables
//         // let val: i8 = val.try_into().expect("The value has overflowed in expression example function");
//
//         // Wrapping results a tuple, wrapping.0 results in
//         // first item of the tuple, which is our Two's Compliment truncated item.
//         // a simple val as i8 would have also worked
//         // Wrapping function is just more explicit
//         // it provides addition, subtraction, multiplication
//         // for the value.
//
//         let i8val: i8 = (Wrapping(val as i8) + Wrapping(val as i8)).0;
//         i8val
//     }
// }

// fn check_age_for_retirement_reached(age: i32) -> bool {
//
//     if age > 65 {
//         println!("The age of retired is exceeded");
//         return true;
//     } else if age < 32 {
//         println!("You are too young to think about retirement!");
//     } else {
//         println!("You are getting there!");
//     }
//
//     // single line return expression
//     false
// }

// square root that is correct till .1
pub fn sqrt(value: f64) -> f64 {

	if value < 0.0 {
		return f64::NAN;
	}

	let mut start = 0.0;
	let mut end = value.max(1.0);

	while (end - start) > 1e-9 {
		let middle = (start + end) / 2.0;
		let potential = middle * middle;

		if potential > value {
			end = middle;
		} else {
			start = middle;
		}
	}

	(start + end) / 2.0
}