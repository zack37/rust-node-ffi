fn fact(n: usize, acc: usize) -> usize {
	match n {
		0|1 => acc,
		_ => fact(n-1, acc * n)
	}
}

#[no_mangle]
pub extern fn digit_factorials() -> usize {
	(3..1_000_000_0)
		.filter_map(|x| {
			let clump: usize = format!("{}", x).as_str()
				.split("")
				.filter_map(|x| x.parse::<usize>().ok())
				.map(|x| fact(x, 1))
				.sum();
			if clump == x {
				return Some(x);
			}
			None
		})
		.sum()
}