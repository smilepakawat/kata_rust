// The sum of the squares of the first ten natural numbers is,
// 1^2 + 2^2 + 3^2 = ..
// The square of the sum of the first ten natural numbers is,
// (1 + 2 +3)^2 = ..
// Find the difference between the sum of the squares of the first one n natural numbers and the square of the sum.
#![allow(warnings)]
fn sum_suqare_difference(n: u8) -> u64 {
    return square_of_sum(n) - sum_of_square(n);
}

fn square_of_sum(n: u8) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=n {
        sum += i as u64;
    }
    return u64::pow(sum, 2);
}

fn sum_of_square(n: u8) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=n {
        sum += u64::pow(i as u64, 2);
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_suqare_difference() {
        assert_eq!(2640, sum_suqare_difference(10));
        assert_eq!(25164150, sum_suqare_difference(100));
    }

    #[test]
    fn test_square_of_sum() {
        assert_eq!(3025, square_of_sum(10));
        assert_eq!(25502500, square_of_sum(100));
    }

    #[test]
    fn test_sum_of_square() {
        assert_eq!(385, sum_of_square(10));
        assert_eq!(338350, sum_of_square(100));
    }
}
