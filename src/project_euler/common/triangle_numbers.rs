#[derive(Debug)]
pub struct TriangleNumber {
	i: usize
}

impl Iterator for TriangleNumber {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		let cur = (1..self.i).sum();
		self.i += 1;

		Some(cur)
	}
}

pub fn generate_triangle_numbers() -> TriangleNumber {
	TriangleNumber { i: 1 }
}