use std::process::{Command, Stdio};
use std::io::{self, Write};
use std::{thread::sleep, time::Duration};

fn hello(){
    let font_path = "mono9.tlf";

    let output = Command::new("figlet")
        .arg("-f")
        .arg(font_path)
        .arg("-w")
        .arg("200") // Define a largura para 200 caracteres
        .arg("nuniLab") // Texto que queremos exibir
        .stdout(Stdio::piped()) // Captura a saída para redirecionar ao lolcat
        .output()
        .expect("Falha ao executar o comando figlet");

    // Converte a saída do figlet para string
    let figlet_output = String::from_utf8_lossy(&output.stdout);

    // Passa a saída do figlet como entrada para o lolcat
    let mut lolcat = Command::new("lolcat")
        .stdin(Stdio::piped()) // Prepara o lolcat para receber stdin do figlet
        .spawn()
        .expect("Falha ao iniciar o comando lolcat");

    // Escreve a saída do figlet no stdin do lolcat
    lolcat.stdin.as_mut()
        .expect("Erro ao acessar stdin do lolcat")
        .write_all(figlet_output.as_bytes())
        .expect("Erro ao escrever no stdin do lolcat");

    // Espera o lolcat processar e finalizar
    lolcat.wait().expect("Erro: o lolcat não conseguiu processar a saída");

    println!("\x1b[38;5;206m-- 🐈💖 --\x1b[0m"); 
    println!("\x1b[38;5;135mHello uwu!\x1b[0m"); 
    println!("\x1b[38;5;227mObrigado por utilizar nossos exemplos!\x1b[0m"); 
    println!("\x1b[38;5;87mtnx ... ><\x1b[0m"); 

}


fn carregar_barras() {
    let total_passos = 20; // Número de passos para completar a barra
    let intervalo = Duration::from_millis(200); // Intervalo entre cada atualização
    let cores = [
        ("\x1b[34m", "Carregando pacotes"),
        ("\x1b[31m", "Otimizando ambiente"),
        ("\x1b[33m", "Inicializando módulos"),
        ("\x1b[36m", "Criando canais de comunicação"),
        ("\x1b[32m", "Definindo permissões de módulos"),
    ];

    for passo in 0..=total_passos {
        let progresso = (passo * 100) / total_passos;

        // Limpa o terminal e posiciona no topo
        print!("\r\x1b[2J\x1b[H");
        
        for (cor, texto) in &cores {
            println!("{}[{:=>20}] {}%\x1b[0m\t::{}", cor, "=".repeat(passo), progresso, texto);
        }

        // Atualiza o terminal e aguarda o próximo passo
        io::stdout().flush().unwrap();
        sleep(intervalo);
    }

    println!("\nCarregamento completo!");
}

fn main() {
    carregar_barras();
    hello();
}
