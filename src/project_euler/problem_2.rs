use project_euler::common::fibonacci_generator::generate;


#[no_mangle]
pub extern fn even_fibonacci_numbers(value_limit: usize) -> usize {
	generate()
		.take_while(|&x| x < value_limit)
		.filter(|x| x % 2 == 0)
		.sum()
}