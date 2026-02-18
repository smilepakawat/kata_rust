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
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_true() {
        assert_eq!(is_palindrome(121), true);
    }
}
