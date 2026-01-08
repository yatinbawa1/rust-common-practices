#![allow(unused)]
#![allow(irrefutable_let_patterns)]

use ordinal::ToOrdinal as _;
mod ownership;
mod float_sqrt;
mod if_statements;

fn main() {
	// ownership::enums();
	// ownership::imp_exp_ref();
	// ownership::ref_pointers();
	let mut array = [132,1,43,53,26,74,9];
	array.sort();
	println!("Original Array: {:?}",array);
	let search_values: &mut [i32] = &mut array;
	let where_is_it = ownership::binary_search_array_split(search_values,74,0);
	if(where_is_it == -1) {
		println!("Cannot find the item!");
	}else {
		println!("Found the item at {} place!",where_is_it.to_ordinal_string());
	}
}

