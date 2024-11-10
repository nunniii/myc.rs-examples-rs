fn count_vowels(phrase: &str) -> Vec<u32> {
    // Definimos um array com as vogais que queremos contar
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    
    // Criamos um vetor dinâmico (Vec) que começa com 0 em todas as posições,
    // uma para cada vogal. Esse vetor irá armazenar a contagem das vogais
    let mut vowel_counts = vec![0; vowels.len()];

    // Percorremos cada caractere da frase
    for ch in phrase.chars() {
        // Convertemos o caractere atual para minúsculo e pegamos o primeiro caractere do resultado.
        // Isso garante que estamos contando 'A' e 'a' como a mesma vogal.
        let ch = ch.to_lowercase().next().unwrap();

        // Aqui, verificamos se o caractere atual é uma vogal:
        // `position` retorna `Some(index)` se `ch` for uma das vogais, e `None` caso contrário.
        if let Some(index) = vowels.iter().position(|&v| v == ch) {
            // Incrementamos a contagem da vogal correspondente no vetor `vowel_counts`
            vowel_counts[index] += 1;
        }
    }

    // Retornamos o vetor com as contagens de cada vogal
    vowel_counts
}

fn main() {
    // Define a frase que queremos analisar
    let phrase = "hello My nAme is mateus";
    
    // Exibe a contagem das vogais
    println!("{:?}", count_vowels(phrase));
}
