use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style, Styled},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, StatefulWidget, Widget},
};
use std::{collections::HashSet, ops::Rem};

use super::app_color;
use super::loading_icon::LoadingIcon;
use crate::guitar::{semi_tone_down, semi_tone_up, Note};

#[derive(Clone, Debug)]
pub struct TuningNotes();

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub notes: Vec<Note>,
    pub tuned_notes: HashSet<Note>,
    pub selected_note_index: Option<usize>,
    pub detecting_note: Option<Note>,
}

impl StatefulWidget for TuningNotes {
    type State = State;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        for (index, tuning_note) in state.notes.iter().enumerate() {
            let mut surround_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain);

            let mut spans = vec![Span::from(tuning_note.to_string())];
            let mut paragraph_style = Style::default();

            if Some(tuning_note) == state.detecting_note.as_ref() {
                surround_block = surround_block
                    .border_style(Style::default().fg(Color::Gray))
                    .border_type(BorderType::Double);

                paragraph_style = paragraph_style
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD);

                spans.extend([Span::from(" "), LoadingIcon::new().into()]);
            }

            if state.tuned_notes.contains(tuning_note) {
                surround_block = surround_block.border_style(Style::default().fg(app_color::GREEN));
                paragraph_style = paragraph_style.fg(app_color::GREEN);
                spans.push(Span::from(" ✓"));
            }

            if Some(index) == state.selected_note_index {
                surround_block = surround_block
                    .border_style(Style::default().bg(app_color::BACKGROUND_LIGHT))
                    .border_type(BorderType::Thick);

                paragraph_style = paragraph_style
                    .fg(app_color::TEXT_DARK)
                    .bg(app_color::BACKGROUND_LIGHT)
                    .add_modifier(Modifier::BOLD);

                spans.insert(0, Span::from("< "));
                spans.push(Span::from(" >"));
            }

            let paragraph = Paragraph::new(vec![Line::from(spans)])
                .block(surround_block)
                .set_style(paragraph_style)
                .alignment(Alignment::Center);

            let item_height = 3;
            let render_area = Rect {
                x: area.x,
                y: area.y + (index as u16) * item_height,
                width: area.width,
                height: item_height,
            };
            paragraph.render(render_area, buf);
        }
    }
}

impl TuningNotes {
    pub fn new() -> Self {
        Self {}
    }
}

impl State {
    pub fn next_note(&mut self) {
        if let Some(index) = self.selected_note_index {
            self.notes[index] = semi_tone_up(&self.notes[index])
        }
    }

    pub fn prev_note(&mut self) {
        if let Some(index) = self.selected_note_index {
            self.notes[index] = semi_tone_down(&self.notes[index])
        }
    }

    pub fn next_string(&mut self) {
        let new_selected_index = match self.selected_note_index {
            Some(current_index) => (current_index + 1).rem(self.notes.len()),
            None => 0,
        };

        self.selected_note_index = Some(new_selected_index);
    }

    pub fn prev_string(&mut self) {
        let tuning_notes = &self.notes;

        let new_selected_index = match self.selected_note_index {
            Some(current_index) => (current_index + tuning_notes.len() - 1).rem(tuning_notes.len()),
            None => tuning_notes.len() - 1,
        };

        self.selected_note_index = Some(new_selected_index);
    }
}
