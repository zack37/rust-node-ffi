use num::pow;

#[no_mangle]
pub extern fn digit_fifth_powers(exp: usize) -> usize {
	(2..pow(9, exp)*exp)
		.filter_map(|i| {
			let clump: usize = format!("{}", i).as_str()
				.split("")
				.map(|x| pow(x.parse::<usize>().unwrap_or(0), exp))
				.sum();
			if clump == i {
				return Some(clump);
			}
			None
		})
		.sum()
}