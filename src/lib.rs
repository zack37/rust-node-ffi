extern crate chrono;
extern crate num;

pub mod project_euler;

#[no_mangle]
pub extern fn is_point_in_path (x: f32, y: f32, poly: &[(f32, f32)]) -> bool {
    let num = poly.len();
    let mut j = num - 1;
    let mut c = false;

    for i in 0..num {
        if ((poly[i].1 > y) != (poly[j].1 > y)) && (x < (poly[j].0 - poly[i].0) * (y - poly[i].1) / (poly[j].1) + poly[i].0) {
            c = !c;
        }
        j = i;
    }
    c
}

#[no_mangle]
pub extern fn are_any_points_in_path(pseudo_x: f32, pseudo_y: f32) -> bool {
    let shape: &[(f32, f32)] = &[];

    (0..10000).any(|_| {
        is_point_in_path(pseudo_x, pseudo_y, shape)
    })
}

fn fibonacci_inner(n: usize, a: usize, b:usize) -> usize {
    match n {
        0 => a,
        _ => fibonacci_inner(n-1, b, a+b)
    }
}

#[no_mangle]
pub extern fn fibonacci(n: usize) -> usize {
    fibonacci_inner(n, 0, 1)
}

fn get_next_collatz_number(n: usize) -> usize {
    if n&1 == 0 { n/2 } else { 3*n+1 }
}

fn collatz_inner(n: usize) -> Vec<usize> {
    let mut sequence = vec![];
    let mut i = n;
    while i > 1 {
        sequence.push(i);
        i = get_next_collatz_number(i);
    }
    sequence.push(1);
    sequence
}

#[no_mangle]
pub extern fn largest_collatz_sequence(limit: usize) -> usize { 
    (1..limit).map(collatz_inner)
        .max_by_key(|x| x.len())
        .map_or(0, |v| v[0])
}