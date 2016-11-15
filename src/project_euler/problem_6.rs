#[no_mangle]
pub extern fn sum_square_difference(limit: usize) -> usize {
	// Square of the sum               - Sum of squares
	(1..limit+1).sum::<usize>().pow(2) - (1..limit+1).map(|x| x*x).sum::<usize>()
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn sum_square_difference_below_10() {
		assert_eq!(sum_square_difference(10), 2640);
	}

	#[test]
	fn sum_square_difference_below_100() {
		assert_eq!(sum_square_difference(100), 25164150);
	}

}