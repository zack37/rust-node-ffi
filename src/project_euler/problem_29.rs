use std::collections::HashSet;
use num::bigint::BigUint;
use num::{pow ,FromPrimitive};

#[no_mangle]
pub extern fn distinct_powers(limit: usize) -> usize {
	let mut terms: HashSet<BigUint> = HashSet::new();

	for base in 2..limit+1 {
		for exp in 2..limit+1 {
			let big_base = BigUint::from_usize(base).unwrap();
			terms.insert(pow(big_base, exp));
		}
	}

	terms.len()
}