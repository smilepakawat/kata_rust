// Find the largest palindrome made from the product of x-digit numbers.
#![allow(warnings)]
fn largest_palindrome_product_by_digit(digit: u8) -> u32 {
    let mut counter: u32 = (0..digit).map(|d| 9 * u32::pow(10, d as u32)).sum();
    let mut res: u32 = 0;
    for i in (1..=counter).rev() {
        for j in (1..=counter).rev() {
            if res >= i * j {
                break;
            }
            if is_palindrome_number(i * j) {
                res = i * j;
            }
        }
    }
    return res;
}

fn is_palindrome_number(n: u32) -> bool {
    let num_string = &n.to_string();
    let num_len = (&n.to_string()).len();
    for i in 0..num_len / 2 {
        let num_char = num_string.chars().nth(i).unwrap();
        let reverse_num_char = num_string.chars().nth(num_len - 1 - i).unwrap();
        if num_char != reverse_num_char {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_palindrome_3digit() {
        assert_eq!(9009, largest_palindrome_product_by_digit(2));
        assert_eq!(906609, largest_palindrome_product_by_digit(3));
    }

    #[test]
    fn test_is_palindrome_number_should_true() {
        assert_eq!(true, is_palindrome_number(121));
        assert_eq!(true, is_palindrome_number(90009));
        assert_eq!(true, is_palindrome_number(8567658));
    }

    #[test]
    fn test_is_palindrome_number_should_false() {
        assert_eq!(false, is_palindrome_number(12100));
        assert_eq!(false, is_palindrome_number(55334));
        assert_eq!(false, is_palindrome_number(100));
    }
}
