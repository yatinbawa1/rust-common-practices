#![allow(unused)]
#![allow(irrefutable_let_patterns)]

mod ownership;
mod float_sqrt;
mod if_statements;
mod ownership_again;

fn main() {
	// ownership::enums();
	// ownership::imp_exp_ref();
	// ownership::ref_pointers();
	let x = String::from("hello1");
	let y = String::from("world");

	let z = ownership_again::longest(&x,&y);
	println!("{}",z);
}

