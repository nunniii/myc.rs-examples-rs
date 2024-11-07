// Exemplo de propriedade e empréstimo em Rust

fn main() {
    // Propriedade
    let s1 = String::from("Olá");  // s1 é o proprietário da String
    let s2 = &s1;  // s2 é uma referência imutável a s1

    println!("s1: {}, s2: {}", s1, s2);  // Funciona porque s2 é uma referência

    // Empréstimo Mutável
    let mut s3 = String::from("Mundo");
    let s4 = &mut s3;  // s4 é uma referência mutável a s3
    s4.push_str(" Rust!");

    println!("s3: {}", s3);  // s3 foi alterado através de s4

    // Propriedade movida (não podemos usar s1 depois que foi movido)
    let s5 = s1;  // Agora s1 não pode ser mais usado
    // println!("{}", s1);  // Erro: s1 não é mais válido após a transferência de propriedade
    println!("{}", s5);  // s5 é o novo proprietário da String
}
