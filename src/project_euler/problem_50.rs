use project_euler::common::primes::{generate_primes, is_prime};

#[no_mangle]
pub extern fn consecutive_prime_sum(limit: usize) -> usize {
	let result = (0..limit)
		.flat_map(|x| (1..limit).map(move |y| generate_primes().skip(x).take(y).collect::<Vec<usize>>()))
		// [ [2], [2, 3], [2, 3, 5], ..., [3], [3, 5], [3, 5, 7], ... ]
		.filter(|x| {
			let sum = x.iter().sum();
			sum < limit && is_prime(sum)
		})
		.max_by_key(|x| x.len());
	println!("{:?}", result);
	0
}