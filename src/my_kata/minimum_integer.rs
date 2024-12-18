#[cfg(test)]
fn min_num(arr: &[i32]) -> i32 {
    *arr.iter().reduce(|acc, e| acc.min(e)).unwrap()
}

#[cfg(test)]
mod test {
    use super::min_num;

    #[test]
    fn test_min_num() {
        assert_eq!(min_num(&[1, -99, 2, 3]), -99);
    }
}
