
// Objetivo: Crie uma função em Rust que calcula a média de uma lista de notas (valores inteiros) e retorne o valor como um f32 com duas casas decimais.


fn calculate_average(numbers: Vec<i32>) -> f32 {

    let sum: i32 = numbers.iter().sum();
    let average: f32 = sum as f32 / numbers.len() as f32;

    average
}


fn main() {
    let numbers: Vec<i32> = vec![1, 1, 2, 2];
    println!("{} ", calculate_average(numbers));
}
