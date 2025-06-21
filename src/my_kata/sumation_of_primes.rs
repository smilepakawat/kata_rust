#![allow(warnings)]
fn find_sumary_primes(n: u64) -> u64 {
    let mut sum: u64 = 0;
    let mut count: u64 = 0;
    loop {
        count += 1;
        if count > n {
            break;
        }
        if is_prime(count) {
            sum += count;
        }
    }
    sum
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
    fn test_find_summary_primes() {
        assert_eq!(17, find_sumary_primes(10));
        assert_eq!(142913828922, find_sumary_primes(2_000_000));
    }
}