pub fn sum(slice: &[u8]) -> i32 {
    let mut total = 0;
    for &value in slice {
        total += value as i32;
    }
    total
}