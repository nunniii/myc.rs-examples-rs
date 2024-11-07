#[warn(dead_code)]

// enum Tipo {
//     Variante1,
//     Variante2,
//     Variante3,
// }

enum Cor {
    Vermelho,
    Verde,
    Azul,
}


enum Forma {
    Circulo(f64),    // raio
    Retangulo(f64, f64),  // largura e altura
}

fn area(forma: Forma) -> f64 {
    match forma {
        Forma::Circulo(raio) => std::f64::consts::PI * raio * raio,
        Forma::Retangulo(largura, altura) => largura * altura,
    }
}

fn main() {
    let minha_cor: Cor = Cor::Vermelho;

    match minha_cor {
        Cor::Vermelho => println!("A cor é vermelha!"),
        Cor::Verde => println!("A cor é verde!"),
        Cor::Azul => println!("A cor é azul!"),
    }





    let c: Forma = Forma::Circulo(3.0);
    let r: Forma = Forma::Retangulo(4.0, 5.0);

    println!("Área do círculo: {}", area(c));
    println!("Área do retângulo: {}", area(r));

}


