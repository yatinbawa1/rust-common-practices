fn main() {
    let thesize: usize = 1303993;
    println!("Size of USIZE is: {}", size_of_val(&thesize));

    let val = expression_example(12);
    println!("The value is: {val}");
}

fn expression_example(val: i32) -> i8 {
    {
        let i8val: i8 = val.try_into().expect("Overflow");
        i8val
    }
}