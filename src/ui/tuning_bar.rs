use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Alignment;
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, StatefulWidget, Widget};

use super::{app_color, utils, IN_TUNE_RANGE};
use crate::guitar::{get_note_frequency, semi_tone_down, semi_tone_up, Note};

#[derive(Clone, Debug)]
pub struct TuningBar {}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub min: f64,
    pub max: f64,
    pub center: f64,
    pub accept_range: (f64, f64),
    pub current_pitch: Option<f64>,
    pub pitch_in_accept_range_once: bool,
}

impl StatefulWidget for TuningBar {
    type State = State;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(app_color::BORDER));

        let mut bar_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width - 19,
            height: 3,
        };
        bar_area = utils::max_width(bar_area, 128);

        utils::center_rect_in_container(&mut bar_area, &area);
        block.render(bar_area, buf);

        let sharp_text = Paragraph::new("Sharp");
        sharp_text.render(
            Rect {
                x: bar_area.x + bar_area.width + 1,
                y: bar_area.y + 1,
                width: 5,
                height: 1,
            },
            buf,
        );

        let flat_text = Paragraph::new("Flat");
        flat_text.render(
            Rect {
                x: bar_area.x - 4,
                y: bar_area.y + 1,
                width: 4,
                height: 1,
            },
            buf,
        );

        render_accept_range(&state, &bar_area, buf);
        render_current_pitch(&state, &bar_area, buf);
        render_pitch_difference(&state, &bar_area, buf);
    }
}

fn render_accept_range(state: &State, bar_area: &Rect, buf: &mut Buffer) {
    let bucket_index_min = find_pitch_bucket(state.accept_range.0, state, bar_area);
    let bucket_index_max = find_pitch_bucket(state.accept_range.1, state, bar_area);

    let rect = Rect {
        x: bar_area.x + bucket_index_min,
        y: bar_area.y,
        width: bucket_index_max - bucket_index_min + 1,
        height: bar_area.height,
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(app_color::GREEN));
    block.render(rect, buf);
}

fn render_current_pitch(state: &State, bar_area: &Rect, buf: &mut Buffer) {
    if let Some(pitch) = state.current_pitch {
        // Each character can represent one buckets, so we have in total bar_area.width buckets
        // to put our current pitch tick
        let bucket_index = find_pitch_bucket(pitch, state, bar_area);

        let buf_index = bucket_index;
        let buf_chars = ("█", "▀", "▄");

        let style = Style::default().fg(app_color::RED);
        buf.get_mut(bar_area.x + buf_index, bar_area.y + 1)
            .set_symbol(buf_chars.0)
            .set_style(style);
        buf.get_mut(bar_area.x + buf_index, bar_area.y + 2)
            .set_symbol(buf_chars.1)
            .set_style(style);
        buf.get_mut(bar_area.x + buf_index, bar_area.y)
            .set_symbol(buf_chars.2)
            .set_style(style);
    }
}

// The whole pitch range (min-max) is divided to multiple equal size buckets. The amount of bucket
// is equal to the bar_width
// Given a pitch, find its bucket index
fn find_pitch_bucket(pitch: f64, state: &State, bar_area: &Rect) -> u16 {
    let total_buckets = bar_area.width;
    let bucket_range = (state.max - state.min) / (total_buckets as f64);

    ((pitch - state.min) / bucket_range).floor() as u16
}

fn render_pitch_difference(state: &State, bar_area: &Rect, buf: &mut Buffer) {
    if let Some(pitch) = state.current_pitch {
        let mut rect = Rect {
            x: 0,
            y: 0,
            width: bar_area.width,
            height: 1,
        };

        utils::center_rect_in_container(&mut rect, bar_area);
        rect = utils::transform(rect, 0, -2);

        let diff = pitch_difference(state.center, pitch);
        let mut text = Paragraph::new(diff).alignment(Alignment::Center);

        if state.accept_range.0 <= pitch && state.accept_range.1 >= pitch {
            text = text.style(Style::default().fg(app_color::GREEN));
        }

        text = text.add_modifier(Modifier::BOLD);

        text.render(rect, buf);
    }
}

fn pitch_difference(target: f64, current: f64) -> String {
    let rounded = ((current - target) / 0.5).floor() * 0.5;

    if rounded > 0.0 {
        format!("+{}", rounded)
    } else if rounded < 0.0 {
        rounded.to_string()
    } else {
        String::from("0")
    }
}

impl TuningBar {
    pub fn new() -> Self {
        Self {}
    }
}

impl State {
    pub fn new(note: &Note) -> Self {
        let base_note = get_note_frequency(note);
        let sharp = get_note_frequency(&semi_tone_up(note));
        let flat = get_note_frequency(&semi_tone_down(note));

        let flat_cent = (base_note - flat) / 100.0;
        let sharp_cent = (base_note - flat) / 100.0;

        Self {
            current_pitch: None,
            min: flat,
            max: sharp,
            center: base_note,
            accept_range: (
                base_note - flat_cent * IN_TUNE_RANGE,
                base_note + sharp_cent * IN_TUNE_RANGE,
            ),
            pitch_in_accept_range_once: false,
        }
    }

    pub fn in_tune_range(&self, frequency: f64) -> bool {
        self.accept_range.0 < frequency && self.accept_range.1 > frequency
    }
}
