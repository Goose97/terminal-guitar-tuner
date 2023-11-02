use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols::Marker,
    widgets::{Axis, Chart, Dataset, GraphType, StatefulWidget, Widget},
};

use super::app_color;
use super::utils;

#[derive(Clone, Debug)]
pub struct AudioGraph();

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub dataset: Vec<f64>,
}

impl State {
    pub fn new() -> Self {
        Self { dataset: vec![] }
    }
}

impl StatefulWidget for AudioGraph {
    type State = State;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if state.dataset.is_empty() {
            return;
        }

        let dataset = normalize(&state.dataset);

        let datasets = vec![Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(*app_color::BLUE))
            .data(&dataset)];

        let chart = Chart::new(datasets)
            .x_axis(Axis::default().bounds([0.0, dataset.len() as f64]))
            .y_axis(Axis::default().bounds([0.0, 1.0]))
            .style(Style::default().bg(*app_color::BACKGROUND_DARK));

        let mut rect = Rect {
            width: area.width - 16,
            height: area.height - 4,
            x: 0,
            y: 0,
        };

        utils::center_rect_in_container(&mut rect, &area);
        chart.render(rect, buf);
    }
}

fn normalize(dataset: &[f64]) -> Vec<(f64, f64)> {
    let max = dataset
        .iter()
        .max_by(|m1, m2| f64::total_cmp(&m1, &m2))
        .unwrap()
        .clone();

    dataset
        .iter()
        .enumerate()
        .map(|(index, x)| (index as f64, x / max))
        .collect()
}

impl AudioGraph {
    pub fn new() -> Self {
        Self {}
    }
}
