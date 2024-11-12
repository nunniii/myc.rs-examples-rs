
fn main() {
    let vals = [1, 2, 3, 4, 5]; // Um array com valores de 1 a 5
    let mut iter = vals.iter(); // Criação de um iterador sobre o array
    println!("{:?}", iter.next()); // Imprime o primeiro elemento do iterador
    println!("{:?}", iter.skip(2).take(2).collect::<Vec<_>>()); // Pule 2 elementos, pegue os próximos 2 e colecione em um vetor


    
}