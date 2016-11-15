use project_euler::common::primes::prime_factors;

#[no_mangle]
pub extern fn largest_prime_factor(target: usize) -> usize {
	prime_factors(target)
		.iter()
		.max()
		.map_or(0, |&x| x)
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn largest_prime_factor_of_13195() {
		assert_eq!(largest_prime_factor(13195), 29);
	}

	#[test]
	fn largest_prime_factor_of_600851475143() {
		assert_eq!(largest_prime_factor(600851475143), 6857);
	}

}