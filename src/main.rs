use std::process::{Command, Stdio};
use std::io::{self, Write};

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
}

fn main() {
    hello();
}
