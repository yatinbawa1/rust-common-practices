use crate::ownership_again;


// Lifetime annotations
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() { x } else { y }
}