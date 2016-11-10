use num::{Zero};
use num::bigint::BigUint;
use num::{FromPrimitive, ToPrimitive};
use num::pow;

fn power_inner(value: BigUint, sum: BigUint) -> usize {
	if value.is_zero() {
		return sum.to_usize().unwrap_or(0);
	}
	let ten = BigUint::from_usize(10).unwrap();
	power_inner(
		value.clone() / ten.clone(),
		sum + value.clone() % ten.clone()
	)
}

#[no_mangle]
pub extern fn power_digit_sum(exp: usize) -> usize {
	power_inner(pow(BigUint::from_usize(2).unwrap(), exp), BigUint::zero())
}