 pub fn is_prime(target: usize) -> bool {
	match target {
		n if n <= 1 => false,
		n if n <= 3 => true,
		n if n % 2 == 0 || n % 3 == 0 => false,
		n => {
			let mut i = 5;
			while i*i <= n {
				if n % i == 0 || n % (i+2) == 0 {
					return false;
				}
				i = i+6;
			}
			true
		}
	}
}

pub fn prime_factors(target: usize) -> Vec<usize> {
	let sqrt = (target as f64).sqrt();
	(1..sqrt as usize)
		.filter(|&x| target % x == 0 && is_prime(x))
		.collect()
}

pub struct Primes {
	curr: usize
}

impl Iterator for Primes {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {

		self.curr = (self.curr+1..).find(|&x| is_prime(x)).unwrap();

		Some(self.curr)
	}
}

pub fn generate_primes() -> Primes {
	Primes { curr: 0 }
}