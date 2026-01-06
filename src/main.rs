// implicit and explicit reference conversion
fn imp_exp_ref() {
	let x = 5;
	let y = &x;
	let z = i32::abs(*y);
	let k = y.abs();

	println!("Implicit: {k}  Explicit: {z}");
}

fn main() {
	imp_exp_ref();
}

