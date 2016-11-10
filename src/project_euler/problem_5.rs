
#[no_mangle]
pub extern fn smallest_multiple(limit: usize) -> usize {
	(1..)
		.find(|&x| (2..limit+1).all(|y| x % y == 0))
		.unwrap_or(0)
}