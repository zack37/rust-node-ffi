#[no_mangle]
pub extern fn sum_square_difference(limit: usize) -> usize {
	// Square of the sum               - Sum of squares
	(1..limit+1).sum::<usize>().pow(2) - (1..limit+1).map(|x| x*x).sum::<usize>()
}