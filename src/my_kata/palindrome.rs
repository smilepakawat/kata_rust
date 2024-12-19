#[cfg(test)]
fn is_palindrome(words: &str) -> bool {
    for i in 0..words.len() / 2 {
        if words.chars().nth(i).unwrap() != words.chars().nth(words.len() - i - 1).unwrap() {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_true() {
        assert_eq!(is_palindrome("wow"), true);
        assert_eq!(is_palindrome("121"), true);
    }

    #[test]
    fn test_is_palindrome_false() {
        assert_eq!(is_palindrome("words"), false);
        assert_eq!(is_palindrome("Hello world!"), false);
    }
}
