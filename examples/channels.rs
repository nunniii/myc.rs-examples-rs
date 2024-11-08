use std::{thread, time};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::io::{self, Write};
use std::process;

fn display_progress_bar() {
    // Função para exibir barra de progresso
    let total = 20; // Tamanho da barra de progresso
    for i in 0..=total {
        print!("\r\t[");
        for _ in 0..i {
            print!("#");
        }
        for _ in i..total {
            print!(" ");
        }
        print!("] {}/{}", i, total);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(100)); // Atualiza o progresso a cada 100ms
    }
    println!();
}

fn to_quit() {
    println!("Saindo...");
    println!("tnx ><");
    process::exit(0);
}

fn main() {
    println!("Hello uwu \n");

    // Definindo canais de comunicação entre threads
    let (tx1, rx1) = mpsc::channel(); // Canal para comunicação de Thread 1 para a principal
    let (tx2, rx2) = mpsc::channel(); // Canal para comunicação de Thread 2 para a principal

    // A variável compartilhada entre as threads
    let shared_var = Arc::new(Mutex::new(1)); // Começa com o valor 1 (pode ser qualquer número inicial)

    // Thread 1 envia dados para a thread 2
    let handle1 = {
        let shared_var = Arc::clone(&shared_var);
        thread::spawn(move || {
            loop {
                let mut num = shared_var.lock().unwrap();
                *num += 2; // Soma 2 à variável
                tx2.send(*num).unwrap(); // Envia para a thread 2
                thread::sleep(time::Duration::from_secs(1)); // Aguarda 1 segundo antes de enviar novamente
            }
        })
    };

    // Thread 2 envia dados para a thread 1
    let handle2 = {
        let shared_var = Arc::clone(&shared_var);
        thread::spawn(move || {
            loop {
                let mut num = shared_var.lock().unwrap();
                *num *= 2; // Multiplica a variável por 2
                tx1.send(*num).unwrap(); // Envia para a thread 1
                thread::sleep(time::Duration::from_secs(1)); // Aguarda 1 segundo antes de enviar novamente
            }
        })
    };

    let mut exit = false;
    while !exit {
        // Exibe a barra de progresso para a troca de dados
        display_progress_bar();

        // Recebe e envia as mensagens alternadamente sem armazená-las
        rx1.recv().unwrap(); // Recebe da Thread 1
        rx2.recv().unwrap(); // Recebe da Thread 2

        // Exibe a barra de progresso para a troca de dados
        display_progress_bar();

        // Imprime o valor da variável compartilhada após modificações das threads
        let num = shared_var.lock().unwrap(); // Destrava o Mutex e acessa o valor
        println!("Valor atual de shared_var após troca: {}", *num);

        // Pergunta ao usuário se deseja continuar ou sair
        println!("\nPressione 'q' para sair ou qualquer tecla para ordenar a troca 🔃 de dados...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "q" {
            exit = true;
            to_quit();
        }
    }

    // Esperando as threads terminarem
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Programa encerrado.");
}
