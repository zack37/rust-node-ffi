use chrono::*;

pub struct DateGenerator {
	date: Date<UTC>
}

impl Iterator for DateGenerator {
	type Item = Date<UTC>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.date.succ_opt() {
			Some(x) => {
				self.date = x;
				Some(x)
			},
			None => None
		}
	}
}

pub fn generate_dates(start_date: Date<UTC>) -> DateGenerator {
	DateGenerator { date: start_date }
}
