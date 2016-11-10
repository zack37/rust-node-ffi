#[no_mangle]
pub extern fn special_pythagorean_triplet() -> usize {
	let sum = 1000;
	(1..sum/3+1).flat_map(|a| {
		(a+1..sum/2+1).map(|b| {
			let c = sum - a - b;
			(a, b, c)
		}).collect::<Vec<(usize, usize, usize)>>()
	})
	.find(|&(a, b, c)| a*a + b*b == c*c)
	.map_or(0, |(a, b, c)| a*b*c)
}