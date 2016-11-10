pub struct Fibonacci {
	curr: usize,
	next: usize
}

pub fn generate() -> Fibonacci {
	Fibonacci { curr: 0, next: 1 }
}

impl Iterator for Fibonacci {
	type Item = usize;

	fn next(&mut self) -> Option<usize> {
		let new_next = self.curr + self.next;
		self.curr = self.next;
		self.next = new_next;

		Some(self.curr)
	}
}

// use num::bigint::BigUint;
// use num::{Zero, One};

// pub struct BigFibonacci {
// 	curr: BigUint,
// 	next: BigUint
// }

// pub fn generate_big_fibonacci() -> BigFibonacci {
// 	BigFibonacci { curr: BigUint::zero(), next: BigUint::one() }
// }

// impl Iterator for BigFibonacci {
// 	type Item = BigUint;

// 	fn next(&mut self) -> Option<Self::Item> {
// 		let new_next = self.curr + self.next;
// 		self.curr = self.next;
// 		self.next = new_next;

// 		Some(self.curr)
// 	}
// }