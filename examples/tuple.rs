fn main() {
    let pessoa: (&str, i32, f64) = ("Alice", 30, 1.65);
    
    println!("{:?}", pessoa);

    println!("Nome: {}", pessoa.0);
    println!("Idade: {}", pessoa.1);
    println!("Altura: {}", pessoa.2);

}