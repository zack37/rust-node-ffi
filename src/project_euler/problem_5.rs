
#[no_mangle]
pub extern fn smallest_multiple(limit: usize) -> usize {
	(1..)
		.find(|&x| (2..limit+1).all(|y| x % y == 0))
		.unwrap_or(0)
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn smallest_multiple_below_10() {
		assert_eq!(smallest_multiple(10), 2520);
	}

	#[test]
	fn smallest_multiple_below_20() {
		assert_eq!(smallest_multiple(20), 232792560);
	}

}