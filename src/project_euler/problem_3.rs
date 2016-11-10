use project_euler::common::primes::prime_factors;

#[no_mangle]
pub extern fn largest_prime_factor(target: usize) -> usize {
	prime_factors(target)
		.iter()
		.max()
		.map_or(0, |&x| x)
}