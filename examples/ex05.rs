// string_examples.rs

use std::collections::HashMap;

// Função para iterar sobre cada caractere de uma string e imprimir
fn print_chars(phrase: &str) {
    println!("Iterando com .chars():");
    for c in phrase.chars() {
        println!("{}", c);
    }
}

// Função para iterar sobre cada byte de uma string e imprimir os códigos ASCII
fn print_bytes(phrase: &str) {
    println!("\nIterando com .bytes(): (Códigos ASCII)");
    for b in phrase.bytes() {
        println!("Byte: {} (ASCII)", b);
    }
}

// Função para dividir uma string por linhas e iterar sobre cada linha
fn print_lines(phrase: &str) {
    println!("\nIterando com .lines():");
    for line in phrase.lines() {
        println!("Linha: {}", line);
    }
}

// Função para dividir uma string por espaços em branco e iterar sobre cada palavra
fn print_words(phrase: &str) {
    println!("\nIterando com .split_whitespace():");
    for word in phrase.split_whitespace() {
        println!("Palavra: {}", word);
    }
}

// Função para mapear cada caractere da string para seu valor ASCII em um HashMap
fn map_ascii_values(phrase: &str) -> HashMap<char, u8> {
    let mut ascii_map = HashMap::new();

    // Itera sobre cada caractere usando .chars() e mapeia seu valor ASCII
    for c in phrase.chars() {
        // Adiciona ao HashMap o caractere com seu valor ASCII (usando .bytes() para obter o código)
        if let Some(ascii_value) = c.to_string().bytes().next() {
            ascii_map.insert(c, ascii_value);
        }
    }

    ascii_map
}

// Função para exibir o conteúdo do HashMap de ASCII
fn print_ascii_map(ascii_map: &HashMap<char, u8>) {
    println!("\nConteúdo do HashMap de ASCII:");
    for (character, ascii_value) in ascii_map {
        println!("'{}' -> ASCII: {}", character, ascii_value);
    }
}

// Função principal para chamar todas as funções de exemplo
fn main() {
    let example_text = "Hello Rust\nWelcome to string manipulation examples!";

    // Imprime os caracteres da string
    print_chars(example_text);

    // Imprime os bytes (códigos ASCII) da string
    print_bytes(example_text);

    // Imprime cada linha da string
    print_lines(example_text);

    // Imprime cada palavra da string
    print_words(example_text);

    // Mapeia os caracteres para valores ASCII e imprime
    let ascii_map = map_ascii_values(example_text);
    print_ascii_map(&ascii_map);
}





