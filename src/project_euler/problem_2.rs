use num::Integer;
use project_euler::common::fibonacci_generator::generate;


#[no_mangle]
pub extern fn even_fibonacci_numbers(value_limit: usize) -> usize {
	generate()
		.take_while(|&x| x < value_limit)
		.filter(|&x| x.is_even())
		.sum()
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn sum_below_100() {
		assert_eq!(even_fibonacci_numbers(100), 44);
	}

	#[test]
	fn sum_below_1000000() {
		assert_eq!(even_fibonacci_numbers(4_000_000), 4613732);
	}

}