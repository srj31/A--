use crate::log::color_helper::{text_message_with_color, BasicColor, Color};

pub fn print_error_msg(msg: &str) {
    let error_msg = format!("Error: {}", msg);
    println!(
        "{}",
        text_message_with_color(Color::Basic(BasicColor::Red), &error_msg)
    );
}
