use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, StatefulWidget, Widget};

use super::app_color;
use super::utils;

#[derive(Clone, Debug)]
pub struct TuningPegs();

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub focus_peg: Option<usize>,
}

impl StatefulWidget for TuningPegs {
    type State = State;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let text = vec![
            Line::from("  ┌────┐ "),
            Line::from(vec![peg(state, 3), Span::from(" ││││││ "), peg(state, 2)]),
            Line::from("  ││││││  "),
            Line::from(vec![peg(state, 4), Span::from(" ││││││ "), peg(state, 1)]),
            Line::from("  ││││││  "),
            Line::from(vec![peg(state, 5), Span::from(" ││││││ "), peg(state, 0)]),
            Line::from("  └────┘  "),
            Line::from("   │  │   "),
            Line::from("   ├──┤   "),
            Line::from("   │  │   "),
            Line::from("   ├──┤   "),
            Line::from("   │  │   "),
        ];

        let mut rect = Rect {
            width: 10,
            height: text.len() as u16,
            x: 0,
            y: 0,
        };

        utils::center_rect_in_container(&mut rect, &area);
        let paragraph = Paragraph::new(text).style(Style::default().fg(*app_color::TEXT_LIGHT));
        paragraph.render(rect, buf);
    }
}

fn peg(state: &State, index: usize) -> Span {
    match state.focus_peg {
        Some(x) if x == index => Span::styled("⬤", Style::default().fg(*app_color::TEXT_LIGHT)),
        _ => Span::from("◯"),
    }
}

impl TuningPegs {
    pub fn new() -> Self {
        Self {}
    }
}
