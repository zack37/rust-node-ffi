use project_euler::common::pentagon_number_generator::*;

fn is_pentagonal(x: usize) -> bool {
	((24.0*(x as f64)+1.0).sqrt()+1.0)/6.0 % 1.0 == 0.0
}

#[no_mangle]
pub extern fn pentagon_numbers() {
	println!("{}", is_pentagonal(22));
	println!("{}", is_pentagonal(70));
	println!("{}", is_pentagonal(92));
	println!("{}", is_pentagonal(48));
	println!("{}", is_pentagonal(2));
	println!("{}", is_pentagonal(0));


	let result = generate_pentagon_numbers()
		.flat_map(move |j| {
			generate_pentagon_numbers_starting_at(j).take(1000).map(move |k| (j, k))
		})
		// .inspect(|&(j,k)| println!("({},{})", j,k))
		.find(|&(j,k)| is_pentagonal(j+k) && is_pentagonal(k-j));

	println!("{:?}", result);
}