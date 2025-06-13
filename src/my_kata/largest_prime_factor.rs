// What is the largest prime factor of the number
#![allow(warnings)]
pub fn largest_prime_factor(n: u64) -> u64 {
    let mut count: u64 = n / 2;
    let limit = (n as f64).sqrt() as u64;
    for i in (1..=limit).rev() {
        if n % i == 0 && is_prime(i) {
            return i;
        }
    }
    panic!("there's no prime factor")
}

fn is_prime(n: u64) -> bool {
    if (n == 0 || n == 1) {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as u64;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(true, is_prime(2));
        assert_eq!(true, is_prime(3));
        assert_eq!(true, is_prime(5));
    }

    #[test]
    fn test_is_not_prime() {
        assert_eq!(false, is_prime(0));
        assert_eq!(false, is_prime(1));
        assert_eq!(false, is_prime(4));
    }

    #[test]
    fn test_largest_prime_factor() {
        assert_eq!(6857, largest_prime_factor(600851475143));
        assert_eq!(29, largest_prime_factor(13195));
    }
}
