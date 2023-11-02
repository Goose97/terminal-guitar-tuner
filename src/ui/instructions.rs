use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Widget};

use super::app_color;
use super::utils;

#[derive(Clone, Copy)]
pub struct Instruction {}

impl Instruction {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for Instruction {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = vec![
            Line::styled(
                "Pluck any strings to start tuning",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Line::from(""),
            Line::from("To use alternative tunings:"),
            Line::from("  • j/k: Select strings"),
            Line::from("  • h/l: Select notes"),
            Line::from("  • <esc>: Exit selection"),
            Line::from(""),
            Line::from("q: Quit"),
        ];

        let max_width = text.iter().map(|line| line.width()).max().unwrap();
        let mut rect = Rect {
            width: max_width as u16,
            height: text.len() as u16,
            x: 0,
            y: 0,
        };

        utils::center_rect_in_container(&mut rect, &area);
        let paragraph = Paragraph::new(text).style(Style::default().fg(*app_color::TEXT_LIGHT));
        paragraph.render(rect, buf);
    }
}
