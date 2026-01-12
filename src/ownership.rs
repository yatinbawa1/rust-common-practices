use crate::ownership;

use ordinal::ToOrdinal as _;
pub fn ref_pointers() {
	// rust prevents aliasing and mutation at same time
	// It can be a problem because One variable can “pull the rug out” from another variable in many ways, for example
	// By deallocating the aliased data, leaving the other variable to point to deallocated memory.
	// By mutating the aliased data, invalidating runtime properties expected by the other variable.
	// By concurrently mutating the aliased data, causing a data race with nondeterministic behavior for the other variable.

	let mut num = vec![1,2,3]; // created a num with [R,W,O] Permissions
	let mut num_ref =  &num[1]; // Create a num_ref with [R,W,O] Permissions, num loses write and own permissions
	// num.push(5); This would not work because permission has been moved
	// *num_ref = 25; cannot do that because &num[1] is not assigned as mutable, even if _num_ref is mut.
	println!("Borrowed num_ref 1: {}",num_ref);
	println!("Before Changing Anything: {:#?}", &num);
	num_ref = &mut num[2]; // changing num_ref to a mut reference (also known as unique references), num loses all permissions
	// println!("Will not be able to print: {:#?}", num);
	println!("Borrowed num_ref 2: {}",num_ref);
	// *num_ref = 4; -> this would not work because num_ref is still immutable i32, assignment works because of coercion
	// coercion: rust allow some values to be downgraded &mut T -> &T, because it is always safe

	let num_ref: &mut i32 = &mut num[2]; // shadowing
	*num_ref = 4;

	println!("Before Pushing: {:#?}",&num);
	num.push(5); // this works because _num_ref and num_ref2 have no more usage
	// hence permissions of [R,W,O] Have been returned to num
	println!("After Pushing: {:#?}",&num);

}

pub fn imp_exp_ref() {
	let x = 5;
	let y = &x;
	let z = i32::abs(*y);
	let k = y.abs();
	// test
	assert_eq!(z, k);

	println!("Implicit: {k}  Explicit: {z}");
}

// rust want to compile a program
// that requires as little runtime checks as possible
// about 70% vulnerabilities in lower level systems are caused by memory corruption

pub fn enums()  {
	// enums
	enum Pet {
		Dog(usize),
		Cat(usize),
	}

	enum PetName {
		Dog(String),
	}

	let pet1 = Pet::Cat(22);
	let pet2 = Pet::Dog(3);

	// pattern-matching
	// This is the way to extract / destruct value from a
	// enum based on its kind

	let Pet::Cat(age) = pet1 else {
		// this is not a set value of age
		// as whatever is inside else type of
		// situation
		// This is more like error handler
		// whatever happens inside else should block further
		// execution of code
		return;
	};
	println!("{age}");

	let Pet::Dog(age2) = pet2 else {
		return;
	};
	println!("{age2}");

	let Pet::Cat(age4) = pet1 else {
		return;
	};
	println!("{age4}");

	let brownie = PetName::Dog("brownie".to_string());
	let PetName::Dog(name1) = brownie else {
		return;
	};

	// This Particular will not work
	// because value this destructuring does not
	// use Copy like usize, so it actually moves
	// value ownership from brownie to name, trying
	// to access brownie again is not possible as
	// it does not contain the value
	// let PetName::Dog(name2) = brownie else {
	//     return;
	// };

	// Allocating Memory
	// the data *moves* from owner to newOwner
	let new_owner = allocating_heap(5);
	println!("{:?}", new_owner);

	function_reference();
	function_reference_using_reference();

	// this will not print because before this
	// the program would have returned because of wrong pattern-matching
	let Pet::Cat(age3) = pet2 else {
		panic!("This is not a Cat!");
	};

	println!("{age3}");

}

fn allocating_heap(size: usize) -> Box<Vec<usize>> {

	// a piece of memory can be
	// allocated in heap using Box::New()

	let array_in_stock: Vec<usize> = (0..size).collect();
	let owner = Box::new(array_in_stock);

	// moving ownership of box

	owner
}

// References
// It would be inconvenient to always reassign
// a variable that does not implement copy after passing
// it to a program
fn function_reference(){
	let value1 = String::from("Hello");
	let value2 = function_reference_passing(value1);
	// The following line would not work because value1 has been moved
	// println!("value1 = {}. value2 = {}", value1, value2);
}

fn function_reference_passing(mut value: String) -> String {
	value.push_str(" World!");
	value
}

fn function_reference_using_reference() {
	let mut value = String::from("Hello");
	function_reference_using_reference_passing(&mut value);
	println!("{}", value);
}

fn function_reference_using_reference_passing(value: &mut String)
{
	value.push_str(" World!");
}


// flow permission is given when a fn has input like &strings[0]
// or output like first
// it does not change throughout function

fn f_permission(strings: &Vec<String>) -> &String {
	let first = &strings[0];
	first
}

// This will not compile because the strings and default lack flow permissions
// fn first_or<'a, 'b, 'c>(strings: &'a Vec<String>, default: &'b String) -> &'c String {
// 	if strings.len() > 0 {
// 		&strings[0]
// 	} else {
// 		default
// 	}
// }


// Arrays in Rust have only 1 permission for all element -> [_]. If one of them loses them, all of them
// loses them for times like these, rust provides Standard Library functions
// that work around these borrower barriers

pub fn run_binary_search() {
	let mut array = [132,1,43,53,26,74,9];
	array.sort();
	println!("Original Array: {:?}",array);
	let search_values: &mut [i32] = &mut array;
	let where_is_it = binary_search_array_split(search_values,74,0);
	if(where_is_it == -1) {
		println!("Cannot find the item!");
	}else {
		println!("Found the item at {} place!",where_is_it.to_ordinal_string());
	}
}
// TODO: Fix this search
fn binary_search_array_split(search_values: &mut [i32], search_term: i32, index: usize) -> i32 {


	let len = search_values.len() - 1;
	let mid_idx = len / 2;
	let middle_value  =  search_values[mid_idx];

	if(len == 0){
		if (middle_value == search_term){
			return (mid_idx + index) as i32;
		}
		return -1;
	}

	// this is how you split an array to use multiple
	// index references.
	let (array_low, array_high) = search_values.split_at_mut(mid_idx + 1);

	let mut a_low = &mut &array_low[0];
	let mut a_high = &mut &array_high[array_high.len() - 1];
	println!("Highest Point this cycle is: {a_high} Lowest Point this cycle is: {a_low}");

	if middle_value == search_term {
		(mid_idx + index) as i32
	} else if (middle_value > search_term) {
		binary_search_array_split(array_low, search_term, index)
	} else {
		binary_search_array_split(array_high, search_term, mid_idx + index + 1)
	}
}