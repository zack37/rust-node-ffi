#![feature(test)]

extern crate chrono;
extern crate num;

pub mod project_euler;
pub mod point_in_polygon;

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