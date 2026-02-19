/// Determine whether an integer is a palindrome. An integer is a palindrome when it reads the same backward as forward.
///
/// # Arguments
///
/// * `num` - A signed integers.
///
/// # Returns
///
/// `ture' if palidrome, otherwise `false`.
///
/// # Examples
///
/// ```
/// use kata_02::is_palindrome;
///
/// assert_eq!(is_palindrome(121), true);
/// assert_eq!(is_palindrome(-121), false);
/// assert_eq!(is_palindrome(10), false);
/// ```
pub fn is_palindrome(num: i32) -> bool {
    num.to_string() == num.to_string().chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_true_for_palindrome_number() {
        assert!(is_palindrome(121));
    }

    #[test]
    fn should_return_false_for_negative_number() {
        assert!(!is_palindrome(-121));
    }

    #[test]
    fn should_return_false_for_non_palindrome_number() {
        assert!(!is_palindrome(10));
    }

    #[test]
    fn should_return_true_for_single_digit() {
        assert!(is_palindrome(7));
    }

    #[test]
    fn should_return_true_for_zero() {
        assert!(is_palindrome(0));
    }

    #[test]
    fn should_return_true_for_large_palindrome() {
        assert!(is_palindrome(1234321));
    }
}
