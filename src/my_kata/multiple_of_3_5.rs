#![allow(warnings)]
fn multiple_of_3_5(input: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..input {
        if i % 3 == 0 || i % 5 == 0 {
            sum = sum + i;
        }
    }
    return sum
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn multiple_of_3_5_of_1000() {
        assert_eq!(233168, multiple_of_3_5(1000))
    }
}