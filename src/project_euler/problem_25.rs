use std::mem::size_of;
use num::PrimInt;
// use project_euler::common::fibonacci_generator::generate_big_fibonacci;

#[no_mangle]
pub extern fn nth_digit_fibonacci_number(n: usize) -> usize {
	println!("{}, {}", size_of::<usize>(), usize::max_value());
	0
	// generate_big_fibonacci()
	// 	.take(10)
	// 	.count()
}
