use project_euler::common::collatz_sequence::generate_collatz_sequence;

#[no_mangle]
pub extern fn longest_collatz_sequence() -> usize {
	(1..1_000_000)
		.map(|x| generate_collatz_sequence(x))
		.max_by_key(|x| x.count())
		.map_or(0, |x| x.clone().nth(0).unwrap())
}