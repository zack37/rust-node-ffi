use chrono::*;

use project_euler::common::date_generator::generate_dates;

#[no_mangle]
pub extern fn counting_sundays() -> usize {
	let start_date = UTC.ymd(1901, 1, 1);

	generate_dates(start_date)
		.take_while(|&d| !(d.day() == 31 && d.month() == 12 && d.year() == 2000))
		.filter(|d| d.weekday() == Weekday::Sun && d.day() == 1)
		.count()
	// counting_inner(start_date, 0)
}