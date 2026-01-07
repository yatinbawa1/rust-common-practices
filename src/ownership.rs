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
	num_ref = &mut num[2]; // changing num_ref to a mut reference (also known as unique references)
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