use num::bigint::BigUint;
use num::{FromPrimitive, One, ToPrimitive, Zero};

pub fn factorial(n: usize) -> usize {
	fn inner(n: usize, acc: usize) -> usize {
		if n <= 1 { return acc; }
		inner(n-1, acc * n)
	}
	inner(n, 1)
}

pub fn big_factorial(n: usize) -> BigUint {
	fn fac_inner(n: usize, acc: BigUint) -> BigUint {
		if n <= 1 { return acc; }
		fac_inner(n-1, acc * BigUint::from_usize(n).unwrap())
	}
	fac_inner(n, BigUint::one())
}

pub fn digit_sum(n: BigUint) -> usize {
	fn digit_sum_inner(value: BigUint, sum: BigUint) -> usize {
		if value.is_zero() {
			return sum.to_usize().unwrap_or(0);
		}
		let ten = BigUint::from_usize(10).unwrap();
		digit_sum_inner(
			value.clone() / ten.clone(),
			sum + value.clone() % ten.clone()
		)
	}
	digit_sum_inner(n, BigUint::zero())
}