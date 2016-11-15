use project_euler::common::primes::generate_primes;

#[no_mangle]
pub extern fn nth_prime(n: usize) -> usize {
	generate_primes()
		.nth(n-1)
		.unwrap_or(0)
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn nth_prime_6() {
		assert_eq!(nth_prime(6), 13);
	}	

	#[test]
	fn nth_prime_10001() {
		assert_eq!(nth_prime(10001), 104743);
	}	

}