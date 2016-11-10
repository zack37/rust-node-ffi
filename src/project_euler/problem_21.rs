fn proper_divisors(target: usize) -> Vec<usize> {
 	let mut f = vec![1];
	let sqrt = (target as f64).sqrt() as usize;
	for i in 2..sqrt {
		if target % i == 0 {
			f.push(i);
			f.push(target/i);
		}
	}
	f.sort();
	f
}

#[no_mangle]
pub extern fn amicable_numbers(limit: usize) -> usize {
	(1..limit+1).flat_map(|a| -> Vec<(usize, usize)> {
		(a..limit+1).map(|b| {
			(proper_divisors(a).iter().sum(), proper_divisors(b).iter().sum())
		}).collect()
	})
	.filter_map(|(a, b)| -> Option<usize> {
		if a == b{ Some(a + b) } else { None }
	})
	.sum()
}