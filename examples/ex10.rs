
use myrust::is_palindrome; 

enum Isit {
    Is,
    Isnot,
}

impl Isit {
    fn check_palindrome(input: &str) -> Isit {
        if is_palindrome(input) { Isit::Is } else { Isit::Isnot }
    }

    fn as_str(&self) -> &str {
        match self { Isit::Is => "é palíndromo", Isit::Isnot => "não é palíndromo" }
    }
}

fn main() {
    let inputs = ["A man a plan a canal Panama", "hello uwu"];

    for &input in &inputs {
        println!("{} -> {}", input, Isit::check_palindrome(input).as_str());
    }
}
