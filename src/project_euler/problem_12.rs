use project_euler::common::triangle_numbers::generate_triangle_numbers;

pub fn factors_count(target: usize) -> usize {
	let mut count = 2;
	let sqrt = ((target+1) as f64).sqrt() as usize;
	for i in 2..sqrt {
		if target % i == 0 {
			count += 2;
		}
	}
	count
}

#[no_mangle]
pub extern fn highly_divisible_triangular_number(divisor_limit: usize) -> usize {
	generate_triangle_numbers()
		.find(|&x| factors_count(x) > divisor_limit)
		.unwrap_or(0)
}