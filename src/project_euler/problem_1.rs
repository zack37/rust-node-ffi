#[no_mangle]
pub extern fn multiples_of_three_and_five(limit: usize) -> usize {
    (1..limit)
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .sum()
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn sum_below_10() {
		assert_eq!(multiples_of_three_and_five(10), 23);
	}

	#[test]
	fn sum_below_1000() {
		assert_eq!(multiples_of_three_and_five(1000), 233168);
	}

}
