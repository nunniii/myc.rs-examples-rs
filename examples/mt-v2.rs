

#[derive(Debug)]
struct RgbCode {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

// Analyze the color code (RgbCode) and indicate the color (Color) name
fn analyze_color(code: &RgbCode) -> Color {
    match (code.red, code.green, code.blue) {
        (r, _, _) if r > code.green && r > code.blue => Color::Red,
        (_, g, _) if code.green > code.red && g > code.blue => Color::Green,
        _ => Color::Blue, // Caso padrão
    }
}

// Convert the color enum to a string
fn handle_color_type(color: Color) -> &'static str {
    match color {
        Color::Red => "red",
        Color::Green => "green",
        Color::Blue => "blue",
    }
}

fn main() {
    let with_more_red = RgbCode { red: 255, green: 28, blue: 142 };
    let with_more_green = RgbCode { red: 0, green: 255, blue: 0 };
    let with_more_blue = RgbCode { red: 0, green: 21, blue: 255 };

    // Usando `analyze_color` diretamente dentro do `handle_color_type`
    println!("🩸 with_more_red --> {:?} --> probaly, this is {}.", &with_more_red, handle_color_type(analyze_color(&with_more_red)));
    println!("🌿 with_more_green --> {:?} --> probaly, this is {}.", &with_more_green, handle_color_type(analyze_color(&with_more_green)));
    println!("🌌 with_more_blue --> {:?} --> probaly, this is {}.", &with_more_blue, handle_color_type(analyze_color(&with_more_blue)));
}
