#[derive(Clone, Copy, Debug)]
pub struct CollatzSequence {
	last: usize,
	curr: usize
}

impl Iterator for CollatzSequence {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		if self.last == 1  {
			return None;
		}
		self.last = self.curr;
		self.curr = match self.curr {
			n if n%2 == 0 => n/2,
			n => 3*n+1
		};

		Some(self.last)
	}
}

pub fn generate_collatz_sequence(n: usize) -> CollatzSequence {
	CollatzSequence { last: 0, curr: n }
}