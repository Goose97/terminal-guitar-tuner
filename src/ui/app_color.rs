use crossterm::style::available_color_count;
use ratatui::style::Color;
use std::env;

lazy_static! {
    pub static ref BORDER: Color =
        color(Color::Rgb(242, 232, 207), Color::Indexed(255), Color::White);
    pub static ref TEXT_DARK: Color =
        color(Color::Rgb(0, 18, 25), Color::Indexed(233), Color::Black);
    pub static ref TEXT_LIGHT: Color =
        color(Color::Rgb(242, 232, 207), Color::Indexed(255), Color::White);
    pub static ref BACKGROUND_DARK: Color =
        color(Color::Rgb(0, 18, 25), Color::Indexed(233), Color::Black);
    pub static ref BACKGROUND_LIGHT: Color =
        color(Color::Rgb(242, 232, 207), Color::Indexed(255), Color::White);
    pub static ref GREEN: Color = color(Color::Rgb(56, 176, 0), Color::Indexed(34), Color::Green);
    pub static ref RED: Color = color(Color::Rgb(193, 18, 30), Color::Indexed(160), Color::Red);
    pub static ref BLUE: Color = color(Color::Rgb(72, 202, 228), Color::Indexed(27), Color::Blue);
}

fn color(truecolor: Color, color256: Color, fallback: Color) -> Color {
    if truecolor_support() {
        truecolor
    } else if color256_support() {
        color256
    } else {
        fallback
    }
}

fn truecolor_support() -> bool {
    let colorterm = env::var("COLORTERM").unwrap_or_default();

    colorterm == "truecolor" || colorterm == "24bit"
}

fn color256_support() -> bool {
    available_color_count() == 256
}
