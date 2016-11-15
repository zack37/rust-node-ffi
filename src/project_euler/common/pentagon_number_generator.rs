pub struct PentagonNumber {
	curr: usize
}

impl Iterator for PentagonNumber {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		let cur = self.curr;
		self.curr += 1;

		Some(cur*(3*cur+1)/2)
	}
}

pub fn generate_pentagon_numbers() -> PentagonNumber {
	generate_pentagon_numbers_starting_at(1)
}

pub fn generate_pentagon_numbers_starting_at(start: usize) -> PentagonNumber {
	PentagonNumber{ curr: start }
}