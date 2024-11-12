use myrust::is_palindrome;

#[test]
fn test_is_palindrome() {
    assert_eq!(is_palindrome("A man a plan a canal Panama"), true);
    assert_eq!(is_palindrome("Hello"), false);
}
