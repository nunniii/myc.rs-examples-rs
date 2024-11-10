
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
    Blue
}

// Analyze the color code (RgBCode) and indicate the color (Color) name
fn analyze_color(code: &RgbCode) -> Color {
    if code.red > code.green && code.red > code.blue {
        return Color::Red;
    } else if code.green > code.red && code.green > code.blue {
        return Color::Green;
    } else {
        return Color::Blue;
    }
}

fn handle_color_type(color: Color) -> String {
    match color {
        Color::Red => String::from("red"),
        Color::Green => String::from("green"),
        Color::Blue => String::from("blue"),
    }
}


fn main() {
    
    let with_more_red: RgbCode = RgbCode { red: 255, green: 28, blue: 142 }; 
    let with_more_green: RgbCode = RgbCode { red: 0, green: 255, blue: 0 };
    let with_more_blue: RgbCode = RgbCode { red: 0, green: 21, blue: 255 };


    println!("🩸 with_more_red --> {:?} --> probaly, this is {}.", &with_more_red, handle_color_type(analyze_color(&with_more_red)));
    println!("🌿 with_more_green --> {:?} --> probaly, this is {}.", &with_more_green, handle_color_type(analyze_color(&with_more_green)));
    println!("🌌 with_more_blue --> {:?} --> probaly, this is {}.", &with_more_blue, handle_color_type(analyze_color(&with_more_blue)));




}

