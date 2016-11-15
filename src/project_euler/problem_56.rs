use num::bigint::BigUint;
use num::{FromPrimitive, pow};
use project_euler::common::math::digit_sum;

#[no_mangle]
pub extern fn powerful_digit_sum() -> usize {
	(1..100)
		.flat_map(|a|
			(1..100).map(move |b| pow(BigUint::from_usize(a).unwrap(), b))
		)
		.map(digit_sum)
		.max()
		.unwrap_or(0)

	// ================================================================ //
	// The code below is functionaly equivalent to the code above, but
	// is a little more concise and easier to modify if needed.
	// Performance is similar (~3s)
	// ================================================================ //
	// let mut max_sum = 0_usize;
	// for a in 1..100 {
	// 	for b in 1..100 {
	// 		let p = pow(BigUint::from_usize(a).unwrap(), b);
	// 		let ds = digit_sum(p);
	// 		if ds > max_sum {
	// 			max_sum = ds;
	// 		}
	// 	}
	// }

	// max_sum
}