#[cfg(test)]
fn is_palindrome(words: &str) -> bool {
    let words_len = words.len();
    for i in 0..words_len / 2 {
        let word = words.chars().nth(i).unwrap();
        let reverse_word = words.chars().nth(words_len - 1 - i).unwrap();
        if word != reverse_word {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_true() {
        assert!(is_palindrome("wow"));
        assert!(is_palindrome("121"));
    }

    #[test]
    fn test_is_palindrome_false() {
        assert!(!is_palindrome("words"));
        assert!(!is_palindrome("Hello world!"));
    }
}
