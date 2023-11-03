use ratatui::buffer::Buffer;
use ratatui::layout::Alignment;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use super::{app_color, utils};

#[derive(Clone, Copy)]
pub struct InsufficientSizeNotice;

impl InsufficientSizeNotice {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for InsufficientSizeNotice {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut rect = Rect {
            x: 0,
            y: 0,
            width: area.width,
            height: 3,
        };

        utils::center_rect_in_container(&mut rect, &area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(*app_color::BORDER));

        let paragraph = Paragraph::new(vec![
            Line::from("Your terminal size is too small"),
            Line::from("Please resize to use the app"),
        ])
        .alignment(Alignment::Center)
        .add_modifier(Modifier::BOLD);

        block.render(area, buf);
        paragraph.render(rect, buf);
    }
}
