
// Para `ex10.rs`
pub fn is_palindrome(word: &str) -> bool {
    let cleaned: String = word.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_lowercase();
    
    let reversed = cleaned.chars().rev().collect::<String>();
    cleaned == reversed
}

fn main() {}
