// By considering the terms in the Fibonacci sequence whose values do not exceed four million,
// find the sum of the even-valued terms.
#![allow(warnings)]
fn even_fibonacci_numbers() -> u64 {
    let mut sum: u64 = 0;
    let mut count: u64 = 0;
    loop {
        count += 1;
        let fibo: u64 = fibonacci(count);
        if fibo > 4000000 {
            break;
        }
        if fibo % 2 == 0 {
            sum += fibo
        }
    }
    return sum
}

fn fibonacci(i: u64) -> u64 {
    if i == 0 || i == 1 {
        return i
    } else {
        return fibonacci(i-1) + fibonacci(i-2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibbonacci() {
        assert_eq!(1, fibonacci(2));
        assert_eq!(55, fibonacci(10));
    }
    
    #[test]
    fn test_even_fibonacci_numbers() {
        assert_eq!(4613732, even_fibonacci_numbers());
    }
}
