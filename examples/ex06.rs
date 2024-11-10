use rand::Rng;


fn roll_dice() -> u8 {
    // Isso criará um gerador de números aleatórios
    let mut rng = rand::thread_rng();

    // Retornar o número aleatório entre 1 e 6
    rng.gen_range(1..=6)
    // Outra alternativa seria:
    // rng.gen_range(1..7)
}


fn simulate_rolls(n: u32) -> Vec<u8> {

    (0..n)
        // aplica roll_dice() para cada item do intervalo,
        // ignorando o valor do índice _ (não precisamos dele)
        .map(|_| roll_dice())
         // coleta todos os resultados de roll_dice() em um vetor
        .collect()


}

fn main() {
    let n_rolls = 10;
    let rolls = simulate_rolls(n_rolls);

    println!("🎲 --> {:?}", rolls);
}
