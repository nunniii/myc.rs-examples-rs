use std::io::{self, Write};

fn printAll(emojis: &Vec<char>, water_position: usize) {
    for (i, emoji) in emojis.iter().enumerate() {
        println!(" -- {} -- ", i + 1); // Para mostrar a posição de cada emoji
        
        if i == water_position {
            // Se a posição for igual à escolhida, coloca o emoji da água
            print!("\t💦");
        } else {
            // Senão, imprime o emoji normal
            print!("\t{}", emoji);
        } 
        println!();
    }
}

fn main() {
    let emojis: Vec<char> = vec!['🤑', '💚', '💖'];

    let mut permission = true;
    while permission {
        println!("Escolha onde quer posicionar o emoji '💦' (1, 2 ou 3), ou 'q' para sair:");

        // Captura a entrada do usuário
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Falha ao ler linha");

        // Removendo espaços em branco ao redor da entrada
        let input = input.trim();

        // Se o usuário pressionar 'q', o programa sai
        if input == "q" {
            println!("Saindo...");
            permission = false;
            continue;
        }

        // Converte a entrada para um número inteiro
        let choice: usize = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida, por favor insira 1, 2 ou 3.");
                continue;
            }
        };

        // Verifica se o número inserido está dentro do range de posições válidas
        if choice >= 1 && choice <= 3 {
            printAll(&emojis, choice - 1);
        } else {
            println!("Oops! Por favor, insira uma entrada válida (1, 2 ou 3).");
        }
    }
}
