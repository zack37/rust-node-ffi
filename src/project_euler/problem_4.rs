fn is_palindrome(target: usize) -> bool {
	fn inner (target: usize, decrementer: usize, res: usize) -> bool {
		if decrementer == 0 {
			return res == target;
		}
		inner(target, decrementer/10, res * 10 + decrementer % 10)
	};
	inner(target, target, 0)

}

#[no_mangle]
pub extern fn largest_palindrome_product(digit_length: u32) -> usize {
	let base: usize = 10;
	let (min, max) = (base.pow(digit_length-1), base.pow(digit_length));

	(min..max).flat_map(|i: usize| -> Vec<(usize, usize)> {
		(min..max).map(|j| (i,j)).collect()
	})
	.map(|(i,j)| i*j)
	.filter(|&product| is_palindrome(product))
	.max()
	.unwrap_or(0)
}