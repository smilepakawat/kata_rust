#![allow(warnings)]
fn find_prime(n: u16) -> u64 {
    let mut count: u64 = 0;
    let mut prime_count: u16 = 0;
    let mut res: u64 = 0;
    loop {
        if is_prime(count) {
            prime_count += 1
        }
        if prime_count == n {
            res = count;
            break;
        }
        count += 1;
    }
    return res;
}

// copy from mod largest_prime_factor
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
    fn test_find_prime() {
        assert_eq!(13, find_prime(6));
        assert_eq!(7919, find_prime(1000));
        assert_eq!(104743, find_prime(10001));
    }
}
