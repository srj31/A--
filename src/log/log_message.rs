use crate::log::color_helper::{text_message_with_color, BasicColor, Color};

pub fn print_error_msg(msg: &str) {
    let error_msg = format!(
        "{}: {}",
        text_message_with_color(Color::Bold(BasicColor::Red), "Error"),
        text_message_with_color(Color::Basic(BasicColor::Red), msg)
    );
    println!("{}", error_msg);
}
