use project_euler::common::primes::generate_primes;

#[no_mangle]
pub extern fn nth_prime(n: usize) -> usize {
	generate_primes()
		.nth(n)
		.unwrap_or(0)
}
#[no_mangle]
pub extern fn check_primes_up_to(n: usize) {
	println!("{}", n);
	let primes: Vec<usize> = generate_primes()
		.take(n)
		.collect();
	println!("{:?}", primes)
}