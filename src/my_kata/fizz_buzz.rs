#![allow(warnings)]
fn fizz_buzz(n: u32) -> Vec<String> {
    (1..=n)
        .map(|e| match e {
            e if e % 3 == 0 && e % 5 != 0 => "fizz".to_string(),
            e if e % 3 != 0 && e % 5 == 0 => "buzz".to_string(),
            e if e % 3 == 0 && e % 5 == 0 => "fizzbuzz".to_string(),
            _ => e.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::fizz_buzz;

    #[test]
    fn test_fizz_buzz() {
        assert_eq!(
            fizz_buzz(15),
            [
                "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
                "14", "fizzbuzz"
            ]
        );
    }
}
