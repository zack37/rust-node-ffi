pub fn generate_plot(n: usize, precision: f64) -> Vec<(f64, f64)> {
    fn round(n: f64) -> f64 {
        let result = (n*10.0).round() / 10.0;
        result
    }

    let each_length: f64 = n as f64/4.0;

    let mut bottom = (0..each_length as usize).map(|x| (round(x as f64/precision), 0.0)).collect::<Vec<(f64, f64)>>();
    let mut right = (0..each_length as usize).map(|x| (round((each_length-1.0)/precision), round(x as f64/precision))).collect::<Vec<(f64, f64)>>();
    let mut top = (0..each_length as usize).map(|x| (round((each_length-1.0)/precision - x as f64/precision), round((each_length-1.0)/precision))).collect::<Vec<(f64, f64)>>();
    let mut left = (0..each_length as usize).map(|x| (0.0, round((each_length-1.0/precision - x as f64/precision)))).collect::<Vec<(f64, f64)>>();

    bottom.append(&mut right);
    bottom.append(&mut top);
    bottom.append(&mut left);

    bottom
}

#[no_mangle]
pub extern fn is_point_in_path (x: f64, y: f64, poly: &[(f64, f64)]) -> bool {
    let num = poly.len();
    let mut j = num - 1;
    let mut c = false;

    for i in 0..num {
        if ((poly[i].1 > y) != (poly[j].1 > y)) && (x < (poly[j].0 - poly[i].0) * (y - poly[i].1) / (poly[j].1 + poly[i].0) + poly[i].0) {
            c = !c;
        }
        j = i;
    }
    c
}

#[no_mangle]
pub extern fn are_any_points_in_path(number_of_events: usize, number_of_polygon_points: usize, precision: usize) -> bool {
    let shape = generate_plot(number_of_polygon_points, precision as f64);
    let boundary: &[(f64, f64)] = &[(-1.0, -1.0), (-11.0, -1.0), (-11.0, -11.0), (-1.0, -11.0), (-1.0, -1.0)];

    (1..number_of_events+1).filter(|_| {
        shape.iter().any(|&(x, y)| is_point_in_path(x, y, boundary))
    }).count() > 0
}

#[cfg(test)]
mod tests {
	 extern crate test;

	use super::*;
	use self::test::Bencher;

	#[test]
	fn should_return_true_for_single_point_well_inside_boundary() {
	    let boundary: &[(f64, f64)] = &[(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0), (0.0, 0.0)];
		assert!(is_point_in_path(5.0, 5.0, boundary));
	}

	#[test]
	fn should_return_true_for_single_point_on_edge_of_boundary() {
	    let boundary: &[(f64, f64)] = &[(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0), (0.0, 0.0)];
		assert!(is_point_in_path(0.0, 0.0, boundary));
	}

	#[test]
	fn should_return_false_for_single_point_outside_boundary() {
	    let boundary: &[(f64, f64)] = &[(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0), (0.0, 0.0)];
		assert_eq!(is_point_in_path(-5.0, -5.0, boundary), false);
	}

	#[test]
	fn should_return_true_for_shape_well_inside_boundary() {
	    let shape = generate_plot(20, 1.0);
	    let boundary: &[(f64, f64)] = &[(0.0, 0.0), (20.0, 0.0), (20.0, 20.0), (0.0, 20.0), (0.0, 0.0)];
	    assert!(shape.iter().any(|&(x, y)| is_point_in_path(x, y, boundary)))
	}

	#[test]
	fn should_return_true_for_shape_on_edge_of_boundary() {
	    let shape = generate_plot(20, 1.0);
	    let boundary: &[(f64, f64)] = &[(0.0, 0.0), (5.0, 0.0), (5.0, 5.0), (0.0, 5.0), (0.0, 0.0)];
	    assert!(shape.iter().any(|&(x, y)| is_point_in_path(x, y, boundary)))
	}

	#[test]
	fn should_return_false_for_shape_outside_boundary() {
	    let shape = generate_plot(20, 1.0);
	    let boundary: &[(f64, f64)] = &[(-1.0, -1.0), (-11.0, -1.0), (-11.0, -11.0), (-1.0, -11.0), (-1.0, -1.0)];
	    assert_eq!(shape.iter().any(|&(x, y)| is_point_in_path(x, y, boundary)), false)
	}

	#[bench]
	fn benchmark_point_in_polygon_1_event(b: &mut Bencher) {
	    let shape = generate_plot(50000, 10.0);
	    let boundary: &[(f64, f64)] = &[(-1.0, -1.0), (-11.0, -1.0), (-11.0, -11.0), (-1.0, -11.0), (-1.0, -1.0)];
		b.iter(|| {
			shape.iter().any(|&(x, y)| is_point_in_path(x, y, boundary))
		});
	}

	#[bench]
	fn benchmark_point_in_polygon_10_event(b: &mut Bencher) {
	    let shape = generate_plot(50000, 10.0);
	    let boundary: &[(f64, f64)] = &[(-1.0, -1.0), (-11.0, -1.0), (-11.0, -11.0), (-1.0, -11.0), (-1.0, -1.0)];
		b.iter(|| {
			(1..11).filter(|_| shape.iter().any(|&(x, y)| is_point_in_path(x, y, boundary))).count() > 0
		});
	}

	#[bench]
	fn benchmark_point_in_polygon_100_events(b: &mut Bencher) {
	    let shape = generate_plot(50000, 10.0);
	    let boundary: &[(f64, f64)] = &[(-1.0, -1.0), (-11.0, -1.0), (-11.0, -11.0), (-1.0, -11.0), (-1.0, -1.0)];
		b.iter(|| {
			(1..101).filter(|_| shape.iter().any(|&(x, y)| is_point_in_path(x, y, boundary))).count() > 0
		});
	}

	#[bench]
	fn benchmark_point_in_polygon_1000_events(b: &mut Bencher) {
	    let shape = generate_plot(50000, 10.0);
	    let boundary: &[(f64, f64)] = &[(-1.0, -1.0), (-11.0, -1.0), (-11.0, -11.0), (-1.0, -11.0), (-1.0, -1.0)];
	    b.iter(|| {
			(1..1_001).filter(|_| shape.iter().any(|&(x, y)| is_point_in_path(x, y, boundary))).count() > 0
		});
	}

}
