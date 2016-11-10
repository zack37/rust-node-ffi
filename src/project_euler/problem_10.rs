use project_euler::common::primes::generate_primes;

#[no_mangle]
pub extern fn summation_of_primes(limit: usize) -> usize {
	generate_primes()
		.take_while(|&x| x < limit)
		.sum()
}