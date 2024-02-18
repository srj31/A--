pub enum BasicColor {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

pub enum Color {
    Basic(BasicColor),
    Bold(BasicColor),
    RGB(u8, u8, u8),
    Gray(u8),
}

pub fn text_message_with_color(color: Color, text: &str) -> String {
    text_with_color(color_to_int(color), text)
}

fn color_to_int(color: Color) -> u8 {
    match color {
        Color::Basic(c) => c as u8,
        Color::Bold(c) => 8 + c as u8,
        Color::RGB(r, g, b) => 16 + b + g * 6 + r * 36,
        Color::Gray(g) => 232 + g,
    }
}

fn text_with_color(number: u8, text: &str) -> String {
    format!("\x1b[38;5;{}m{}\x1b[0m", number, text)
}
