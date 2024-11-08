use std::{process::{Command, Stdio}, io::{self, Write}, thread::sleep, time::Duration};

use crossterm::{
    event::{self, KeyCode, KeyEvent, Event},
    terminal::{self, ClearType, Clear, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    ExecutableCommand,
    cursor::{Hide, Show},
};
use tui::{
    backend::CrosstermBackend,
    widgets::{List, ListItem, Block, Borders, ListState},
    layout::{Layout, Constraint, Direction},
    Terminal,
};


fn clear_screen(handle: &mut dyn std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
    // Função para limpar a tela
    handle.execute(Clear(ClearType::All))?; 
    Ok(())
}

fn run_emoji_selector() -> Result<String, Box<dyn std::error::Error>> {
    let emojis = vec![
        "🐶", "🐱", "🐰", "🦊", "🐻", "🐼", "🦁", "🐮", "🐸", "🐷",
    ];

    let stdout = io::stdout();  
    let mut handle = stdout.lock();  

    let mut selected_index = 0; 

    // Habilita o modo raw e alterna para a tela secundária
    enable_raw_mode()?;
    handle.execute(EnterAlternateScreen)?;  
    handle.execute(Hide)?;  

    // Limpa o terminal antes de começar
    clear_screen(&mut handle)?;

    // Criamos o backend para o terminal
    let backend = CrosstermBackend::new(stdout); 
    let mut terminal = Terminal::new(backend)?;

    let mut state = ListState::default();
    state.select(Some(selected_index));

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)])
                .split(size);

            let items: Vec<ListItem> = emojis.iter().map(|emoji| ListItem::new(*emoji)).collect();
            let list = List::new(items)
                .block(Block::default().title("Escolha um Emoji de Animal Fofinho").borders(Borders::ALL))
                .highlight_symbol(">> ") 
                .highlight_style(tui::style::Style::default().bg(tui::style::Color::Blue).fg(tui::style::Color::White)); 

            f.render_stateful_widget(list, chunks[0], &mut state);
        })?;

        if let Ok(event) = event::read() {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc, .. }) => {
                    println!("\nSeleção cancelada.");
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter, .. }) => {
                    // Limpa a tela antes de exibir o emoji selecionado
                    clear_screen(&mut handle)?;
                    println!("\nVocê selecionou: {}", emojis[selected_index]);
                    disable_raw_mode()?;

                    return Ok(emojis[selected_index].to_string());
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Down, .. }) => {
                    if selected_index < emojis.len() - 1 {
                        selected_index += 1;
                    }
                    state.select(Some(selected_index));
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Up, .. }) => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                    state.select(Some(selected_index));
                }
                _ => {}
            }
        }
    }

    // Sai da tela alternada e desabilita o modo raw
    disable_raw_mode()?;
    handle.execute(Show)?;    
    handle.execute(LeaveAlternateScreen)?; 

    Ok("".to_string())
}


fn hello() {
    let font_path = "mono9.tlf";

    // Executa o comando `figlet` para criar uma arte com o texto
    let output = Command::new("figlet")
        .arg("-f")
        .arg(font_path)
        .arg("-w")
        .arg("200") // Define a largura para 200 caracteres
        .arg("nuniLab") // Texto que queremos exibir
        .stdout(Stdio::piped()) // Captura a saída para redirecionar ao lolcat
        .output()
        .expect("Falha ao executar o comando figlet");

    // Converte a saída do `figlet` para string
    let figlet_output = String::from_utf8_lossy(&output.stdout);

    // Envia a saída do `figlet` para o comando `lolcat`
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

    // Mensagens estilizadas com cores
    println!("\x1b[38;5;206m-- 🐈💖 --\x1b[0m");
    println!("\x1b[38;5;135mHello uwu!\x1b[0m");
    println!("\x1b[38;5;227mObrigado por utilizar nossos exemplos!\x1b[0m");
    println!("\x1b[38;5;87mtnx ... ><\x1b[0m");
}

fn carregar_barras() {
    let total_passos = 20; // Número de passos para completar a barra
    let intervalo = Duration::from_millis(200); // Intervalo entre cada atualização

    // Cores para cada barra
    let barras = [
        ("\x1b[34m", "Carregando pacotes"),
        ("\x1b[31m", "Otimizando ambiente"),
        ("\x1b[33m", "Inicializando módulos"),
        ("\x1b[36m", "Criando canais de comunicação"),
        ("\x1b[32m", "Definindo permissões de módulos"),
    ];

    for passo in 0..=total_passos {
        let progresso = (passo * 100) / total_passos;

        // Limpa o terminal e posiciona o cursor no topo
        print!("\r\x1b[2J\x1b[H");
        println!("\n\n");

        // Exibe cada barra de progresso
        for (cor, texto) in &barras {
            println!("{}[{:=>20}] {}%\x1b[0m\t::{}", cor, "=".repeat(passo), progresso, texto);
        }

        // Atualiza o terminal e aguarda o próximo passo
        io::stdout().flush().unwrap();
        sleep(intervalo);
    }

    println!("\nCarregamento completo!");
}

fn main() {
    
    // Aqui estou criando esta variável para coletar o emoji escolhido 
    let mut emoji_char = String::from("🐱");
    match run_emoji_selector() {
        Ok(emoji) => {
            // Atribuindo o emoji a emoji_char ...
            emoji_char = emoji;
        }
        Err(e) => {
            eprintln!("Erro: {}", e);
        }
    }
    carregar_barras(); 
    print!("\n-- uwu -- ");
    println!("Emoji selecionado: {}", emoji_char);
    hello();
}
