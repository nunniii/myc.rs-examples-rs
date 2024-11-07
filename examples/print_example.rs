use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Número a ser adicionado a 'a' com a flag --print
    #[arg(long, default_value_t = 0)]
    print: u8,
}

fn main() {
    let args = Args::parse();
    
    let a: u8 = 5;
    let printable_a: u8 = a + args.print;

    println!("Hello uwu, a = {}", printable_a);
}
