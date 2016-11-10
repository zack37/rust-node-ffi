#[no_mangle]
pub extern fn multiples_of_three_and_five(limit: usize) -> usize {
    (1..limit)
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .sum()
}
