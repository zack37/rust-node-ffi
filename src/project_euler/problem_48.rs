use num::bigint::BigUint;
use num::{FromPrimitive, pow, Zero};

#[no_mangle]
pub extern fn self_powers(limit: usize) -> usize {
	let result = (1..limit+1)
		.fold(BigUint::zero(), |acc, cur| {
			let num = BigUint::from_usize(cur).unwrap_or(BigUint::zero());
			acc + pow(num, cur)
		})
		.to_str_radix(10)
		.chars()
		.rev()
		.take(10)
		.collect::<Vec<_>>();

	println!("Disclaimer: If number is not 10 digits, then it should have a leading zero (0)");

	result.iter().enumerate().fold(0, |acc, (i, x)| {
		acc + x.to_digit(10).unwrap() as usize * (10 as usize).pow(i as u32) as usize
	})
}