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
    match n&1 {
        0 => n/2,
        _ => 3*n+1
    }
}

fn collatz_inner(n: usize, sequence: Vec<usize>) -> Vec<usize> {
    let mut s = sequence;
    if n == 1 {
        s.push(n);
        return s;
    }
    let new_n = get_next_collatz_number(n);
    s.push(n);
    collatz_inner(new_n, s)
}

#[no_mangle]
pub extern fn largest_collatz_sequence(limit: usize) -> usize { 
    (1..limit).map(|x| collatz_inner(x, vec![]))
        .max_by_key(|x| x.len())
        .map_or(0, |v| v[0])
}