// Verifica se a frase está vazia
fn is_empty(phrase: &str) -> bool {
    phrase.trim().is_empty()
}

// Conta o número de palavras na frase
fn count_words(phrase: &str) -> usize {
    if is_empty(phrase) {
        return 0;
    }

    // Usa split_whitespace para dividir por espaços em branco
    phrase.split_whitespace().count()
}

fn main() {
    let my_phrase = "Hello uwu";
    println!("Número de palavras: {}", count_words(my_phrase));
}
