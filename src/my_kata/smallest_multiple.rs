// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20
#![allow(warnings)]
fn smallest_multiple(start: usize, end: usize) -> usize {
    let mut count = end;
    let mut check = false;
    loop {
        count += 1;
        for i in start..=end {
            if count % i != 0 {
                check = false;
                break;
            } else {
                check = true
            }
        }
        if check {
            return count;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_smallest_multiple() {
        assert_eq!(2520, smallest_multiple(1, 10));
        assert_eq!(232792560, smallest_multiple(1, 20));
    }
}
