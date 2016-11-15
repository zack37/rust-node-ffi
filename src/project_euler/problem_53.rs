use num::bigint::BigUint;
use num::FromPrimitive;
use project_euler::common::math::big_factorial;

fn choose(n: usize, r: usize) -> BigUint {
	big_factorial(n) / (big_factorial(r)*big_factorial(n - r))
}

#[no_mangle]
pub extern fn combinatoric_selections(lower_limit: usize) -> usize {
	let ref ll = BigUint::from_usize(lower_limit).unwrap();
	(1..101)
		.flat_map(|n| (0..n).map(move |r| choose(n, r)))
		.filter(|x| x > ll)
		.count()
}