#[cfg(test)]
fn find_odd(arr: &[i32]) -> i32 {
    let mut result = 0;

    for &num in arr.iter() {
        result ^= num;
    }

    result
}

#[cfg(test)]
mod test {
    use super::find_odd;

    #[test]
    fn test_find_odd() {
        assert_eq!(
            find_odd(&[20, 1, -1, 2, -2, 3, 3, 5, 5, 1, 2, 4, 20, 4, -1, -2, 5]),
            5
        );
        assert_eq!(find_odd(&[1, 1, 2, -2, 5, 2, 4, 4, -1, -2, 5]), -1);
        assert_eq!(find_odd(&[10]), 10);
        assert_eq!(find_odd(&[1, 1, 1, 1, 1, 1, 10, 1, 1, 1, 1]), 10);
    }
}
